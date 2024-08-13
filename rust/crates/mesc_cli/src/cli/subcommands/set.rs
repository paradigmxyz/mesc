use crate::{MescCliError, SetArgs};
use mesc::{RpcConfig, TryIntoChainId};
use serde_json::{Map, Value};
use std::collections::HashMap;
use toolstr::Colorize;

pub(crate) async fn set_command(args: SetArgs) -> Result<(), MescCliError> {
    // load old config data
    let path = match (args.path.clone(), mesc::load::get_config_mode()) {
        (Some(path), _) => path,
        (_, Ok(mesc::ConfigMode::Path)) => mesc::load::get_config_path()?,
        _ => {
            eprintln!("to use set, must be in MESC_MODE=PATH or use the --path argument");
            std::process::exit(1);
        }
    };
    let mut config = match mesc::load::load_file_config(Some(path.clone())) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("could not load MESC config: {}", e);
            std::process::exit(1);
        }
    };
    let old_config = config.clone();

    // decide what to edit
    let kv_pairs = determine_kv_pairs(&args)?;
    let n_kv_pairs = kv_pairs.len();

    // edit
    for (key, value) in kv_pairs.into_iter() {
        if let Some(value) = value {
            set_config_value(&mut config, key, value)?
        } else {
            delete_config_value(&mut config, key)?
        }
    }

    // check that new config is valid
    if let Err(e) = config.validate() {
        eprintln!("Aborting because these updated resulted in an invalid config: {}", e);
        std::process::exit(1);
    }

    // write new file
    if config != old_config {
        mesc::write::write_config(config, path)?;
        if n_kv_pairs == 1 {
            println!("Applied {} update to config", n_kv_pairs.to_string().green().bold());
        } else {
            println!("Applied {} updates to config", n_kv_pairs.to_string().green().bold());
        }
    } else {
        println!("Updates did not change the content of the config");
    }

    Ok(())
}

fn determine_kv_pairs(args: &SetArgs) -> Result<Vec<(String, Option<String>)>, MescCliError> {
    let kv_pairs = match (
        args.key.clone(),
        args.value.clone(),
        args.keys.clone(),
        args.values.clone(),
        args.delete,
    ) {
        (None, _, None, _, _) => {
            return Err(MescCliError::InvalidInput("specify one of key or --keys".to_string()))
        }
        (Some(_), _, Some(_), _, _) => {
            return Err(MescCliError::InvalidInput(
                "cannot specify both key and --keys".to_string(),
            ))
        }
        (_, None, _, None, false) => {
            return Err(MescCliError::InvalidInput("specify value or --values".to_string()))
        }
        (_, Some(_), _, Some(_), _) => {
            return Err(MescCliError::InvalidInput(
                "cannot specify both value and --values".to_string(),
            ))
        }
        (_, Some(_), _, _, true) | (_, _, _, Some(_), true) => {
            return Err(MescCliError::InvalidInput(
                "cannot specify value or --values when using --delete".to_string(),
            ))
        }
        (_, _, None, Some(_), false) => {
            return Err(MescCliError::InvalidInput(
                "must specify --keys when using --values".to_string(),
            ))
        }
        (Some(key), Some(value), None, None, false) => vec![(key, Some(value))],
        (Some(key), None, None, None, true) => vec![(key, None)],
        (None, None, Some(keys), Some(values), false) => {
            keys.into_iter().zip(values.into_iter().map(Some)).collect()
        }
        (None, None, Some(keys), None, true) => keys.into_iter().map(|k| (k, None)).collect(),
        (None, Some(value), Some(keys), None, false) => {
            keys.into_iter().map(|k| (k, Some(value.clone()))).collect()
        }
    };
    Ok(kv_pairs)
}

