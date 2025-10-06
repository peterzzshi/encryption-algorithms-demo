use super::types::RsaKeyPair;

pub(super) fn print_header(message_text: Option<&str>, message_number: u64, p: u64, q: u64) {
    if let Some(text) = message_text {
        println!("🔐 RSA Text Encryption Demo");
        println!("Text: \"{}\" → Number: {}", text, message_number);
    } else {
        println!("🔐 RSA Number Encryption Demo");
        println!("Message: {}", message_number);
    }
    println!("Primes: p={}, q={}", p, q);
}

pub(super) fn print_keys(key_pair: &RsaKeyPair, p: u64, q: u64) {
    println!("\n=== Key Generation ===");
    let phi_n = (p - 1) * (q - 1);

    println!("  n = p × q = {} × {} = {}", p, q, key_pair.public_key.n);
    println!("  φ(n) = (p-1) × (q-1) = {}", phi_n);
    println!("  Public key:  (n={}, e={})", key_pair.public_key.n, key_pair.public_key.e);
    println!("  Private key: (n={}, d={})", key_pair.private_key.n, key_pair.private_key.d);

    // Verification
    let check = key_pair.public_key.e as u128 * key_pair.private_key.d as u128 % phi_n as u128;
    println!("  Verify: {} × {} ≡ {} (mod {})", key_pair.public_key.e, key_pair.private_key.d, check, phi_n);
}

pub(super) fn print_verification(
    original_text: Option<&str>,
    original_number: u64,
    decrypted_number: u64,
    decrypted_text: Option<&str>,
) {
    println!("\n=== Verification ===");

    let success = if let Some(orig_text) = original_text {
        println!("  Original text: {}", orig_text);
        println!("  Original number: {}", original_number);
        println!("  Decrypted number: {}", decrypted_number);
        if let Some(dec_text) = decrypted_text {
            println!("  Decrypted text: {}", dec_text);
            orig_text == dec_text && original_number == decrypted_number
        } else {
            false
        }
    } else {
        println!("  Original: {}", original_number);
        println!("  Decrypted: {}", decrypted_number);
        original_number == decrypted_number
    };

    if success {
        println!("\n✅ Success! Encryption and decryption worked correctly.");
    } else {
        println!("\n❌ Failed! Something went wrong.");
    }
}
