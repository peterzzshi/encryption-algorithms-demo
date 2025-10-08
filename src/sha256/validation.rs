pub fn validate_message(message: &[u8]) -> Result<(), String> {
    if message.is_empty() {
        return Err("Message cannot be empty".to_string());
    }

    if message.len() > 1000 {
        return Err("Message too long for demo (max 1000 bytes)".to_string());
    }

    Ok(())
}

pub fn is_printable_ascii(bytes: &[u8]) -> bool {
    bytes.iter().all(|&b| b >= 32 && b <= 126)
}