mod commands;
mod fabric;
mod github;
mod instance;
mod java;
mod mc;
mod modrinth;

use commands::Commands;
use commands::add::add;
use commands::new::new;
use commands::remove::remove;
use commands::start::start;
use commands::stop::stop;

use anyhow::Result;
use clap::Parser;
use tokio;

#[derive(Debug, Parser)]
#[command(
    name = "pickaxe",
    about = "A tool for managing Minecraft clients and servers."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::New { client, server } => new(client, server).await,
        Commands::Add { path, package } => add(package),
        Commands::Remove { package } => remove(package),
        Commands::Start { path } => start(path),
        Commands::Stop { path } => stop(path),
    }
}
