use super::constants::MAX_TEXT_LENGTH;

/// Convert text to a number for RSA encryption
/// Returns None if text is empty or longer than MAX_TEXT_LENGTH
pub fn text_to_number(text: &str) -> Option<u64> {
    if text.is_empty() {
        return None;
    }

    let bytes = text.as_bytes();

    if bytes.len() > MAX_TEXT_LENGTH {
        return None;
    }

    let mut number: u64 = 0;
    for &byte in bytes {
        number = (number << 8) | (byte as u64);
    }

    Some(number)
}

/// Convert a number back to text
/// Reverses the text_to_number conversion
pub fn number_to_text(number: u64, length: usize) -> String {
    let mut bytes = Vec::new();
    let mut num = number;

    for _ in 0..length {
        bytes.push((num & 0xFF) as u8);
        num >>= 8;
    }

    bytes.reverse();
    String::from_utf8_lossy(&bytes).to_string()
}
