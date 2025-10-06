use super::math_utils::mod_pow;
use super::types::{RsaPublicKey, RsaPrivateKey};

pub fn encrypt(message: u64, public_key: &RsaPublicKey) -> u64 {
    mod_pow(message, public_key.e, public_key.n)
}

pub fn decrypt(ciphertext: u64, private_key: &RsaPrivateKey) -> u64 {
    mod_pow(ciphertext, private_key.d, private_key.n)
}
