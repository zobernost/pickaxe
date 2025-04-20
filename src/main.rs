use clap::{builder::styling::Color, ArgGroup, Args, Parser, Subcommand, ValueEnum};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest::blocking;
use serde::Deserialize;
use std::{ffi::{OsStr, OsString}, fmt};

const MODRINTH_URI: &str = "https://api.modrinth.com";

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
struct Version {
    version: String,
    version_type: String,
    date: String,
    major: bool,
}

impl fmt::Display for Version {
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

const SUPPORTED_LOADERS: &[&str] = &["fabric", "forge", "vanilla"];

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
    let versions_uri = format!("{}{}", MODRINTH_URI, "/v2/tag/game_version");
    let versions = blocking::get(versions_uri)?
        .json::<Vec<Version>>()?
        .into_iter()
        .filter(|v| v.version_type == "release" && v.major == true)
        .collect::<Vec<Version>>();
    let default_version_idx = &versions
        .iter()
        .position(|v| v.version == "1.20.1")
        .unwrap_or(0);
    let version_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a version")
        .items(&versions)
        .default(*default_version_idx)
        .interact();
    let default_loader = &SUPPORTED_LOADERS
        .iter()
        .position(|l| *l == "fabric")
        .unwrap_or(0);
    let loader_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a loader")
        .items(SUPPORTED_LOADERS)
        .default(*default_loader)
        .interact();
    Ok(())
}

fn add(os: OsString) {}

fn remove(os: OsString) {}

fn start(os: Option<OsString>) {}

fn stop(os: Option<OsString>) {}

