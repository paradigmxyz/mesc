use crate::{MescCliError, MetadataArgs};

pub(crate) fn metadata_command(args: MetadataArgs) -> Result<(), MescCliError> {
    let config = mesc::load::load_config_data()?;
    match (args.endpoint, args.profile, args.profile_only) {
        (None, None, None) => {
            println!("{}", serde_json::to_string_pretty(&config.global_metadata)?);
            Ok(())
        }
        (Some(endpoint), None, None) => {
            if let Some(endpoint) = config.endpoints.get(endpoint.as_str()) {
                println!("{}", serde_json::to_string_pretty(&endpoint.endpoint_metadata)?);
                Ok(())
            } else {
                Err(MescCliError::Error("endpoint not found".to_string()))
            }
        }
        (None, Some(profile), None) => {
            let metadata = mesc::get_global_metadata(Some(profile.as_str()))?;
            println!("{}", serde_json::to_string_pretty(&metadata)?);
            Ok(())
        }
        (None, None, Some(profile)) => {
            if let Some(profile) = config.profiles.get(profile.as_str()) {
                println!("{}", serde_json::to_string_pretty(&profile.profile_metadata)?);
                Ok(())
            } else {
                Err(MescCliError::Error("profile not found".to_string()))
            }
        }
        _ => Err(MescCliError::Error("cannot specify more than one metadata type".to_string())),
    }
}
