use crate::{MescError, RpcConfig};
use std::fs::File;

pub fn write_config(config: RpcConfig, path: String) -> Result<(), MescError> {
    let file = File::create(path)?;
    Ok(serde_json::to_writer(file, &config)?)
}
