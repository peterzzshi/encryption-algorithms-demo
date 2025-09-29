use num_bigint::BigUint;
use crate::rsa::key_generation::*;
use crate::rsa::math_utils::*;

pub fn string_to_bigint(message: &str) -> BigUint {
    let bytes = message.as_bytes();
    BigUint::from_bytes_be(bytes)
}

pub fn bigint_to_string(value: &BigUint) -> String {
    let bytes = value.to_bytes_be();
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn encrypt(message: &BigUint, public_key: &RsaPublicKey) -> BigUint {
    mod_pow(message, &public_key.e, &public_key.n)
}

pub fn decrypt(ciphertext: &BigUint, private_key: &RsaPrivateKey) -> BigUint {
    mod_pow(ciphertext, &private_key.d, &private_key.n)
}

pub fn print_encryption_steps(message: &str, message_int: &BigUint, ciphertext: &BigUint, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Original message", message);
    print_step("Message as integer", &message_int.to_string());
    print_step("Encryption formula", "c = m^e mod n");
    print_step(&format!("c = {}^{} mod {}", message_int, public_key.e, public_key.n), "");
    print_step("Ciphertext", &ciphertext.to_string());
}

pub fn print_decryption_steps(ciphertext: &BigUint, decrypted_int: &BigUint, decrypted_message: &str, private_key: &RsaPrivateKey) {
    print_header("RSA Decryption");
    print_step("Ciphertext", &ciphertext.to_string());
    print_step("Decryption formula", "m = c^d mod n");
    print_step(&format!("m = {}^{} mod {}", ciphertext, private_key.d, private_key.n), "");
    print_step("Decrypted integer", &decrypted_int.to_string());
    print_step("Decrypted message", decrypted_message);
}