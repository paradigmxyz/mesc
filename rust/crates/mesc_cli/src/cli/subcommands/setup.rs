use crate::{print_defaults, MescCliError, SetupArgs};
use inquire::ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled};
use mesc::{ChainId, Endpoint, RpcConfig, TryIntoChainId};
use std::collections::{HashMap, HashSet};
use toolstr::Colorize;

pub(crate) fn setup_command(args: SetupArgs) -> Result<(), MescCliError> {
    inquire::set_global_render_config(get_render_config());

    if args.editor {
        return edit_config_in_editor(args);
    }

    let config_mode = mesc::load::get_config_mode()?;
    match config_mode {
        mesc::ConfigMode::Path => {
            println!("MESC is enabled");
            println!();
            println!("config stored at: {}", mesc::load::get_config_path()?.bold());
        }
        mesc::ConfigMode::Env => {
            println!("MESC is enabled");
            println!();
            println!("config stored in MESC_ENV environment variable");
        }
        mesc::ConfigMode::Disabled => {
            println!("MESC is not enabled");
        }
    };
    println!();

    loop {
        if mesc::is_mesc_enabled() {
            match mesc::load::load_config_data() {
                Ok(config) => return modify_existing_config(config),
                Err(_) => {
                    println!("The current MESC config contains improper data");
                    println!();
                    let options = vec!["Fix the data manually", "Create a new config from scratch"];
                    match inquire::Select::new("How to proceed?", options).prompt()? {
                        "Fix the data manually" => match config_mode {
                            mesc::ConfigMode::Path => {
                                let path = mesc::load::get_config_path()?;
                                edit::edit_file(path)?
                            }
                            _ => {
                                println!("Config data stored in MESC_ENV environment variable");
                                println!();
                                println!("Edit MESC_ENV in your terminal config files and then restart your terminal");
                            }
                        },
                        "Create a new config from scratch" => return setup_new_config(),
                        _ => println!("Improper selection"),
                    };
                    return Ok(());
                }
            }
        } else if let Ok(path) = mesc::load::get_config_path() {
            println!("a MESC config path has still been specified: {}", path.bold());

            if std::path::Path::new(&path).exists() {
                println!();
                match mesc::load::load_file_config() {
                    Ok(config) => {
                        println!("To enable MESC, select \"Setup environment\" below");
                        println!();
                        return modify_existing_config(config);
                    }
                    Err(_) => {
                        println!("This MESC config file contains improper data");
                        let options =
                            vec!["Fix the data manually", "Create a new config from scratch"];
                        match inquire::Select::new("How to proceed?", options).prompt()? {
                            "Fix the data manually" => edit::edit_file(path)?,
                            "Create a new config from scratch" => return setup_new_config(),
                            _ => {}
                        };
                    }
                }
            } else {
                println!();
                return setup_new_config();
            }
        } else {
            println!("Setting up new config");
            return setup_new_config();
        }
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
    println!("Creating new config...");
    println!();
    println!("default endpoint URL?");
    println!();
    println!("default endpoint name?");
    Ok(())
}

fn modify_existing_config(config: RpcConfig) -> Result<(), MescCliError> {
    let options = [
        "Setup environment",
        "Add new endpoint",
        "Modify endpoint",
        "Modify defaults",
        "Modify global metadata",
        "Print config as JSON",
        "Exit and save changes",
        "Exit without saving",
    ]
    .to_vec();
    let original_config = config.clone();
    let mut valid_config = config.clone();
    let mut config = config.clone();
    let mut config_write_mode: Option<ConfigWriteMode> = None;
    loop {
        // modify config
        match inquire::Select::new("What do you want to do?", options.clone())
            .with_page_size(10)
            .prompt()?
        {
            "Setup environment" => setup_environment(&mut config)?,
            "Add new endpoint" => add_endpoint(&mut config)?,
            "Modify endpoint" => modify_endpoint(&mut config)?,
            "Modify defaults" => modify_defaults(&mut config)?,
            "Modify global metadata" => modify_global_metadata(&mut config)?,
            "Print config as JSON" => {
                println!("{}", colored_json::to_colored_json_auto(&config)?)
            }
            "Exit and save" => break,
            "Exit without saving" => return Ok(()),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };

        // validation
        match config.validate() {
            Ok(()) => {}
            Err(e) => {
                println!("Invalid data: {:?}", e);
                println!("Reverting to previous config state");
                config = valid_config.clone();
                continue;
            }
        };

        valid_config = config.clone();
    }

    // write file
    if serde_json::to_string(&config)? != serde_json::to_string(&original_config)? {
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

fn setup_environment(_config: &mut RpcConfig) -> Result<(), MescCliError> {
    // print current environment
    let env_vars = ["MESC_MODE", "MESC_PATH", "MESC_ENV"];
    println!();
    println!("    Current environment variables:");
    for env_var in env_vars.iter() {
        match std::env::var(env_var) {
            Ok(value) => println!("         {}: {}", env_var.bold(), value.green()),
            Err(_) => println!("         {}: {}", env_var.bold(), "[not set]".green()),
        }
    }
    let overrides = [
        "MESC_NETWORK_NAMES",
        "MESC_NETWORK_DEFAULTS",
        "MESC_ENDPOINTS",
        "MESC_DEFAULT_ENDPOINT",
        "MESC_GLOBAL_METADATA",
        "MESC_ENDPOINT_METADATA",
        "MESC_PROFILES",
    ];
    println!("    Current environment overrides:");
    for env_var in overrides.iter() {
        match std::env::var(env_var) {
            Ok(value) => println!("         {}: {}", env_var.bold(), value.green()),
            Err(_) => println!("         {}: {}", env_var.bold(), "[not set]".green()),
        }
    }

    if mesc::is_mesc_enabled() {
        println!();
        println!(" MESC is already enabled. Disable by unsetting these environment variables");
        println!();
    } else {
        let options =
            vec!["Store MESC config in a file", "Store MESC config in an environment variable"];
        let prompt = "How do you want to setup your environment?";
        match inquire::Select::new(prompt, options).prompt()? {
            "Store MESC config in a file" => {
                let prompt = "Where should this file be saved? Provide a path";
                let _path = inquire::Text::new(prompt).prompt()?;
                // if path already exists, confirm overrite
                todo!()
            }
            "Store MESC config in an environment variable" => {}
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
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
    let endpoint = match inquire::Text::new("New endpoint URL?").prompt() {
        Ok(url) => {
            let default_name = "new_name";
            let input = inquire::Text::new("New endpoint name?").with_default(default_name);
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
        "Set the default endpoint",
        "Set the default endpoint for network",
        "Add new profile",
        "Modify existing profile",
        "Print current defaults",
        "Return to main menu",
    ]
    .to_vec();

    loop {
        match inquire::Select::new("What do you want to do?", options.clone()).prompt()? {
            "Set the default endpoint" => {
                let prompt = "Which endpoint should be the default?";
                let endpoint_name = select_endpoint(config, prompt)?;
                config.default_endpoint = Some(endpoint_name.clone());
                let endpoint = mesc::query::get_endpoint_by_name(config, endpoint_name.as_str())?;
                if let Some(chain_id) = endpoint.chain_id {
                    config.network_defaults.insert(chain_id, endpoint_name);
                };
            }
            "Set the default endpoint for network" => {
                let prompt = "Set the default endpoint for which network?";
                let chain_id = select_chain_id(config, prompt)?;
                let prompt = "What should be the default endpoint for this network?";
                let endpoint_name = select_endpoint(config, prompt)?;
                config.network_defaults.insert(chain_id, endpoint_name);
            }
            "Add new profile" => {
                let name = inquire::Text::new("Name?").prompt()?;
                if config.profiles.contains_key(&name) {
                    println!();
                } else {
                    config.profiles.insert(name.clone(), mesc::Profile::new(name));
                    println!(" profile added");
                }
            }
            "Modify existing profile" => {
                if config.profiles.is_empty() {
                    println!(" no profiles are currently configured");
                    continue;
                }

                let profile_name = select_profile(config, "Which profile to modify?")?;
                let options = vec![
                    "Set the profile's name",
                    "Set the profile's default endpoint",
                    "Set the profile's default endpoint for a network",
                ];
                match inquire::Select::new("What to modify?", options).prompt()? {
                    "Set the profile's name" => {
                        let new_name = inquire::Text::new("New name?").prompt()?;
                        if config.profiles.contains_key(&new_name) {
                            println!("profile with this name already exists");
                            continue;
                        };
                        if let Some(mut profile) = config.profiles.remove(&profile_name) {
                            profile.name = new_name.clone();
                            config.profiles.insert(new_name, profile);
                        } else {
                            println!("profile not present");
                        }
                    }
                    "Set the profile's default endpoint" => {
                        let prompt = "Which endpoint to use as profile default?";
                        let default_endpoint = select_endpoint(config, prompt)?;
                        let endpoint =
                            mesc::query::get_endpoint_by_name(config, &default_endpoint)?;
                        if let Some(profile) = config.profiles.get_mut(&profile_name) {
                            profile.default_endpoint = Some(default_endpoint.clone());
                            if let Some(chain_id) = endpoint.chain_id.clone() {
                                profile.network_defaults.insert(chain_id, default_endpoint);
                            }
                        } else {
                            println!("profile not present");
                        }
                    }
                    "Set the profile's default endpoint for a network" => {
                        let prompt = "Set the profile's default endpoint for which network?";
                        let chain_id = select_chain_id(config, prompt)?;
                        let prompt = "What should be the default endpoint for this network?";
                        let endpoint_name = select_endpoint(config, prompt)?;
                        if let Some(profile) = config.profiles.get_mut(&profile_name) {
                            profile.network_defaults.insert(chain_id, endpoint_name);
                        } else {
                            println!("profile not present");
                        }
                    }
                    _ => {
                        println!("invalid input");
                    }
                }
            }
            "Print current defaults" => {
                println!();
                print_defaults(config)?;
                println!();
            }
            "Return to main menu" => return Ok(()),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    }
}

fn select_endpoint(config: &RpcConfig, prompt: &str) -> Result<String, MescCliError> {
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    options.sort();
    Ok(inquire::Select::new(prompt, options).prompt()?)
}

fn select_profile(config: &RpcConfig, prompt: &str) -> Result<String, MescCliError> {
    let mut options: Vec<_> = config.profiles.keys().collect();
    options.sort();
    Ok(inquire::Select::new(prompt, options).prompt()?.to_string())
}

fn select_chain_id(config: &RpcConfig, prompt: &str) -> Result<ChainId, MescCliError> {
    let mut options: Vec<_> = vec![];
    for chain_id in config.network_defaults.keys() {
        options.push(chain_id.as_str())
    }
    options.push("Enter a chain_id");
    options.push("Enter a network name");
    match inquire::Select::new(prompt, options).prompt()? {
        "Enter a chain_id" => {
            let input = inquire::Text::new("Enter a chain_id").prompt()?;
            Ok(input.try_into_chain_id()?)
        }
        "Enter a network name" => {
            todo!()
        }
        input => Ok(input.try_into_chain_id()?),
    }
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
