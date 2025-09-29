use num_bigint::BigUint;
use num_traits::One;
use crate::rsa::key_generation::*;
use crate::rsa::encryption::*;
use crate::rsa::math_utils::*;

pub fn run_rsa_demo(message: &str, key_size: usize) {
    println!("🔐 RSA Encryption Algorithm Demo");
    println!("Message: \"{}\"", message);
    println!("Key Size: {} bits", key_size);

    // Validate inputs
    if key_size < 64 {
        eprintln!("❌ Error: Key size must be at least 64 bits");
        return;
    }

    if message.is_empty() {
        eprintln!("❌ Error: Message cannot be empty");
        return;
    }

    // Check if message is too large for the key size
    let message_int = string_to_bigint(message);
    let max_bits = key_size - 11; // PKCS#1 padding requires at least 11 bytes overhead

    if message_int.bits() as usize > max_bits {
        eprintln!("❌ Error: Message too large for key size. Message requires {} bits, but maximum for {}-bit key is {} bits",
                 message_int.bits(), key_size, max_bits);
        eprintln!("💡 Tip: Use a larger key size or shorter message");
        return;
    }

    // Generate key pair
    println!("\n⏳ Generating RSA key pair...");
    let p = generate_prime(key_size / 2);
    let q = generate_prime(key_size / 2);
    let n = &p * &q;
    let phi_n = (&p - BigUint::one()) * (&q - BigUint::one());
    let e = BigUint::from(65537u32);

    let d = match mod_inverse(&e, &phi_n) {
        Some(d) => d,
        None => {
            eprintln!("❌ Error: Failed to compute modular inverse. This is rare - try again.");
            return;
        }
    };

    let key_pair = RsaKeyPair {
        public_key: RsaPublicKey { n: n.clone(), e },
        private_key: RsaPrivateKey { n, d },
    };

    // Ensure message is smaller than modulus
    if message_int >= key_pair.public_key.n {
        eprintln!("❌ Error: Message integer representation is too large for the modulus");
        eprintln!("💡 Tip: Use a larger key size");
        return;
    }

    // Print key generation steps
    print_key_generation_steps(&key_pair, &p, &q, &phi_n);

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
        println!("Original: {:?}", message.as_bytes());
        println!("Decrypted: {:?}", decrypted_message.as_bytes());
    }
}