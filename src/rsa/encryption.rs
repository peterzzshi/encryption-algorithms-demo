use num_bigint::BigUint;
use crate::rsa::key_generation::*;
use crate::rsa::math_utils::*;

pub fn encrypt(message: &BigUint, public_key: &RsaPublicKey) -> BigUint {
    mod_pow(message, &public_key.e, &public_key.n)
}

pub fn decrypt(ciphertext: &BigUint, private_key: &RsaPrivateKey) -> BigUint {
    mod_pow(ciphertext, &private_key.d, &private_key.n)
}
