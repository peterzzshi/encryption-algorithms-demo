use super::encryption::{decrypt, encrypt};
use super::key_generation::generate_keypair;
use super::output::{
    print_decryption_steps, print_demo_header, print_encryption_steps, print_error,
    print_key_generation_steps, print_mathematical_foundation, print_result,
    print_text_encoding_note, print_verification,
};
use super::text_encoding::{number_to_text, text_to_number};
use super::types::{InputType, MessageFormat};
use super::validation::{validate_message_size, validate_primes};

// ============================================================================
// Main Entry Point (orchestration)
// ============================================================================

/// Core RSA demo function that handles ANY input type
/// This demonstrates that RSA always operates on numbers, regardless of input
fn run_rsa_demo_internal(input: InputType, p: u64, q: u64) {
    // Step 1: Convert input to number (required for RSA)
    let message_number = match input.to_number(text_to_number) {
        Some(num) => num,
        None => {
            eprintln!("\n❌ Error: Text is too long or empty. Maximum 8 characters.");
            return;
        }
    };

    let message_format = MessageFormat::from(&input);
    print_demo_header(&message_format, message_number, p, q);

    // Step 2: Validate primes
    if let Err(error) = validate_primes(p, q) {
        print_error(&error);
        return;
    }

    // Step 3: Generate RSA keys
    println!("\n⏳ Generating RSA key pair...");
    let key_pair = match generate_keypair(p, q) {
        Ok(kp) => kp,
        Err(error) => {
            print_error(&error);
            return;
        }
    };

    // Step 4: Validate message size
    if let Err(error) = validate_message_size(message_number, key_pair.public_key.n) {
        eprintln!("\n❌ Error: {}", error);
        let tip = if input.is_text() {
            "💡 Tip: Use larger primes or shorter text"
        } else {
            "💡 Tip: Use larger primes or a smaller message"
        };
        eprintln!("{}", tip);
        return;
    }

    // Step 5: Perform RSA encryption/decryption (THE CORE OPERATION)
    let phi_n = (p - 1) * (q - 1);
    let ciphertext = encrypt(message_number, &key_pair.public_key);
    let decrypted_number = decrypt(ciphertext, &key_pair.private_key);

    // Step 6: If input was text, convert the decrypted number back to text
    let decrypted_text = input
        .as_text()
        .map(|text| number_to_text(decrypted_number, text.len()));

    // Step 7: Determine success
    let success = match (&input, decrypted_text.as_deref()) {
        (InputType::Text(original_text), Some(dec_text)) => {
            message_number == decrypted_number && original_text == dec_text
        }
        _ => message_number == decrypted_number,
    };

    // Step 8: Print all results
    print_key_generation_steps(&key_pair, p, q, phi_n);
    print_encryption_steps(message_number, ciphertext, &key_pair.public_key);
    print_decryption_steps(ciphertext, decrypted_number, &key_pair.private_key);
    print_verification(
        &message_format,
        message_number,
        decrypted_number,
        decrypted_text.as_deref(),
    );
    print_result(success);
    print_mathematical_foundation();

    if input.is_text() {
        print_text_encoding_note();
    }
}

// ============================================================================
// Public API
// ============================================================================

pub fn run_rsa_demo(message: u64, p: u64, q: u64) {
    run_rsa_demo_internal(InputType::Number(message), p, q);
}

pub fn run_rsa_demo_text(text: &str, p: u64, q: u64) {
    run_rsa_demo_internal(InputType::Text(text.to_string()), p, q);
}
