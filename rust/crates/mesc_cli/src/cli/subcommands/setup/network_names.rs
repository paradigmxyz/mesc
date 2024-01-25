use crate::MescCliError;
use inquire::InquireError;
use mesc::{RpcConfig, TryIntoChainId};
use toolstr::Colorize;

use super::selectors::*;

pub(crate) async fn modify_custom_network_names(
    config: &mut RpcConfig,
) -> Result<(), MescCliError> {
    print_custom_network_names(config);

    loop {
        let options = if config.network_names.is_empty() {
            vec![
                "Add custom network name",
                "Print custom network names",
                "Done editing custom names",
            ]
        } else {
            vec![
                "Add custom network name",
                "Edit custom network name",
                "Remove custom network name",
                "Print custom network names",
                "Done editing custom names",
            ]
        };
        match inquire::Select::new("What do you want to do?", options).prompt() {
            Ok("Add custom network name") => {
                let name = match inquire::Text::new("Custom network name?").prompt() {
                    Ok(name) => name,
                    Err(InquireError::OperationCanceled) => continue,
                    _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                };
                loop {
                    match inquire::Text::new("What is the chain id?").prompt() {
                        Ok(chain_id) => match chain_id.try_into_chain_id() {
                            Ok(chain_id) => {
                                config.network_names.insert(name.clone(), chain_id);
                                println!(" Custom network name added");
                                break;
                            }
                            _ => {
                                println!(" not a valid chain id");
                                continue;
                            }
                        },
                        Err(InquireError::OperationCanceled) => break,
                        _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                    }
                }
            }
            Ok("Edit custom network name") => match select_custom_network_name(config) {
                Ok(Some(old_name)) => match inquire::Text::new("New name?").prompt() {
                    Ok(new_name) => match config.network_names.remove(&old_name) {
                        Some(chain_id) => {
                            config.network_names.insert(new_name, chain_id);
                        }
                        None => continue,
                    },
                    Err(InquireError::OperationCanceled) => return Ok(()),
                    _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
                },
                Ok(None) => return Ok(()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            },
            Ok("Remove custom network name") => match select_custom_network_name(config) {
                Ok(Some(name)) => {
                    config.network_names.remove(&name);
                    println!(" Name removed");
                }
                Ok(None) => return Ok(()),
                _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
            },
            Ok("Print custom network names") => print_custom_network_names(config),
            Ok("Done editing custom names") => return Ok(()),
            Err(InquireError::OperationCanceled) => return Ok(()),
            _ => return Err(MescCliError::InvalidInput("invalid input".to_string())),
        }
    }
}

fn print_custom_network_names(config: &RpcConfig) {
    if config.network_names.is_empty() {
        println!(" No custom network names currently being used");
    } else {
        println!(
            " {} custom network names currently being used:",
            config.network_names.len().to_string().green().bold()
        );
        for (network_name, chain_id) in config.network_names.iter() {
            println!("    - {} = {}", network_name.bold(), chain_id.as_str().green().bold());
        }
    };
}
