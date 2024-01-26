use crate::{ImportArgs, MescCliError};
use mesc::RpcConfig;

#[allow(unreachable_code)]
#[allow(unused_variables)]
pub(crate) async fn import_command(args: ImportArgs) -> Result<(), MescCliError> {
    println!("import command under construction, try again later");
    return Ok(());

    // get output path
    let output_path = match (args.output_path, std::env::var("MESC_PATH")) {
        (Some(path), _) => path,
        (_, Ok(path)) if !path.is_empty() => path,
        _ => {
            return Err(MescCliError::Error(
                "MESC_PATH not set, must specify --output-path".to_string(),
            ));
        }
    };

    // gather existing import sources
    if !mesc::is_mesc_enabled() {
        println!("MESC is not enabled, but importing anyway...")
    };

    // fetch config data
    let imported = match args.source.as_deref() {
        Some("chainid.network") => import_chain_id_dot_network().await?,
        Some("chainlist") => import_chainlist().await?,
        Some(source) => import_custom_source(source).await?,
        None => return Err(MescCliError::InvalidInput("must specify import source".to_string())),
    };

    // integrate data into config
    let new_config = integrate_import(imported)?;

    // write new config data
    mesc::write::write_config(new_config, output_path)?;

    Ok(())
}

async fn import_chain_id_dot_network() -> Result<RpcConfig, MescCliError> {
    todo!();
}

async fn import_chainlist() -> Result<RpcConfig, MescCliError> {
    todo!();
}

async fn import_custom_source(source: &str) -> Result<RpcConfig, MescCliError> {
    let path = std::path::Path::new(source);
    if path.exists() && path.is_file() {
        println!("Importing file...");
        let config = mesc::load::load_file_config(Some(source.to_string()))?;
        Ok(config)
    } else {
        println!("Importing url...");
        let response = reqwest::get(source).await?;
        if response.status().is_success() {
            let config: RpcConfig = response.json().await?;
            Ok(config)
        } else {
            Err(MescCliError::Error("Could not import MESC config from source".to_string()))
        }
    }
}

fn integrate_import(imported: RpcConfig) -> Result<RpcConfig, MescCliError> {
    Ok(imported)
}
