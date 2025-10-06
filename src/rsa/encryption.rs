use crate::rsa::key_generation::*;
use crate::rsa::math_utils::*;

pub fn encrypt(message: u64, public_key: &RsaPublicKey) -> u64 {
    mod_pow(message, public_key.e, public_key.n)
}

pub fn decrypt(ciphertext: u64, private_key: &RsaPrivateKey) -> u64 {
    mod_pow(ciphertext, private_key.d, private_key.n)
}

pub fn text_to_number(text: &str) -> Option<u64> {
    if text.is_empty() {
        return None;
    }

    // Convert string to bytes and then to a number
    let bytes = text.as_bytes();

    // Limit to 8 bytes to fit in u64
    if bytes.len() > 8 {
        return None;
    }

    let mut number: u64 = 0;
    for &byte in bytes {
        number = (number << 8) | (byte as u64);
    }

    Some(number)
}

/// Convert a number back to a string
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
