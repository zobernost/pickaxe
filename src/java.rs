use crate::github;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub version: usize,
    pub threshold: String,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.version)
    }
}

impl Clone for Version {
    fn clone(&self) -> Self {
        Version {
            version: self.version,
            threshold: self.threshold.clone(),
        }
    }
}

pub fn supported_java_versions() -> [Version; 2] {
    return [
        Version {
            version: 21,
            threshold: "1.20.5".to_string()
        },
        Version {
            version: 17,
            threshold: "1.17.0".to_string(),
        },
    ];
}

#[derive(Debug, Deserialize)]
struct Release {
    #[serde(rename = "name")]
    version: String,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    #[serde(rename = "browser_download_url")]
    url: String,
}

pub async fn download(java_dir: PathBuf, version: &Version) -> Result<()> {
    let url = format!(
        "{}{}{}{}",
        github::URL,
        "/repos/adoptium/temurin",
        version,
        "-binaries/releases/latest"
    );

    let client = reqwest::Client::builder()
        .user_agent("zobernost.pickaxe")
        .build()?;

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(indicatif::ProgressStyle::default_spinner());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_message("Downloading Java...");

    let res = client.get(&url).send().await?;
    let releases: Release = res.json::<Release>().await?;

    pb.finish_with_message("Done!");

    Ok(())
}
