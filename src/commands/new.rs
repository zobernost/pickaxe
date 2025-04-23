use crate::fabric::meta::{Game, Installer, Loader, Server};
use crate::{fabric, java};

use anyhow::Result;
use capitalize::Capitalize;
use console::{Term, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use number_names::ordinal;
use tokio::try_join;

pub async fn new() -> Result<()> {
    let term = Term::stdout();

    let title = style("New server").bold().green();

    term.write_line("")?;
    term.write_line(&title.to_string())?;
    term.write_line("")?;

    let server_number: String = ordinal((get_server_count()?) + 1);

    let default_name = format!(
        "My {} Server",
        server_number.to_string().capitalize()
    );

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Server name")
        .default(default_name.into())
        .interact_text()?;

    let http_client = reqwest::Client::builder()
        .user_agent("zobernost.pickaxe")
        .build()?;

    let game_versions = Game::get_all(&http_client).await?;

    let matching_game_versions = &game_versions
        .into_iter()
        .filter(|v| v.stable == true)
        .collect::<Vec<Game>>();

    let default_game_version_idx = matching_game_versions
        .iter()
        .position(|v| v.version == "1.20.1")
        .unwrap_or(0);

    let game_version_selection_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game version")
        .items(&matching_game_versions)
        .default(default_game_version_idx)
        .interact()?;

    let game_version = &matching_game_versions[game_version_selection_input];

    let java_versions = java::supported_java_versions();

    let default_java_version_idx = 0;

    let java_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Java version")
        .items(&java_versions)
        .default(default_java_version_idx)
        .interact()?;

    let java_version = &java_versions[java_version_input];

    let fabric_versions = Loader::get_all(&game_version.to_string(), &http_client).await?;

    let matching_fabric_versions = &fabric_versions
        .into_iter()
        .filter(|v| v.stable == true)
        .collect::<Vec<Loader>>();

    let fabric_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Fabric version")
        .items(&matching_fabric_versions)
        .default(0)
        .interact()?;

    let fabric_version = &matching_fabric_versions[fabric_version_input];

    let installer_version = Installer::get_latest(&http_client).await?;

    let server = Server::new(
        name,
        game_version.to_string(),
        fabric_version.to_string(),
        installer_version.to_string(),
        java_version.to_string(),
    );
    server.build(&http_client).await?;
    Ok(())

}

fn get_server_count() -> Result<u64> {
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let pickaxe_dir = config_dir.join("pickaxe");
    let servers_dir = pickaxe_dir.join("servers");
    match std::fs::read_dir(servers_dir.clone()) {
        Ok(dir) => Ok(dir.count() as u64),
        Err(_) => Ok(0),
    }
}