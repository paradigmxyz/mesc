use crate::{print_endpoints, LsArgs, MescCliError};

pub(crate) fn ls_command(args: LsArgs) -> Result<(), MescCliError> {
    let config = mesc::load::load_config_data()?;
    let reveal = if args.reveal {
        true
    } else {
        config.global_metadata.get("reveal") == Some(&serde_json::Value::Bool(true))
    };
    if args.json {
        println!("{}", serde_json::to_string_pretty(&config.endpoints)?);
        Ok(())
    } else {
        print_endpoints(&config, reveal)
    }
}
