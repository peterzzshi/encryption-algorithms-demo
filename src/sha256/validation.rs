use crate::common::validation::{validate_non_empty_message, validate_message_length};

pub fn validate_message(message: &[u8]) -> Result<(), String> {
    validate_non_empty_message(message)?;
    validate_message_length(message, 1000)?;
    Ok(())
}
