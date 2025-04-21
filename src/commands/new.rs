use crate::fabric;
use crate::github;
use crate::instance::{Instance, InstanceType, get_instance_count};
use crate::java;
use crate::mc;
use crate::modrinth;

use anyhow::Result;
use capitalize::Capitalize;
use console::{Term, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use number_names::ordinal;

pub async fn new(client: bool, mut server: bool) -> Result<()> {
    let term = Term::stdout();

    let title = style("New instance").bold().green();

    term.write_line("")?;
    term.write_line(&title.to_string())?;
    term.write_line("")?;

    if client == false && server == false {
        let type_input = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Instance type")
            .items(&["Client", "Server"])
            .default(0)
            .interact()?;

        if type_input == 1 {
            server = true;
        }
    }

    let r#type: InstanceType = if server == true {
        InstanceType::Server
    } else {
        InstanceType::Client
    };

    let instance_number: String = match r#type {
        InstanceType::Server => ordinal((get_instance_count(InstanceType::Server)?) + 1),
        InstanceType::Client => ordinal((get_instance_count(InstanceType::Client)?) + 1),
    };

    let default_name = format!(
        "My {} {} Instance",
        instance_number.to_string().capitalize(),
        r#type
    );

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Instance name")
        .default(default_name.into())
        .interact_text()?;

    let http_client = reqwest::Client::builder()
        .user_agent("zobernost.pickaxe")
        .build()?;

    let versions_uri = format!("{}{}", modrinth::URL, "/v2/tag/game_version");

    let versions = http_client
        .get(versions_uri)
        .send()
        .await?
        .json::<Vec<mc::Version>>()
        .await?;

    let matching_versions = &versions
        .into_iter()
        .filter(|v| v.version_type == "release" && v.major == true)
        .collect::<Vec<mc::Version>>();

    let default_version_idx = matching_versions
        .iter()
        .position(|v| v.version == "1.20.1")
        .unwrap_or(0);

    let version_selection_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Game version")
        .items(&matching_versions)
        .default(default_version_idx)
        .interact()?;

    let version = &matching_versions[version_selection_input];

    let java_versions = java::supported_java_versions();

    let default_java_version_idx = java_versions
        .iter()
        .position(|v| v.threshold.version <= version.version)
        .unwrap_or(0);

    let java_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Java version")
        .items(&java_versions)
        .default(default_java_version_idx)
        .interact()?;

    let java_version = &java_versions[java_version_input];

    let fabric_versions_uri = format!("{}{}", github::URL, "/repos/FabricMC/fabric/releases");

    let fabric_versions: Vec<fabric::Version> = http_client
        .get(fabric_versions_uri)
        .send()
        .await?
        .json::<Vec<fabric::Version>>()
        .await?
        .into_iter()
        .filter(|v| v.prerelease == false)
        .collect();

    let fabric_version_input = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Fabric version")
        .items(&fabric_versions)
        .default(0)
        .interact()?;

    let fabric_version = &fabric_versions[fabric_version_input];

    let instance = Instance::new(
        name.clone(),
        r#type,
        version.clone(),
        fabric_version.clone(),
        java_version.clone(),
    );

    let build = tokio::spawn(async move { instance.build().await });

    build.await;

    Ok(())
}
