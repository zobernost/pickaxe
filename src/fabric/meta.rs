use anyhow::Result;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{fs, io::Write, path::PathBuf};

const META_API_URL: &str = "https://meta.fabricmc.net";

#[derive(Debug, Deserialize)]
pub struct Game {
    pub version: String,
    pub stable: bool,
}

impl Game {
    pub async fn get_all(http: &reqwest::Client) -> Result<Vec<Game>> {
        let url = format!("{}/v2/versions/game", META_API_URL);
        let response = http.get(url)
            .send()
            .await?;
        let games = response.json::<Vec<Game>>().await?;
        Ok(games)
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Debug, Deserialize)]
pub struct Bundle {
    loader: Loader,
}

#[derive(Debug, Deserialize)]
pub struct Loader {
    pub version: String,
    pub stable: bool
}

impl Loader {
    pub async fn get_all(game_version: &str,http: &reqwest::Client) -> Result<Vec<Loader>> {
        let url = format!("{}/v2/versions/loader/{}", META_API_URL, game_version);
        let response = http.get(url)
            .send()
            .await.expect("Failed to get response");
        let bundles = response.json::<Vec<Bundle>>().await?;
        let loaders = bundles.into_iter().map(|b| b.loader).collect::<Vec<Loader>>();
        Ok(loaders)
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Installer {
    version: String,
    stable: bool,
}

impl Installer {
    pub async fn get_latest (http: &reqwest::Client) -> Result<Installer> {
        let url = format!("{}/v2/versions/installer", META_API_URL);
        let response = http.get(url)
            .send()
            .await?;
        let installers = response.json::<Vec<Installer>>().await?;
        let latest = installers.into_iter().filter(|i| i.stable == true).collect::<Vec<Installer>>().first().cloned();
        match latest {
            Some(installer) => Ok(installer),
            None => Err(anyhow::anyhow!("Failed to get latest installer"))
        }
    }
}

impl std::fmt::Display for Installer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct Server {
    name: String,
    version: String,
    fabric: String,
    #[serde(skip)]
    installer: String,
    java: String,
}

impl Server {
    pub fn new(
        name: String,
        version: String,
        fabric: String,
        installer: String,
        java: String,
    ) -> Self {
        Self {
            name,
            version,
            fabric,
            installer,
            java,
        }
    }

    pub async fn build(&self, http: &reqwest::Client) -> Result<()> {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(indicatif::ProgressStyle::default_spinner());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message("Creating server folder...");

        let config_dir = dirs::config_dir().expect("Failed to get config directory");
        let pickaxe_dir = config_dir.join("pickaxe");
        let output_dir = pickaxe_dir
            .join("servers")
            .join(&self.name);
        fs::create_dir_all(&output_dir)?;

        let output_file_path =
            output_dir.join("server.toml");
        let output_file_contents = self.to_toml()?;
        fs::write(&output_file_path, output_file_contents)?;

        //TODO: add port number
        //TODO: merge all configs into one pickaxe.toml?
        //TODO: create server.properties
        //TODO: create eula.txt (and add prompt for accepting eula)
        //TODO: run fabric bootstrap with --init-settings and check output

        pb.set_message(format!("Downloading java {}...", &self.java));

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let java_dir = pickaxe_dir
            .join("java")
            .join(&self.java.to_string());
        fs::create_dir_all(&java_dir)?;

        //java::download(java_dir, &self.java).await?;

        pb.set_message(format!("Downloading fabric {}...", &self.fabric));

        let fabric_dir = pickaxe_dir
            .join("fabric")
            .join(&self.fabric.to_string());
        fs::create_dir_all(&fabric_dir)?;

        let file_path = fabric_dir.join(format!("{}+{}.jar", &self.fabric, &self.version));
        let url = format!("{}/v2/versions/loader/{}/{}/{}/server/jar", META_API_URL, &self.version, &self.fabric, &self.installer);
        
        let response = http.get(url)
            .send().await?;

        let mut file = fs::File::create(&file_path)?;
        let content = response.bytes().await?;
        file.write_all(&content)?;

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        pb.set_message(format!("Bootstrapping version {}...", &self.version));
        
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        pb.finish_with_message("Build complete!");
        Ok(())
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}
