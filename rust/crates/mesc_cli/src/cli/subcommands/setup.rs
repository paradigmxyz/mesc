use crate::{MescCliError, SetupArgs};
use inquire::ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use mesc::{Endpoint, RpcConfig, TryIntoChainId};
use std::collections::{HashMap, HashSet};
use toolstr::Colorize;

pub(crate) fn setup_command(args: SetupArgs) -> Result<(), MescCliError> {
    inquire::set_global_render_config(get_render_config());

    if args.editor {
        return edit_config_in_editor(args);
    }

    if mesc::is_mesc_enabled() {
        let config = mesc::load::load_config_data()?;
        println!("MESC is enabled");
        println!();
        modify_existing_config(config)
    } else {
        setup_new_config()
    }
}

fn edit_config_in_editor(args: SetupArgs) -> Result<(), MescCliError> {
    if let Some(path) = args.path {
        edit::edit_file(path)?
    } else if let Ok(mesc::ConfigMode::Path) = mesc::load::get_config_mode() {
        let path = mesc::load::get_config_path()?;
        edit::edit_file(path)?
    } else {
        return Err(MescCliError::Error("no file to edit".to_string()));
    };
    Ok(())
}

fn setup_new_config() -> Result<(), MescCliError> {
    println!("MESC is not enabled, creating config now...");
    println!();
    println!("default endpoint URL?");
    println!();
    println!("default endpoint name?");
    Ok(())
}

fn modify_existing_config(config: RpcConfig) -> Result<(), MescCliError> {
    let options = [
        "Add new endpoint",
        "Modify endpoint",
        "Modify defaults",
        "Modify global metadata",
        "Exit and save",
    ]
    .to_vec();
    let mut old_config = config.clone();
    let mut config = config.clone();
    let mut config_write_mode: Option<ConfigWriteMode> = None;
    loop {
        // modify config
        match inquire::Select::new("What do you want to do?", options.clone()).prompt()? {
            "Add new endpoint" => add_endpoint(&mut config)?,
            "Modify endpoint" => modify_endpoint(&mut config)?,
            "Modify defaults" => modify_defaults(&mut config)?,
            "Modify global metadata" => modify_global_metadata(&mut config)?,
            "Exit and save" => break,
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };

        // validation
        match config.validate() {
            Ok(()) => {}
            Err(e) => {
                println!("invalid data: {:?}", e);
                println!("reverting to previous config state");
                config = old_config.clone();
                continue;
            }
        };

        old_config = config.clone();
    }

    // write file
    if serde_json::to_string(&config)? != serde_json::to_string(&config)? {
        if config_write_mode.is_none() {
            config_write_mode = Some(get_config_write_mode()?);
        };
        let write_mode = match &config_write_mode {
            Some(write_mode) => write_mode,
            None => return Err(MescCliError::Error("could not obtain write mode".to_string())),
        };
        match inquire::Confirm::new("Save changes to file?").prompt() {
            Ok(true) => write_config(&mut config, write_mode)?,
            Ok(false) => {}
            Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    } else {
        println!(" {}", "No updates to save".bold())
    }
    Ok(())
}

#[derive(Clone)]
enum ConfigWriteMode {
    /// file path of JSON config file
    Path(String),
    /// list of config files to write to
    Env(Vec<String>),
}

fn get_config_write_mode() -> Result<ConfigWriteMode, MescCliError> {
    let prompt = "How do you want to save the config?";
    let options = ["Save as path file", "Save as an environment variable"].to_vec();
    loop {
        let write_mode = match inquire::Select::new(prompt, options.clone()).prompt()? {
            "Save as path file" => {
                let text = inquire::Text::new("Path?").prompt()?;
                ConfigWriteMode::Path(text)
            }
            "Save an environment variable" => {
                println!("not supported yet");
                continue;
            }
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };
        return Ok(write_mode);
    }
}

fn write_config(
    _config: &mut RpcConfig,
    _write_mode: &ConfigWriteMode,
) -> Result<(), MescCliError> {
    todo!()
}

fn add_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let endpoint = match inquire::Text::new("Endpoint URL?").prompt() {
        Ok(url) => {
            let default_name = "new_name";
            let input = inquire::Text::new("Endpoint name?").with_default(default_name);
            let name = match input.prompt() {
                Ok(choice) => choice,
                Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            Endpoint {
                url,
                name,
                chain_id: None,
                endpoint_metadata: std::collections::HashMap::new(),
            }
        }
        Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };
    config.endpoints.insert(endpoint.name.clone(), endpoint);
    Ok(())
}

fn modify_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    // select endpoint
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    options.sort();
    let endpoint_name = inquire::Select::new("Which endpoint to modify?", options).prompt()?;
    let mut endpoint = match config.endpoints.get_mut(&endpoint_name) {
        Some(endpoint) => endpoint.clone(),
        None => return Err(MescCliError::InvalidInput("endpoint does not exist".to_string())),
    };
    let old_endpoint = endpoint.clone();
    println!(
        " {}: {}",
        "Current endpoint data".bold(),
        colored_json::to_colored_json_auto(&serde_json::to_value(&endpoint)?)?
    );

    // gather modifications
    let halt_options: HashSet<&str> = vec!["Delete endpoint", "Done"].into_iter().collect();
    let mut option = query_modify_endpoint(&mut endpoint, config, true)?;
    loop {
        if halt_options.contains(option.as_str()) {
            break;
        }
        option = query_modify_endpoint(&mut endpoint, config, false)?;
    }

    // commit modifications
    println!(
        " {}: {}",
        "New endpoint data".bold(),
        colored_json::to_colored_json_auto(&serde_json::to_value(&endpoint)?)?
    );

    config.endpoints.remove(&old_endpoint.name);
    config.endpoints.insert(endpoint.name.clone(), endpoint);

    Ok(())
}

