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
pub struct Instance {
    name: String,
    r#type: InstanceType,
    #[serde_as(as = "DisplayFromStr")]
    version: mc::Version,
    #[serde_as(as = "DisplayFromStr")]
    fabric_version: fabric::Version,
    #[serde_as(as = "DisplayFromStr")]
    java_version: java::Version,
}

#[derive(Debug, Serialize)]
pub enum InstanceType {
    Client,
    Server,
}

impl std::fmt::Display for InstanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceType::Client => write!(f, "Client"),
            InstanceType::Server => write!(f, "Server"),
        }
    }
}

impl Instance {
    pub fn new(
        name: String,
        r#type: InstanceType,
        version: mc::Version,
        fabric_version: fabric::Version,
        java_version: java::Version,
    ) -> Self {
        Self {
            name,
            r#type,
            version,
            fabric_version,
            java_version,
        }
    }

    pub async fn build(&self) -> Result<()> {
        let config_dir = dirs::config_dir().expect("Failed to get config directory");
        let pickaxe_dir = config_dir.join("pickaxe");
        let output_dir = pickaxe_dir
            .join(&format!("{}s", self.r#type.to_string().to_lowercase()))
            .join(&self.name);
        fs::create_dir_all(&output_dir)?;

        let output_file_path =
            output_dir.join(format!("{}.toml", self.r#type.to_string().to_lowercase()));
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

pub fn get_instance_count(r#type: InstanceType) -> Result<u32> {
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let pickaxe_dir = config_dir.join("pickaxe");
    let dir = match r#type {
        InstanceType::Client => pickaxe_dir.join("clients"),
        InstanceType::Server => pickaxe_dir.join("servers"),
    };
    match fs::read_dir(dir.clone()) {
        Ok(dir) => Ok(dir.count() as u32),
        Err(_) => Ok(0),
    }
}
