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

fn truncate_large_number(num: &BigUint, max_digits: usize) -> String {
    let num_str = num.to_string();
    if num_str.len() <= max_digits {
        num_str
    } else {
        format!("{}...{} ({} digits)", 
                &num_str[..10], 
                &num_str[num_str.len()-10..], 
                num_str.len())
    }
}

pub fn print_encryption_steps(message: &str, message_int: &BigUint, ciphertext: &BigUint, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Original message", message);
    print_step("Message as integer", &truncate_large_number(message_int, 50));
    print_step("Public key (n)", &truncate_large_number(&public_key.n, 50));
    print_step("Public key (e)", &public_key.e.to_string());
    print_step("Encryption formula", "c = m^e mod n");
    print_step("Ciphertext", &truncate_large_number(ciphertext, 50));
}

pub fn print_decryption_steps(ciphertext: &BigUint, decrypted_int: &BigUint, decrypted_message: &str, private_key: &RsaPrivateKey) {
    print_header("RSA Decryption");
    print_step("Ciphertext", &truncate_large_number(ciphertext, 50));
    print_step("Private key (d)", &truncate_large_number(&private_key.d, 50));
    print_step("Decryption formula", "m = c^d mod n");
    print_step("Decrypted integer", &truncate_large_number(decrypted_int, 50));
    print_step("Decrypted message", decrypted_message);
}