use serde::{Deserialize, Serialize};
use std::fmt;

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
