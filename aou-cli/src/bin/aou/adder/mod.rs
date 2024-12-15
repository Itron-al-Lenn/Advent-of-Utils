mod input_control;

use aou_cli::error::{AocError, InputError};

use crate::config::AddConfig;

pub fn run(config: &AddConfig) -> Result<(), AocError> {
    // Open the input file in the editor
    if let Err(error) = input_control::edit_input(config.year, config.day, &config.database) {
        return Err(AocError::Input(InputError::TestInputFailed {
            source: Some(error),
        }));
    };

    // Get the results
    if let Err(error) = input_control::edit_input(config.year, config.day, &config.database) {
        return Err(AocError::Input(InputError::TestInputFailed {
            source: Some(error),
        }));
    };

    Ok(())
}
