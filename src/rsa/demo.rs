use num_bigint::BigUint;
use num_traits::One;
use crate::rsa::key_generation::*;
use crate::rsa::encryption::*;
use crate::rsa::math_utils::*;

pub fn run_rsa_demo(message: &str, key_size: usize) {
    println!("🔐 RSA Encryption Algorithm Demo");
    println!("Message: \"{}\"", message);
    println!("Key Size: {} bits", key_size);

    // Generate key pair
    let p = generate_prime(key_size / 2);
    let q = generate_prime(key_size / 2);
    let n = &p * &q;
    let phi_n = (&p - BigUint::one()) * (&q - BigUint::one());
    let e = BigUint::from(65537u32);
    let d = mod_inverse(&e, &phi_n).expect("Failed to compute modular inverse");

    let key_pair = RsaKeyPair {
        public_key: RsaPublicKey { n: n.clone(), e },
        private_key: RsaPrivateKey { n, d },
    };

    // Print key generation steps
    print_key_generation_steps(&key_pair, &p, &q, &phi_n);

    // Convert message to integer
    let message_int = string_to_bigint(message);

    // Encrypt
    let ciphertext = encrypt(&message_int, &key_pair.public_key);
    print_encryption_steps(message, &message_int, &ciphertext, &key_pair.public_key);

    // Decrypt
    let decrypted_int = decrypt(&ciphertext, &key_pair.private_key);
    let decrypted_message = bigint_to_string(&decrypted_int);
    print_decryption_steps(&ciphertext, &decrypted_int, &decrypted_message, &key_pair.private_key);

    // Verification
    print_header("Verification");
    let success = message == decrypted_message;
    print_step("Original == Decrypted", &success.to_string());

    if success {
        println!("\n✅ RSA encryption/decryption successful!");
    } else {
        println!("\n❌ RSA encryption/decryption failed!");
    }
}