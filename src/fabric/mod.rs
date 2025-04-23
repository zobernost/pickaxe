use serde::{Deserialize, Serialize};
use std::fmt;

pub mod maven;
pub mod meta;

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    #[serde(rename = "tag_name")]
    pub version: String,
    pub prerelease: bool,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl Clone for Version {
    fn clone(&self) -> Self {
        Version {
            version: self.version.clone(),
            prerelease: self.prerelease,
        }
    }
}

//https://meta.fabricmc.net/v2/versions/loader/1.21.5/0.16.14
//https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.16.4/
