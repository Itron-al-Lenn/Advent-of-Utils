mod input_control;

use advent_of_utils::error::{AocError, InputError};

use crate::config::AddConfig;

pub fn run(config: &AddConfig) -> Result<(), AocError> {
    // Open the input file in the editor
    match input_control::edit_input(config.year, config.day, &config.database) {
        Err(error) => Err(AocError::Input(InputError::TestInputFailed {
            source: Some(error),
        })),
        Ok(_) => Ok(()),
    }
}
