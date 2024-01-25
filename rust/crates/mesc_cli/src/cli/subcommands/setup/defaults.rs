use super::selectors::*;
use crate::{print_defaults, MescCliError};
use inquire::InquireError;
use mesc::RpcConfig;

pub(crate) fn modify_defaults(config: &mut RpcConfig) -> Result<(), MescCliError> {
    println!(" Current defaults:");
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
        match inquire::Select::new("Which defaults do you want to modify?", options.clone())
            .prompt()
        {
            Ok("Set the default endpoint") => {
                let prompt = "Which endpoint should be the default?";
                let endpoint_name = match select_endpoint(config, prompt)? {
                    Some(value) => value,
                    _ => return Ok(()),
                };
                config.default_endpoint = Some(endpoint_name.clone());
                if let Some(endpoint) =
                    mesc::query::get_endpoint_by_name(config, endpoint_name.as_str())?
                {
                    if let Some(chain_id) = endpoint.chain_id {
                        config.network_defaults.insert(chain_id, endpoint_name);
                    };
                }
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
                        if let Some(endpoint) =
                            mesc::query::get_endpoint_by_name(config, &default_endpoint)?
                        {
                            if let Some(profile) = config.profiles.get_mut(&profile_name) {
                                profile.default_endpoint = Some(default_endpoint.clone());
                                if let Some(chain_id) = endpoint.chain_id.clone() {
                                    profile.network_defaults.insert(chain_id, default_endpoint);
                                }
                            } else {
                                println!("profile not present");
                            }
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
                println!("Current defaults:");
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
