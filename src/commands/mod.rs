use clap::Parser;
use std::ffi::OsString;

pub mod add;
pub mod new;
pub mod remove;
pub mod start;
pub mod stop;

#[derive(Debug, Parser)]
pub enum Commands {
    #[command(
        about = "Create a new client or server instance",
        alias = "n",
        alias = "i",
        alias = "init"
    )]
    New {
        #[arg(short, long, conflicts_with = "server")]
        client: bool,
        #[arg(short, long, conflicts_with = "client")]
        server: bool,
    },
    #[command(
        about = "Add mods, data packs, or other packages to an existing instance",
        alias = "a"
    )]
    Add {
        #[arg(short, long)]
        path: Option<OsString>,
        package: OsString,
    },
    #[command(about = "Remove packages from an existing instance", alias = "r")]
    Remove { package: OsString },
    #[command(about = "Start an instance")]
    Start { path: Option<OsString> },
    #[command(about = "Stop an instance")]
    Stop { path: Option<OsString> },
}
