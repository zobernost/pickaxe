use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub version: String,
    pub version_type: String,
    pub major: bool,
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
            version_type: self.version_type.clone(),
            major: self.major,
        }
    }
}
