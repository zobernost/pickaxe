use crate::fabric;
use crate::java;
use crate::mc;

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use std::fs;

#[serde_as]
#[derive(Debug, Serialize)]
pub struct Server {
    name: String,
    #[serde_as(as = "DisplayFromStr")]
    version: mc::Version,
    #[serde_as(as = "DisplayFromStr")]
    fabric_version: fabric::Version,
    #[serde_as(as = "DisplayFromStr")]
    java_version: java::Version,
}

impl Server {
    pub fn new(
        name: String,
        version: mc::Version,
        fabric_version: fabric::Version,
        java_version: java::Version,
    ) -> Self {
        Self {
            name,
            version,
            fabric_version,
            java_version,
        }
    }

    pub async fn build(&self) -> Result<()> {
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
            .join(&self.java_version.to_string());
        fs::create_dir_all(&java_dir)?;
        java::download(java_dir, &self.java_version).await?;

        let fabric_dir = pickaxe_dir
            .join("fabric")
            .join(&self.fabric_version.to_string());
        fs::create_dir_all(fabric_dir)?;
        //fabric::download(&fabric_dir, &self.fabric_version)?;

        Ok(())
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
