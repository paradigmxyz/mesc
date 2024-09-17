use crate::MescCliError;
use inquire::{ui::IndexPrefix, InquireError};
use mesc::{ChainId, RpcConfig, TryIntoChainId};
use std::collections::HashSet;
use toolstr::Colorize;

use super::{inquire_utils::*, writing::*};

pub(crate) async fn select_chain_id(prompt: String) -> Result<Option<ChainId>, MescCliError> {
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
                    _ => {
                        println!(" Exiting without saving");
                        std::process::exit(0);
                    }
                }
            }
            Err(InquireError::OperationCanceled) => return Ok(None),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    }
}

pub(crate) async fn select_chain_id_by_name() -> Result<Option<ChainId>, MescCliError> {
    let network_list = match crate::network::fetch_network_list().await {
        Ok(mapping) => mapping,
        Err(_) => {
            println!(" could not retrieve network list");
            return Ok(None);
        }
    };
    let mut pairs: Vec<(ChainId, String)> = network_list.into_iter().collect();
    pairs.sort_by(|(a, _), (b, _)| a.cmp(b));

    let options: Vec<_> =
        pairs.iter().map(|(chain_id, name)| format!("{}) {}", chain_id, name)).collect();
    let mut render_config = get_render_config();
    render_config.option_index_prefix = IndexPrefix::None;
    match inquire::Select::new("Which network?", options.clone())
        .with_render_config(render_config)
        .prompt()
    {
        Ok(answer) => match options.iter().position(|option| option == &answer) {
            Some(index) => match pairs.get(index) {
                Some((chain_id, _)) => Ok(Some(chain_id.clone())),
                None => Ok(None),
            },
            None => Err(MescCliError::InvalidInput("bad input".to_string())),
        },
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::Error("invalid input".to_string())),
    }
}

pub(crate) fn select_custom_network_name(
    config: &RpcConfig,
) -> Result<Option<String>, MescCliError> {
    let names: Vec<_> = config.network_names.keys().collect();
    if names.is_empty() {
        return Ok(None);
    }
    match inquire::Select::new("Which custom name?", names).prompt() {
        Ok(name) => Ok(Some(name.clone())),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

pub(crate) fn select_endpoint(
    config: &RpcConfig,
    prompt: &str,
) -> Result<Option<String>, MescCliError> {
    let mut options: Vec<String> = config.endpoints.clone().into_keys().collect();
    if options.is_empty() {
        return Ok(None);
    }
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer)),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

pub(crate) fn select_endpoint_of_network(
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
    if options.is_empty() {
        return Ok(None);
    }
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer)),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

pub(crate) fn select_profile(
    config: &RpcConfig,
    prompt: &str,
) -> Result<Option<String>, MescCliError> {
    let mut options: Vec<_> = config.profiles.keys().collect();
    if options.is_empty() {
        return Ok(None);
    }
    options.sort();
    match inquire::Select::new(prompt, options).prompt() {
        Ok(answer) => Ok(Some(answer.to_string())),
        Err(InquireError::OperationCanceled) => Ok(None),
        _ => Err(MescCliError::InvalidInput("invalid input".to_string())),
    }
}

pub(crate) fn select_config_chain_id(
    config: &RpcConfig,
    prompt: &str,
) -> Result<Option<ChainId>, MescCliError> {
    let mut chain_ids = config
        .endpoints
        .values()
        .filter_map(|endpoint| endpoint.chain_id.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    chain_ids.sort();
    let options: Vec<String> = chain_ids
        .iter()
        .map(|chain_id| match mesc::directory::get_network_name(chain_id) {
            Some(name) => format!("{chain_id}) {name}"),
            None => chain_id.to_string(),
        })
        .collect();
    let input = match inquire::Select::new(prompt, options.clone()).prompt() {
        Ok(answer) => answer,
        Err(InquireError::OperationCanceled) => return Ok(None),
        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
    };
    match options.iter().position(|x| x == &input) {
        Some(index) => Ok(Some(chain_ids[index].clone())),
        None => Err(MescCliError::Error("invalid input".to_string())),
    }
}

pub(crate) fn select_config_mode() -> Result<ConfigWriteMode, MescCliError> {
    let prompt = "Do you want to store your MESC config in a file or in an env var?";
    let options = vec!["File (recommended)", "Environment Variable"];
    match inquire::Select::new(prompt, options).prompt() {
        Ok("File (recommended)") => Ok(ConfigWriteMode::Path(select_config_path()?)),
        Ok("Environment Variable") => Ok(ConfigWriteMode::Env(vec![])),
        Ok(_) | Err(_) => {
            eprintln!("Aborting setup");
            std::process::exit(1);
        }
    }
}

pub(crate) fn select_config_path() -> Result<std::path::PathBuf, MescCliError> {
    loop {
        let prompt = "Where to save MESC config file?";
        match inquire::Text::new(prompt).with_default("~/mesc.json").prompt() {
            Ok(path) => {
                let path = mesc::load::expand_path(path)?;
                let path = std::path::Path::new(path.as_str());
                if path.exists() {
                    if path.is_file() {
                        match path.extension().map(|ext| ext.to_string_lossy()) {
                            Some(ext) if ext == "json" => return Ok(path.to_path_buf()),
                            _ => {
                                let prompt = "Use file even without a .json file extension?";
                                if let Ok(true) =
                                    inquire::Confirm::new(prompt).with_default(false).prompt()
                                {
                                    return Ok(path.to_path_buf());
                                }
                            }
                        }
                    } else if path.is_dir() {
                        let path = path.join("mesc.json");
                        println!("Treating as a directory, will save to {}", path.display());
                        return Ok(path);
                    } else {
                        println!("Not a valid file path");
                    }
                } else {
                    match path.extension() {
                        Some(ext) if ext == "json" => return Ok(path.to_path_buf()),
                        Some(_) => {
                            let prompt = "Use file even without a .json file extension?";
                            if let Ok(true) =
                                inquire::Confirm::new(prompt).with_default(false).prompt()
                            {
                                return Ok(path.to_path_buf());
                            }
                        }
                        None => {
                            let path = path.join("mesc.json");
                            println!("Treating as a directory, will save to {}", path.display());
                            return Ok(path);
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("Aborting setup");
                std::process::exit(1);
            }
        }
    }
}
