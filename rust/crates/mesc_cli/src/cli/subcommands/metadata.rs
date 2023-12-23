use crate::{MescCliError, MetadataArgs};

pub(crate) fn metadata_command(_args: MetadataArgs) -> Result<(), MescCliError> {
    let config = mesc::load::load_config_data()?;
    println!("{}", serde_json::to_string_pretty(&config.global_metadata)?);
    Ok(())
}
