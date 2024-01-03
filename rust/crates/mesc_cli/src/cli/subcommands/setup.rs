use crate::{print_defaults, MescCliError, SetupArgs};
use inquire::{
    ui::{Attributes, Color, IndexPrefix, RenderConfig, StyleSheet, Styled},
    InquireError,
};
use mesc::{ChainId, Endpoint, RpcConfig, TryIntoChainId};
use std::collections::{HashMap, HashSet};
use toolstr::Colorize;

pub(crate) async fn setup_command(args: SetupArgs) -> Result<(), MescCliError> {
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
                Ok(config) => return modify_existing_config(config, None).await,
                Err(_) => {
                    println!("The current MESC config contains improper data");
                    println!();
                    let options = vec!["Fix the data manually", "Create a new config from scratch"];
                    match inquire::Select::new("How to proceed?", options).prompt() {
                        Ok("Fix the data manually") => match config_mode {
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
                        Ok("Create a new config from scratch") => return setup_new_config().await,
                        Err(InquireError::OperationCanceled) => {
                            println!("exiting");
                            std::process::exit(1);
                        }
                        _ => println!("Improper selection"),
                    };
                    return Ok(());
                }
            }
        } else if let Ok(path) = mesc::load::get_config_path() {
            println!("a MESC config path has still been specified: {}", path.bold());

            if std::path::Path::new(&path).exists() {
                println!();
                match mesc::load::load_file_config(None) {
                    Ok(config) => {
                        println!("To enable MESC, select \"Setup environment\" below");
                        println!();
                        return modify_existing_config(config, None).await;
                    }
                    Err(_) => {
                        println!("This MESC config file contains improper data");
                        let options =
                            vec!["Fix the data manually", "Create a new config from scratch"];
                        match inquire::Select::new("How to proceed?", options).prompt() {
                            Ok("Fix the data manually") => edit::edit_file(path)?,
                            Ok("Create a new config from scratch") => {
                                return setup_new_config().await
                            }
                            Err(InquireError::OperationCanceled) => {
                                println!("exiting");
                                std::process::exit(1);
                            }
                            _ => {}
                        };
                    }
                }
            } else {
                println!();
                return setup_new_config().await;
            }
        } else {
            return setup_new_config().await;
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

async fn setup_new_config() -> Result<(), MescCliError> {
    println!(" {}", "Creating new config...".bold());
    let mut write_mode = None;
    let mut config = RpcConfig::default();
    setup_environment(&mut config, &mut write_mode)?;
    if let Some(ConfigWriteMode::Path(path)) = write_mode.clone() {
        if std::path::Path::new(&path).exists() {
            println!(" Config file already exists, loading");
            config = mesc::load::load_file_config(Some(path))?;
        } else {
            mesc::write::write_config(config.clone(), path)?;
            println!(" {}", "Empty configuration created".bold());
        }
    }

    modify_existing_config(config, write_mode).await?;

    Ok(())
}

async fn modify_existing_config(
    config: RpcConfig,
    mut config_write_mode: Option<ConfigWriteMode>,
) -> Result<(), MescCliError> {
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
    // let mut config_write_mode: Option<ConfigWriteMode> = None;
    loop {
        // modify config
        let input = match inquire::Select::new("What do you want to do?", options.clone())
            .with_page_size(10)
            .prompt()
        {
            Ok(input) => input,
            Err(InquireError::OperationCanceled) => {
                println!(" Exiting without saving");
                std::process::exit(0)
            },
            Err(e) => return Err(e.into()),
        };
        match input {
            "Setup environment" => setup_environment(&mut config, &mut config_write_mode)?,
            "Add new endpoint" => add_endpoint(&mut config).await?,
            "Modify endpoint" => modify_endpoint(&mut config).await?,
            "Modify defaults" => modify_defaults(&mut config)?,
            "Modify global metadata" => modify_global_metadata(&mut config)?,
            "Print config as JSON" => {
                println!("{}", colored_json::to_colored_json_auto(&config)?)
            }
            "Exit and save changes" => break,
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
            config_write_mode = get_config_write_mode()?;
        };
        if config_write_mode.is_none() {
            println!(" No write mode selected");
            println!(" Not writing changes to disk");
        } else {
            match &config_write_mode {
                Some(write_mode) => match write_mode {
                    ConfigWriteMode::Path(path) => {
                        mesc::write::write_config(config, path.clone())?;
                        println!(" {} {}", "config written to".bold(), path.green());
                    }
                    ConfigWriteMode::Env(_) => {
                        todo!("writing environment variables not supported yet")
                    }
                },
                None => return Err(MescCliError::Error("could not obtain write mode".to_string())),
            };
        }
    } else {
        println!(" {}", "No updates to save".bold())
    }
    Ok(())
}

fn setup_environment(
    _config: &mut RpcConfig,
    config_write_mode: &mut Option<ConfigWriteMode>,
) -> Result<(), MescCliError> {
    if mesc::is_mesc_enabled() {
        println!();
        println!(" MESC is already enabled");
        println!(" Using path {}", mesc::load::get_config_path()?.green().bold());
        println!();
    } else if let Some(config_write_mode) = config_write_mode {
        match config_write_mode {
            ConfigWriteMode::Path(path) => {
                println!(" MESC not yet enabled, but will write config to {}", path.green().bold())
            }
            ConfigWriteMode::Env(_) => {
                println!(" ENV mode not yet available in the interactive cli")
            }
        };
    } else {
        let options =
            vec!["Store MESC config in a file", "Store MESC config in an environment variable"];
        let prompt = "How do you want to store your MESC config?";
        match inquire::Select::new(prompt, options).prompt() {
            Ok("Store MESC config in a file") => {
                let prompt = "Where should mesc.json file be saved? (enter a directory path)";
                let parent = match inquire::Text::new(prompt).prompt() {
                    Ok(parent) => {
                        let parent = if parent.trim().is_empty() {
                            ".".to_string()
                        } else {
                            mesc::load::expand_path(parent)?
                        };
                        std::path::PathBuf::from(parent)
                    }
                    Err(InquireError::OperationCanceled) => return Ok(()),
                    Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                };
                let path: String = parent.join("mesc.json").to_string_lossy().to_string();
                *config_write_mode = Some(ConfigWriteMode::Path(path.to_string()));
                println!(
                    " Insert this line into your {} and {} files:",
                    "~/.bashrc".green().bold(),
                    "~/.profile".green().bold()
                );
                println!(" {}{}", "MESC_PATH=".green().bold(), path.green().bold());
                println!(" Then restart your terminal shell");
            }
            Ok("Store MESC config in an environment variable") => {
                println!(" This is not available in the interactive MESC cli yet");
            }
            Err(InquireError::OperationCanceled) => {}
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

fn get_config_write_mode() -> Result<Option<ConfigWriteMode>, MescCliError> {
    if let Ok(path) = mesc::load::get_config_path() {
        return Ok(Some(ConfigWriteMode::Path(path)));
    };

    let prompt = "How do you want to save the config?";
    let options = ["Save as path file", "Save as an environment variable"].to_vec();
    loop {
        let write_mode = match inquire::Select::new(prompt, options.clone()).prompt() {
            Ok("Save as path file") => {
                let prompt = "Where should mesc.json file be saved? (enter a directory path)";
                let parent = match inquire::Text::new(prompt).prompt() {
                    Ok(parent) => {
                        let parent = if parent.trim().is_empty() {
                            ".".to_string()
                        } else {
                            mesc::load::expand_path(parent)?
                        };
                        std::path::PathBuf::from(parent)
                    }
                    Err(InquireError::OperationCanceled) => return Ok(None),
                    Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                };
                let path: String = parent.join("mesc.json").to_string_lossy().to_string();
                Some(ConfigWriteMode::Path(path.to_string()))
            }
            Ok("Save an environment variable") => {
                println!("not supported yet");
                continue;
            }
            Err(InquireError::OperationCanceled) => return Ok(None),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        };
        return Ok(write_mode);
    }
}

fn is_ip(url: &str) -> bool {
    use std::net::{Ipv4Addr, Ipv6Addr};
    if url.is_empty() {
        return false;
    }
    let domain = match url.split_once('/') {
        Some((domain, _)) => domain,
        None => url,
    };
    let domain = domain.split(':').next().unwrap_or("");
    domain.parse::<Ipv4Addr>().is_ok() || domain.parse::<Ipv6Addr>().is_ok()
}

async fn add_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    let url = match inquire::Text::new("New endpoint URL?").prompt() {
        Ok(input) => input,
        Err(InquireError::OperationCanceled) => return Ok(()),
        Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    // add transport protocol
    let url = if !url.starts_with("http") & !url.starts_with("ws") & !url.ends_with(".ipc") {
        if url.starts_with("localhost") | is_ip(&url) {
            let url = format!("http://{}", url);
            println!(
                " Transport protocol not included. Defaulting to http: {}",
                url.green().bold()
            );
            url
        } else {
            let url = format!("https://{}", url);
            println!(
                " Transport protocol not included. Defaulting to https: {}",
                url.green().bold()
            );
            url
        }
    } else {
        url
    };

    // get chain_id
    println!(" Querying chain id...");
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(4)).build()?;
    let chain_id = crate::rpc::request_chain_id(client, url.clone()).await;
    let chain_id = match chain_id {
        Ok(chain_id) => {
            println!(" {} {}", "Using chain_id".bold(), chain_id.as_str().green());
            Some(chain_id)
        }
        _ => {
            println!(" {}", "Could not detect chain id".red());
            select_chain_id("How to proceed?".to_string()).await?
        }
    };

    // get name
    let default_name = mesc::overrides::get_default_endpoint_name(&url, chain_id.clone());
    let mut input = inquire::Text::new("New endpoint name?");
    if let Some(default_name) = default_name.as_ref() {
        input = input.with_default(default_name);
    }
    let name = match input.prompt() {
        Ok(choice) => choice,
        Err(InquireError::OperationCanceled) => return Ok(()),
        Err(_) => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    // create endpoint
    let endpoint =
        Endpoint { url, name, chain_id, endpoint_metadata: std::collections::HashMap::new() };
    config.endpoints.insert(endpoint.name.clone(), endpoint);
    println!(" {}", "New endpoint added".bold());
    Ok(())
}

async fn select_chain_id(prompt: String) -> Result<Option<ChainId>, MescCliError> {
    let options = vec![
        "Search known network names",
        "Enter endpoint chain id manually",
        "Do not use a chain id for this endpoint",
    ];
    loop {
        match inquire::Select::new(prompt.as_str(), options.clone()).prompt() {
            Ok("Do not use a chain id for this endpoint") => return Ok(None),
            Ok("Enter endpoint chain id manually") => {
                match inquire::Text::new("Chain id?").prompt() {
                    Ok(text) => {
                        match text.try_into_chain_id() {
                            Ok(chain_id) => return Ok(Some(chain_id)),
                            _ => continue,
                        };
                    }
                    Err(InquireError::OperationCanceled) => return Ok(None),
                    _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                }
            }
            Ok("Search known network names") => {
                println!(
                    " Fetching network names from {}...",
                    "https://chainid.network".green().bold()
                );
                match select_chain_id_by_name().await {
                    Ok(Some(chain_id)) => return Ok(Some(chain_id)),
                    Ok(None) => return Ok(None),
                    _ => continue,
                }
            }
            Err(InquireError::OperationCanceled) => return Ok(None),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    }
}

async fn modify_endpoint(config: &mut RpcConfig) -> Result<(), MescCliError> {
    // select endpoint
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    options.sort();
    let endpoint_name = match inquire::Select::new("Which endpoint to modify?", options).prompt() {
        Ok(name) => name,
        Err(InquireError::OperationCanceled) => return Ok(()),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };
    let endpoint = match config.endpoints.get(&endpoint_name) {
        Some(endpoint) => endpoint.clone(),
        None => return Err(MescCliError::InvalidInput("endpoint does not exist".to_string())),
    };
    println!(" {}", "Current endpoint data:".bold(),);
    println!();
    let colored = colored_json::to_colored_json_auto(&serde_json::to_value(&endpoint)?)?;
    for line in colored.split('\n') {
        println!("    {}", line);
    }
    println!();

    // gather modifications
    let halt_options: HashSet<&str> = vec!["Delete endpoint", "Done"].into_iter().collect();
    let mut option = query_modify_endpoint(endpoint_name.clone(), config, true).await?;
    loop {
        if halt_options.contains(option.as_str()) {
            break;
        }
        option = query_modify_endpoint(endpoint_name.clone(), config, false).await?;
    }

    // commit modifications
    if option != "Delete endpoint" {
        let endpoint = match config.endpoints.get(&endpoint_name) {
            Some(endpoint) => endpoint.clone(),
            None => return Err(MescCliError::InvalidInput("endpoint does not exist".to_string())),
        };
        println!(" {}", "New endpoint data:".bold(),);
        println!();
        let colored = colored_json::to_colored_json_auto(&serde_json::to_value(endpoint)?)?;
        for line in colored.split('\n') {
            println!("    {}", line);
        }
        println!();
    }

    Ok(())
}

async fn query_modify_endpoint(
    endpoint_name: String,
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

    let option = match inquire::Select::new(message, options.clone()).prompt() {
        Ok(answer) => answer,
        Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };

    match option {
        "Modify endpoint name" => {
            let new_name = match inquire::Text::new("New name?").prompt() {
                Ok(answer) => answer,
                Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            mesc::write::update_endpoint_name(config, endpoint_name.as_str(), new_name.as_str())?;
        }
        "Modify endpoint url" => {
            let new_url = match inquire::Text::new("New url?").prompt() {
                Ok(answer) => answer,
                Err(InquireError::OperationCanceled) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
            if let Some(endpoint) = config.endpoints.get_mut(&endpoint_name) {
                endpoint.url = new_url;
            }
        }
        "Modify endpoint chain_id" => {
            match select_chain_id("New chain_id?".to_string()).await {
                Ok(Some(chain_id)) => mesc::write::update_endpoint_chain_id(
                    config,
                    endpoint_name.as_str(),
                    chain_id.clone(),
                )?,
                Ok(None) => return Ok("Done".to_string()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            };
        }
        "Modify endpoint metadata" => {
            if let Some(endpoint) = config.endpoints.get_mut(&endpoint_name) {
                let edited = edit::edit(serde_json::to_string(&endpoint.endpoint_metadata)?)?;
                let value: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
                    serde_json::from_str(&edited);
                endpoint.endpoint_metadata = value?;
            }
        }
        "Delete endpoint" => {
            mesc::write::delete_endpoint(config, endpoint_name.as_str())?;
            println!("{} {}", "Deleted endpoint:".red(), endpoint_name.green());
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
    println!();
    print_defaults(config)?;
    println!();

    let options = [
        "Set the default endpoint",
        "Set the default endpoint for network",
        "Add new profile",
        "Modify existing profile",
        "Print current defaults",
        "Done modifying defaults",
    ]
    .to_vec();

    loop {
        match inquire::Select::new("What do you want to do?", options.clone()).prompt() {
            Ok("Set the default endpoint") => {
                let prompt = "Which endpoint should be the default?";
                let endpoint_name = match select_endpoint(config, prompt)? {
                    Some(value) => value,
                    _ => return Ok(()),
                };
                config.default_endpoint = Some(endpoint_name.clone());
                let endpoint = mesc::query::get_endpoint_by_name(config, endpoint_name.as_str())?;
                if let Some(chain_id) = endpoint.chain_id {
                    config.network_defaults.insert(chain_id, endpoint_name);
                };
            }
            Ok("Set the default endpoint for network") => {
                let prompt = "Set the default endpoint for which network?";
                let chain_id = match select_config_chain_id(config, prompt)? {
                    Some(value) => value,
                    _ => return Ok(()),
                };
                let prompt = "What should be the default endpoint for this network?";
                let endpoint_name = match select_endpoint_of_network(config, &chain_id, prompt)? {
                    Some(value) => value,
                    _ => return Ok(()),
                };
                config.network_defaults.insert(chain_id, endpoint_name);
            }
            Ok("Add new profile") => {
                let name = match inquire::Text::new("Name?").prompt() {
                    Ok(answer) => answer,
                    Err(InquireError::OperationCanceled) => return Ok(()),
                    _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                };
                if config.profiles.contains_key(&name) {
                    println!();
                } else {
                    config.profiles.insert(name.clone(), mesc::Profile::new(name));
                    println!(" profile added");
                }
            }
            Ok("Modify existing profile") => {
                if config.profiles.is_empty() {
                    println!(" no profiles are currently configured");
                    continue;
                }

                let profile_name = match select_profile(config, "Which profile to modify?")? {
                    Some(value) => value,
                    _ => return Ok(()),
                };
                let options = vec![
                    "Set the profile's name",
                    "Set the profile's default endpoint",
                    "Set the profile's default endpoint for a network",
                ];
                match inquire::Select::new("What to modify?", options).prompt() {
                    Ok("Set the profile's name") => {
                        let new_name = match inquire::Text::new("New name?").prompt() {
                            Ok(answer) => answer,
                            Err(InquireError::OperationCanceled) => return Ok(()),
                            _ => {
                                return Err(MescCliError::InvalidInput("invalid input".to_string()))
                            }
                        };
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
                    Ok("Set the profile's default endpoint") => {
                        let prompt = "Which endpoint to use as profile default?";
                        let default_endpoint = match select_endpoint(config, prompt)? {
                            Some(value) => value,
                            _ => return Ok(()),
                        };
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
                    Ok("Set the profile's default endpoint for a network") => {
                        let prompt = "Set the profile's default endpoint for which network?";
                        let chain_id = match select_config_chain_id(config, prompt)? {
                            Some(value) => value,
                            _ => return Ok(()),
                        };
                        let prompt = "What should be the default endpoint for this network?";
                        let endpoint_name =
                            match select_endpoint_of_network(config, &chain_id, prompt)? {
                                Some(value) => value,
                                _ => return Ok(()),
                            };
                        if let Some(profile) = config.profiles.get_mut(&profile_name) {
                            profile.network_defaults.insert(chain_id, endpoint_name);
                        } else {
                            println!("profile not present");
                        }
                    }
                    Err(InquireError::OperationCanceled) => return Ok(()),
                    _ => {
                        println!("invalid input");
                    }
                }
            }
            Ok("Print current defaults") => {
                println!();
                print_defaults(config)?;
                println!();
            }
            Ok("Done modifying defaults") => return Ok(()),
            Err(InquireError::OperationCanceled) => return Ok(()),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    }
}

fn select_endpoint(config: &RpcConfig, prompt: &str) -> Result<Option<String>, MescCliError> {
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer)),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

fn select_endpoint_of_network(
    config: &RpcConfig,
    chain_id: &ChainId,
    prompt: &str,
) -> Result<Option<String>, MescCliError> {
    let mut options: Vec<String> = config
        .endpoints
        .clone()
        .values()
        .filter(|endpoint| endpoint.chain_id.as_ref() == Some(chain_id))
        .map(|endpoint| endpoint.name.clone())
        .collect();
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer)),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

fn select_profile(config: &RpcConfig, prompt: &str) -> Result<Option<String>, MescCliError> {
    let mut options: Vec<_> = config.profiles.keys().collect();
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer.to_string())),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

fn select_config_chain_id(
    config: &RpcConfig,
    prompt: &str,
) -> Result<Option<ChainId>, MescCliError> {
    let mut chain_ids: HashSet<ChainId> = HashSet::new();
    for endpoint in config.endpoints.values() {
        if let Some(chain_id) = endpoint.chain_id.as_ref() {
            chain_ids.insert(chain_id.clone());
        }
    }
    let mut sorted_chain_ids: Vec<ChainId> = chain_ids.into_iter().collect();
    sorted_chain_ids.sort_by_key(|chain_id| chain_id.to_hex_256().unwrap_or("".to_string()));
    let options: Vec<String> = sorted_chain_ids
        .iter()
        .map(|chain_id| match mesc::directory::get_network_name(chain_id) {
            Some(name) => format!("{} ({})", chain_id.as_str(), name),
            None => chain_id.to_string(),
        })
        .collect();
    let input = match inquire::Select::new(prompt, options.clone()).prompt() {
        Ok(answer) => answer,
        Err(InquireError::OperationCanceled) => return Ok(None),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };
    match options.iter().position(|x| x == &input) {
        Some(index) => Ok(Some(sorted_chain_ids.remove(index))),
        None => Err(MescCliError::Error("invalid input".to_string())),
    }
}

async fn select_chain_id_by_name() -> Result<Option<ChainId>, MescCliError> {
    let network_list = match crate::network::fetch_network_list().await {
        Ok(mapping) => mapping,
        Err(_) => {
            println!(" could not retrieve network list");
            return Ok(None);
        }
    };
    let mut pairs: Vec<(ChainId, String)> = network_list.into_iter().collect();
    pairs.sort_by_key(|(chain_id, _)| {
        chain_id.to_hex_256().unwrap_or(chain_id.as_str().to_string())
    });

    let options: Vec<_> =
        pairs.iter().map(|(chain_id, name)| format!("{} ({})", chain_id, name)).collect();
    match inquire::Select::new("Which network?", options.clone()).prompt() {
        Ok(answer) => match options.iter().position(|option| option == &answer) {
            Some(index) => match pairs.get(index) {
                Some((chain_id, _)) => Ok(Some(chain_id.clone())),
                None => Ok(None),
            },
            None => Err(MescCliError::InvalidInput("bad input".to_string())),
        },
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Ok(None),
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
