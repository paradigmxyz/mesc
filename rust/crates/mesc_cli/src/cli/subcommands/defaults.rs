use crate::{print_defaults, DefaultsArgs, MescCliError};

pub(crate) fn defaults_command(_args: DefaultsArgs) -> Result<(), MescCliError> {
    let config = mesc::load::load_config_data()?;
    print_defaults(&config)
}