fn set_config_value(
    config: &mut RpcConfig,
    key: String,
    value: String,
) -> Result<(), MescCliError> {
    let parts = key.split("\\.").flat_map(|part| part.split('.')).collect::<Vec<_>>();
    match parts.as_slice() {
        ["mesc_version"] => config.mesc_version = value,
        ["default_endpoint"] => config.default_endpoint = Some(value),
        ["endpoints", endpoint_name] => {
            config
                .endpoints
                .insert(endpoint_name.to_string(), serde_json::from_str(value.as_str())?);
        }
        ["endpoints", endpoint_name, rest @ ..] => {
            let rest = rest.to_vec();
            if (rest.len() == 1) & (rest[0] == "name") {
                mesc::write::update_endpoint_name(config, endpoint_name, value.as_str())?;
                return Ok(());
            }
            if let Some(endpoint) = config.endpoints.get_mut(*endpoint_name) {
                match rest.as_slice() {
                    ["url"] => endpoint.url = value,
                    ["chain_id"] => endpoint.chain_id = Some(value.try_into_chain_id()?),
                    ["endpoint_metadata", location @ ..] => {
                        set_metadata_entry(&mut endpoint.endpoint_metadata, location, value)?;
                    }
                    _ => {
                        return Err(MescCliError::InvalidInput(format!("cannot set key: {}", key)))
                    }
                }
            };
        }
        ["network_defaults", chain_id] => {
            config.network_defaults.insert(chain_id.try_into_chain_id()?, value);
        }
        ["network_names", network_name] => {
            config.network_names.insert(network_name.to_string(), value.try_into_chain_id()?);
        }
        ["profiles", profile_name] => {
            config.profiles.insert(profile_name.to_string(), serde_json::from_str(value.as_str())?);
        }
        ["profiles", profile_name, rest @ ..] => {
            let rest = rest.to_vec();
            if let Some(profile) = config.profiles.get_mut(*profile_name) {
                match rest.as_slice() {
                    ["name"] => profile.name = value,
                    ["default_endpoint"] => profile.default_endpoint = Some(value),
                    ["network_defaults", chain_id] => {
                        profile.network_defaults.insert(chain_id.try_into_chain_id()?, value);
                    }
                    ["profile_metadata", location @ ..] => {
                        set_metadata_entry(&mut profile.profile_metadata, location, value)?;
                    }
                    ["use_mesc"] => match value.parse::<bool>() {
                        Ok(as_bool) => profile.use_mesc = as_bool,
                        Err(_) => {
                            return Err(MescCliError::InvalidInput(
                                "use_mesc must be a bool value".to_string(),
                            ))
                        }
                    },
                    _ => {
                        return Err(MescCliError::InvalidInput(format!("cannot set key: {}", key)))
                    }
                }
            }
        }
        ["global_metadata", location @ ..] => {
            set_metadata_entry(&mut config.global_metadata, location, value)?;
        }
        _ => return Err(MescCliError::InvalidInput(format!("cannot delete key: {}", key))),
    };
    Ok(())
}

