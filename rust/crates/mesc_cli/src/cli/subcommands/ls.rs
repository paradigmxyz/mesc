use crate::{print_endpoints, LsArgs, MescCliError};

pub(crate) fn ls_command(args: LsArgs) -> Result<(), MescCliError> {
    // get endpoints
    let mut query = mesc::EndpointQuery::new();
    if let Some(chain_id) = args.chain_id {
        query = query.chain_id(chain_id)?;
    }
    if let Some(name) = args.name {
        query = query.name(name)?;
    }
    if let Some(url) = args.url {
        query = query.url(url)?;
    }
    let endpoints = mesc::find_endpoints(query)?;

    let config = mesc::load::load_config_data()?;
    let reveal = if args.reveal {
        true
    } else {
        config.global_metadata.get("reveal") == Some(&serde_json::Value::Bool(true))
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&endpoints)?);
        Ok(())
    } else {
        print_endpoints(&endpoints, reveal)
    }
}
