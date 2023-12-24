use crate::{print_defaults, DefaultsArgs, MescCliError};

pub(crate) fn defaults_command(_args: DefaultsArgs) -> Result<(), MescCliError> {
    print_defaults(&mesc::load::load_config_data()?)
}