fn delete_config_value(config: &mut RpcConfig, key: String) -> Result<(), MescCliError> {
    let parts = key.split("\\.").flat_map(|part| part.split('.')).collect::<Vec<_>>();
    match parts.as_slice() {
        ["mesc_version"] => {
            return Err(MescCliError::InvalidInput("cannot delete mesc_version".to_string()))
        }
        ["default_endpoint"] => config.default_endpoint = None,
        ["endpoints", endpoint_name] => {
            config.endpoints.remove(endpoint_name.to_string().as_str());
        }
        ["endpoints", endpoint_name, rest @ ..] => {
            let rest = rest.to_vec();
            if let Some(endpoint) = config.endpoints.get_mut(*endpoint_name) {
                match rest.as_slice() {
                    ["name"] => {
                        return Err(MescCliError::InvalidInput(
                            "cannot delete endpoint name".to_string(),
                        ))
                    }
                    ["url"] => {
                        return Err(MescCliError::InvalidInput(
                            "cannot delete endpoint url".to_string(),
                        ))
                    }
                    ["chain_id"] => {
                        endpoint.chain_id = None;
                    }
                    ["endpoint_metadata", location @ ..] => {
                        delete_metadata_entry(&mut endpoint.endpoint_metadata, location)?;
                    }
                    _ => {
                        return Err(MescCliError::InvalidInput(format!(
                            "cannot delete key: {}",
                            key
                        )))
                    }
                }
            }
        }
        ["network_defaults", chain_id] => {
            config.network_defaults.remove(&chain_id.try_into_chain_id()?);
        }
        ["network_names", network_name] => {
            config.network_names.remove(*network_name);
        }
        ["profiles", profile_name] => {
            config.profiles.remove(*profile_name);
        }
        ["profiles", profile_name, rest @ ..] => {
            let rest = rest.to_vec();
            if let Some(profile) = config.profiles.get_mut(*profile_name) {
                match rest.as_slice() {
                    ["name"] => {
                        return Err(MescCliError::InvalidInput(
                            "cannot delete profile name".to_string(),
                        ))
                    }
                    ["default_endpoint"] => profile.default_endpoint = None,
                    ["network_defaults", chain_id] => {
                        profile.network_defaults.remove(&chain_id.try_into_chain_id()?);
                    }
                    ["profile_metadata", location @ ..] => {
                        delete_metadata_entry(&mut profile.profile_metadata, location)?;
                    }
                    ["use_mesc"] => {
                        return Err(MescCliError::InvalidInput(
                            "cannot delete profile use_mesc".to_string(),
                        ))
                    }
                    _ => {
                        return Err(MescCliError::InvalidInput(format!(
                            "cannot delete key: {}",
                            key
                        )))
                    }
                }
            }
        }
        ["global_metadata", location @ ..] => {
            delete_metadata_entry(&mut config.global_metadata, location)?;
        }
        _ => return Err(MescCliError::InvalidInput(format!("cannot delete key: {}", key))),
    };
    Ok(())
}

fn set_metadata_entry(
    metadata: &mut HashMap<String, Value>,
    location: &[&str],
    json_content: String,
) -> Result<(), MescCliError> {
    if location.is_empty() {
        return Err(MescCliError::InvalidInput("must specify metadata path to set".to_string()));
    }

    let parsed_value = serde_json::from_str(&json_content)
        .map_err(|e| MescCliError::InvalidInput(format!("Failed to parse JSON content: {}", e)))?;

    if location.len() == 1 {
        metadata.insert(location[0].to_string(), parsed_value);
        return Ok(());
    }

    let mut current_value =
        metadata.entry(location[0].to_string()).or_insert_with(|| Value::Object(Map::new()));

    for &key in &location[1..location.len() - 1] {
        current_value = match current_value {
            Value::Object(map) => {
                map.entry(key.to_string()).or_insert_with(|| Value::Object(Map::new()))
            }
            _ => return Err(MescCliError::InvalidInput(format!("Invalid path: {}", key))),
        };
    }

    if let Some(last_key) = location.last() {
        if let Value::Object(map) = current_value {
            map.insert(last_key.to_string(), parsed_value);
        } else {
            return Err(MescCliError::InvalidInput("Target is not an object".to_string()));
        }
    }

    Ok(())
}

fn delete_metadata_entry(
    metadata: &mut HashMap<String, Value>,
    location: &[&str],
) -> Result<(), MescCliError> {
    if location.is_empty() {
        return Err(MescCliError::InvalidInput("must specify metadata path to delete".to_string()));
    }

    if location.len() == 1 {
        metadata.remove(location[0]);
        return Ok(());
    }

    let mut current_value = match metadata.get_mut(location[0]) {
        Some(value) => value,
        None => return Ok(()),
    };

    for &key in &location[1..location.len() - 1] {
        current_value = match current_value {
            Value::Null => return Ok(()),
            Value::Object(map) => match map.get_mut(key) {
                Some(value) => value,
                None => return Ok(()),
            },
            _ => return Err(MescCliError::InvalidInput(format!("Invalid path: {}", key))),
        };
    }

    if let Some(last_key) = location.last() {
        if let Value::Object(map) = current_value {
            map.remove(*last_key);
        } else {
            return Err(MescCliError::InvalidInput("Target is not an object".to_string()));
        }
    }

    Ok(())
}
