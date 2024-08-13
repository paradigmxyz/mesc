use crate::MescError;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// get api key
pub fn get_api_key<T: AsRef<str>>(
    key_name: T,
    profile_name: Option<T>,
) -> Result<Option<String>, MescError> {
    let key_name = key_name.as_ref();
    let config = crate::load::load_config_data()?;

    if let Some(profile_name) = profile_name {
        if let Some(profile) = config.profiles.get(profile_name.as_ref()) {
            if let Some(api_keys) = profile.profile_metadata.get("api_keys") {
                let api_keys: HashMap<String, String> = get_value_at(api_keys, &[])?;
                if let Some(key) = api_keys.get(key_name) {
                    return Ok(Some(key.clone()));
                }
            }
        }
    };

    if let Some(api_keys) = config.global_metadata.get("api_keys") {
        let api_keys: HashMap<String, String> = get_value_at(api_keys, &[])?;
        if let Some(key) = api_keys.get(key_name) {
            return Ok(Some(key.clone()));
        }
    }

    Ok(None)
}

fn get_value_at<T>(root: &serde_json::Value, path: &[&str]) -> Result<T, MescError>
where
    T: DeserializeOwned,
{
    let mut current = root;

    for key in path {
        current = match current.get(key) {
            Some(value) => value,
            None => return Err(MescError::IntegrityError("missing path".to_string())),
        };
    }

    Ok(serde_json::from_value(current.clone())?)
}
