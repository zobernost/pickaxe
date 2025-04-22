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
        about = "Create a new server",
        alias = "n",
        alias = "i",
        alias = "init"
    )]
    New {
    },
    #[command(
        about = "Add mods, data packs, or other packages to an existing server",
        alias = "a"
    )]
    Add {
        #[arg(short, long)]
        path: Option<OsString>,
        package: OsString,
    },
    #[command(about = "Remove packages from an existing server", alias = "r")]
    Remove { package: OsString },
    #[command(about = "Start a server")]
    Start { path: Option<OsString> },
    #[command(about = "Stop a server")]
    Stop { path: Option<OsString> },
}
