use core::fmt;

use serde::Deserialize;

pub const URL: &str = "https://api.modrinth.com";

#[derive(Debug, Deserialize)]
pub enum ProjectType {
    Mod,
    ModPack,
    DataPack,
    ResourcePack,
    Shader,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
