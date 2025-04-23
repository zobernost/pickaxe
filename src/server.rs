use anyhow::{Context, Result};
use indicatif::ProgressBar;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use std::fs;

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
        java: String,
    ) -> Self {
        Self {
            name,
            version,
            fabric,
            java,
        }
    }

    pub async fn build(&self, http: &reqwest::Client) -> Result<()> {
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

        let java_dir = pickaxe_dir
            .join("java")
            .join(&self.java.to_string());
        fs::create_dir_all(&java_dir)?;

        //java::download(java_dir, &self.java).await?;

        let fabric_dir = pickaxe_dir
            .join("fabric")
            .join(&self.fabric.to_string());
        fs::create_dir_all(fabric_dir)?;

        self::download(self.version, self.fabric, self.installer, fabric_dir, http).await?;

        Ok(())
    }

    fn download(
        version: String,
        fabric: String,
        installer: String,
        output_dir: std::path::PathBuf,
        http: &reqwest::Client,
    ) -> Result<()> {
        let url = format!("{}/v2/versions/loader/{}/{}/{}/server/jar", META_API_URL, version, fabric, installer);
        let response = http.get(url)
            .send()
        let pb: ProgressBar = ProgressBar::new_spinner();
        pb.set_style(indicatif::ProgressStyle::default_spinner());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_message("Downloading Fabric Loader...");
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}

pub fn get_server_count() -> Result<u64> {
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let pickaxe_dir = config_dir.join("pickaxe");
    let servers_dir = pickaxe_dir.join("servers");
    match fs::read_dir(servers_dir.clone()) {
        Ok(dir) => Ok(dir.count() as u64),
        Err(_) => Ok(0),
    }
}
