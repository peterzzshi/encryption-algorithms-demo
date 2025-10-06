use super::encryption::{decrypt, encrypt};
use super::key_generation::generate_keypair;
use super::output::{print_header, print_keys, print_verification};
use super::text_encoding::{number_to_text, text_to_number};
use super::validation::{is_valid_message_size, validate_primes};

fn run_rsa_demo_internal(message_text: Option<&str>, message_number: u64, p: u64, q: u64) {
    print_header(message_text, message_number, p, q);

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

    print_keys(&key_pair, p, q);

    let ciphertext = encrypt(message_number, &key_pair.public_key);
    println!("\n=== Encryption ===");
    println!("  c = m^e mod n = {}^{} mod {} = {}", message_number, key_pair.public_key.e, key_pair.public_key.n, ciphertext);

    let decrypted_number = decrypt(ciphertext, &key_pair.private_key);
    println!("\n=== Decryption ===");
    println!("  m = c^d mod n = {}^{} mod {} = {}", ciphertext, key_pair.private_key.d, key_pair.private_key.n, decrypted_number);

    let decrypted_text = message_text.map(|text| number_to_text(decrypted_number, text.len()));
    print_verification(
        message_text,
        message_number,
        decrypted_number,
        decrypted_text.as_deref(),
    );
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