fn query_modify_endpoint(
    endpoint: &mut Endpoint,
    config: &mut RpcConfig,
    first_change: bool,
) -> Result<String, MescCliError> {
    let options = [
        "Modify endpoint name",
        "Modify endpoint url",
        "Modify endpoint chain_id",
        "Modify endpoint metadata",
        "Delete endpoint",
        "Done",
    ]
    .to_vec();

    let message = if first_change { "How to modify endpoint?" } else { "Any other modifications?" };

    let option = inquire::Select::new(message, options.clone()).prompt()?;

    match option {
        "Modify endpoint name" => {
            let new_name = inquire::Text::new("New name?").prompt()?;
            mesc::write::update_endpoint_name(config, endpoint.name.as_str(), new_name.as_str())?;
            endpoint.name = new_name;
        }
        "Modify endpoint url" => {
            endpoint.url = inquire::Text::new("New url?").prompt()?;
        }
        "Modify endpoint chain_id" => {
            let chain_id = inquire::Text::new("New chain_id?").prompt()?;
            mesc::write::update_endpoint_chain_id(
                config,
                endpoint.name.as_str(),
                chain_id.clone(),
            )?;
            endpoint.chain_id = Some(chain_id.try_into_chain_id()?);
        }
        "Modify endpoint metadata" => {
            let edited = edit::edit(serde_json::to_string(&endpoint.endpoint_metadata)?)?;
            let value: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
                serde_json::from_str(&edited);
            endpoint.endpoint_metadata = value?;
        }
        "Delete endpoint" => {
            mesc::write::delete_endpoint(config, endpoint.name.as_str())?;
            println!("{} {}", "Deleted endpoint:".red(), endpoint.name.green());
        }
        "Done" => {}
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    Ok(option.to_string())
}

fn modify_global_metadata(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let old_metadata = serde_json::to_string(&config.global_metadata)?;
    let new_metadata = edit::edit(&old_metadata)?;
    if old_metadata == new_metadata {
        println!(" {}", "global metadata unchanged".bold());
    } else {
        let value: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
            serde_json::from_str(&new_metadata);
        config.global_metadata = value?;
        println!(" {}", "Global metadata updated".bold());
    }
    Ok(())
}

fn modify_defaults(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let options = [
        "Set default endpoint",
        "Set default endpoint for network",
        "Add new profile",
        "Modify existing profile",
    ]
    .to_vec();

    match inquire::Select::new("What do you want to do?", options).prompt()? {
        "Set default endpoint" => set_default_endpoint(config),
        "Set default endpoint for network" => set_default_endpoint_for_network(config),
        "Add new profile" => add_new_profile(config),
        "Modify existing profile" => modify_existing_profile(config),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

fn set_default_endpoint(_config: &mut RpcConfig) -> Result<(), MescCliError> {
    todo!()
}

fn set_default_endpoint_for_network(_config: &mut RpcConfig) -> Result<(), MescCliError> {
    todo!()
}

fn add_new_profile(_config: &mut RpcConfig) -> Result<(), MescCliError> {
    todo!()
}

fn modify_existing_profile(_config: &mut RpcConfig) -> Result<(), MescCliError> {
    todo!()
}

fn get_render_config() -> RenderConfig {
    let highlight_color = Color::DarkGreen;

    let mut render_config = RenderConfig::default();
    render_config.prompt = StyleSheet::new().with_attr(Attributes::BOLD);
    render_config.prompt_prefix = Styled::new("").with_fg(Color::LightRed);
    render_config.answered_prompt_prefix = Styled::new("").with_fg(Color::LightRed);
    render_config.placeholder = StyleSheet::new().with_fg(Color::LightRed);
    render_config.selected_option = Some(StyleSheet::new().with_fg(highlight_color));
    render_config.highlighted_option_prefix = Styled::new("→").with_fg(highlight_color);
    render_config.selected_checkbox = Styled::new("☑").with_fg(highlight_color);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");
    render_config.option_index_prefix = IndexPrefix::Simple;
    render_config.error_message =
        render_config.error_message.with_prefix(Styled::new("❌").with_fg(Color::LightRed));
    render_config.answer = StyleSheet::new().with_attr(Attributes::BOLD).with_fg(highlight_color);
    let grey = Color::Rgb { r: 100, g: 100, b: 100 };
    render_config.help_message = StyleSheet::new().with_fg(grey).with_attr(Attributes::ITALIC);

    render_config
}
