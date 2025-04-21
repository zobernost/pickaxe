use clap::{builder::styling::Color, ArgGroup, Args, Parser, Subcommand, ValueEnum};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::blocking;
use semver::Version;
use serde::Deserialize;
use std::{ffi::{OsStr, OsString}, fmt, fs, path::PathBuf, thread::spawn};

#[derive(Debug, Parser)]
#[command(name = "pickaxe")]
#[command(about = "A thing for managing Minecraft instances.")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Clone, ValueEnum)]
enum Source {
    #[value(alias = "m")]
    Modrinth,
    #[value(alias = "c")]
    Curseforge,
}

#[derive(Debug, Deserialize)]
struct MCVersion {
    version: String,
    version_type: String,
    major: bool,
}

impl fmt::Display for MCVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Debug, Deserialize)]
struct Loader {
    icon: String,
    name: String,
    supported_project_types: Vec<String>,
}

impl fmt::Display for Loader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize)]
struct FabricVersion {
    #[serde(rename = "tag_name")]
    version: String,
    prerelease: bool,
}

impl fmt::Display for FabricVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

#[derive(Debug, Deserialize)]
struct JavaVersion {
    version: usize,
    threshold: MCVersion,
}

impl fmt::Display for JavaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.version)
    }
}

#[derive(Debug, Deserialize)]
enum ProjectType {
    Mod,
    ModPack,
    DataPack,
    ResourcePack,
    Shader,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(about = "Create a new instance", alias = "n", alias = "i", alias = "init")]
    New {
        #[arg(short, long)]
        server: Option<bool>,
        path: Option<OsString>,
    },
    #[command(about = "Add mods, data packs, or other packages to an existing instance", alias = "a")]
    Add {
        #[arg(short, long, conflicts_with = "source")]
        path: Option<OsString>,
        #[arg(short, long, default_value_t = Source::Modrinth, conflicts_with = "path", value_enum)]
        source: Source,
        package: OsString,
    },
    #[command(about = "Remove packages from an existing instance", alias = "r")]
    Remove {
        package: OsString,
    },
    #[command(about = "Start an instance")]
    Start {
        path: Option<OsString>,
    },
    #[command(about = "Stop an instance")]
    Stop{
        path: Option<OsString>,
    },
}

const MODRINTH_API_URI: &str = "https://api.modrinth.com";
const GITHUB_API_URI: &str = "https://api.github.com";

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::New { server, path } => {
            new(server, path);
        }
        Commands::Add { path, source, package } => {
            add(package);
        }
        Commands::Remove { package } => {
            remove(package);
        }
        Commands::Start { path } => {
            start(path);
        }
        Commands::Stop { path } => {
            stop(path);
        }
    }
}

fn new(server: Option<bool>, os: Option<OsString>) -> Result<()> {
    let supported_loaders: &[&str] = &["fabric", "forge", "vanilla"];

    let java_versions: [JavaVersion; 2] = [
        JavaVersion {
            version: 21,
            threshold: MCVersion {
                version: "1.20.5".to_string(),
                version_type: "release".to_string(),
                major: false,
            }
        },
        JavaVersion {
            version: 17,
            threshold: MCVersion {
                version: "1.17.0".to_string(),
                version_type: "release".to_string(),
                major: true,
            }
        },
    ];

    
    let http_client = reqwest::blocking::Client::builder()
        .user_agent("zobernost.pickaxe")
        .build()?;

    let versions_uri = format!("{}{}", MODRINTH_API_URI, "/v2/tag/game_version");
    
    let versions = http_client.get(versions_uri).send()?.json::<Vec<MCVersion>>()?;

    
    let matching_versions = &versions
        .into_iter()
        .filter(|v| v.version_type == "release" && v.major == true)
        .collect::<Vec<MCVersion>>();

//TODO: Add 'more versions' option that removes filter on release and major.

    let default_version_idx = matching_versions
        .iter()
        .position(|v| v.version == "1.20.1")
        .unwrap_or(0);

    let version_selection_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game version")
        .items(&matching_versions)
        .default(default_version_idx)
        .interact()?;
    
    let version_selection = &matching_versions[version_selection_input];
    
    let default_java_version_idx = java_versions
    .iter()
    .position(|v| v.threshold.version <= version_selection.version)
    .unwrap_or(0);

    let java_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Java version")
        .items(&java_versions)
        .default(default_java_version_idx)
        .interact()?;

    let java_version_selection = &java_versions[java_version_input];

    // let default_loader = &supported_loaders
    //     .iter()
    //     .position(|l| *l == "fabric")
    //     .unwrap_or(0);

    // let loader_selection = supported_loaders[
    //     Select::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Loader")
    //         .items(supported_loaders)
    //         .default(*default_loader)
    //         .interact()?];

    let fabric_versions_uri = format!("{}{}", GITHUB_API_URI, "/repos/FabricMC/fabric/releases");
    
    let fabric_versions: Vec<FabricVersion> = http_client.get(fabric_versions_uri)
        .send()?
        .json::<Vec<FabricVersion>>()?
        .into_iter()
        .filter(|v| v.prerelease == false)
        .collect();

    let fabric_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Fabric version")
        .items(&fabric_versions)
        .default(0)
        .interact()?;

    let fabric_version_selection= &fabric_versions[fabric_version_input];

    let pickaxe_path = PathBuf::from("~/.config/pickaxe");
    let pickaxe_dir = fs::create_dir(&pickaxe_path)?;

    let java_path = pickaxe_path.join("java");
    let java_dir = fs::create_dir(java_path);

    

    Ok(())
}

fn add(os: OsString) {}

fn remove(os: OsString) {}

fn start(os: Option<OsString>) {}

fn stop(os: Option<OsString>) {}

