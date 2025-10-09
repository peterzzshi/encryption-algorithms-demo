use super::encryption::{decrypt, encrypt};
use super::key_generation::generate_keypair;
use super::text_encoding::{number_to_text, text_to_number};
use super::validation::{is_valid_message_size, validate_primes};

fn run_rsa_demo_internal(message_text: Option<&str>, message_number: u64, p: u64, q: u64) {
    // Header
    if let Some(text) = message_text {
        println!("🔐 RSA Text Encryption Demo");
        println!("Text: \"{}\" → Number: {}", text, message_number);
    } else {
        println!("🔐 RSA Number Encryption Demo");
        println!("Message: {}", message_number);
    }
    println!("Primes: p={}, q={}", p, q);

    validate_primes(p, q);

    println!("\n⏳ Generating RSA key pair...");
    let key_pair = generate_keypair(p, q);

    if !is_valid_message_size(message_number, key_pair.public_key.n) {
        eprintln!(
            "\n❌ Error: Message ({}) must be smaller than modulus n ({})",
            message_number, key_pair.public_key.n
        );
        eprintln!("💡 Tip: Use larger primes or a smaller message");
        return;
    }

    // Print keys
    println!("\n=== Key Generation ===");
    let phi_n = (p - 1) * (q - 1);
    println!("  n = p × q = {} × {} = {}", p, q, key_pair.public_key.n);
    println!("  φ(n) = (p-1) × (q-1) = {}", phi_n);
    println!("  Public key:  (n={}, e={})", key_pair.public_key.n, key_pair.public_key.e);
    println!("  Private key: (n={}, d={})", key_pair.private_key.n, key_pair.private_key.d);
    let check = key_pair.public_key.e as u128 * key_pair.private_key.d as u128 % phi_n as u128;
    println!("  Verify: {} × {} ≡ {} (mod {})", key_pair.public_key.e, key_pair.private_key.d, check, phi_n);

    // Encryption
    let ciphertext = encrypt(message_number, &key_pair.public_key);
    println!("\n=== Encryption ===");
    println!("  c = m^e mod n = {}^{} mod {} = {}", message_number, key_pair.public_key.e, key_pair.public_key.n, ciphertext);

    // Decryption
    let decrypted_number = decrypt(ciphertext, &key_pair.private_key);
    println!("\n=== Decryption ===");
    println!("  m = c^d mod n = {}^{} mod {} = {}", ciphertext, key_pair.private_key.d, key_pair.private_key.n, decrypted_number);

    // Verification
    println!("\n=== Verification ===");
    let success = if let Some(orig_text) = message_text {
        let decrypted_text = number_to_text(decrypted_number, orig_text.len());
        println!("  Original text: {}", orig_text);
        println!("  Original number: {}", message_number);
        println!("  Decrypted number: {}", decrypted_number);
        println!("  Decrypted text: {}", decrypted_text);
        orig_text == decrypted_text && message_number == decrypted_number
    } else {
        println!("  Original: {}", message_number);
        println!("  Decrypted: {}", decrypted_number);
        message_number == decrypted_number
    };

    if success {
        println!("\n✅ Success! Encryption and decryption worked correctly.");
    } else {
        println!("\n❌ Failed! Something went wrong.");
    }
}

// ============================================================================
// Public API - Clean interface for main.rs
// ============================================================================

pub fn run_rsa_demo(message: u64, p: u64, q: u64) {
    run_rsa_demo_internal(None, message, p, q);
}

pub fn run_rsa_demo_text(text: &str, p: u64, q: u64) {
    match text_to_number(text) {
        Some(number) => run_rsa_demo_internal(Some(text), number, p, q),
        None => {
            eprintln!("\n❌ Error: Text is too long or empty. Maximum 8 characters.");
        }
    }
}
