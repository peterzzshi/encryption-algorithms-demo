use crate::rsa::key_generation::*;
use crate::rsa::encryption::*;
use crate::rsa::math_utils::*;

// ============================================================================
// Pure Functions (no side effects)
// ============================================================================

/// Validates prime numbers
fn validate_primes(p: u64, q: u64) -> Result<(), String> {
    if !is_prime(p) || !is_prime(q) {
        return Err("Both p and q must be prime numbers".to_string());
    }
    if p == q {
        return Err("p and q must be different primes".to_string());
    }
    Ok(())
}

/// Finds a suitable public exponent e and calculates private exponent d
fn find_exponent_pair(phi_n: u64) -> Option<(u64, u64)> {
    for e in [3, 5, 7, 17] {
        if let Some(d) = mod_inverse(e, phi_n) {
            return Some((e, d));
        }
    }
    None
}

/// Generates RSA key pair from two primes
fn generate_keypair(p: u64, q: u64) -> Result<RsaKeyPair, String> {
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    let (e, d) = find_exponent_pair(phi_n)
        .ok_or_else(|| "Could not find suitable public exponent e for these primes".to_string())?;

    Ok(RsaKeyPair {
        public_key: RsaPublicKey { n, e },
        private_key: RsaPrivateKey { n, d },
    })
}

/// Validates that message is smaller than modulus
fn validate_message_size(message: u64, n: u64) -> Result<(), String> {
    if message >= n {
        Err(format!("Message ({}) must be smaller than modulus n ({})", message, n))
    } else {
        Ok(())
    }
}

/// Performs encryption and decryption, returning results
fn encrypt_decrypt_cycle(message: u64, key_pair: &RsaKeyPair) -> (u64, u64) {
    let ciphertext = encrypt(message, &key_pair.public_key);
    let decrypted = decrypt(ciphertext, &key_pair.private_key);
    (ciphertext, decrypted)
}

// ============================================================================
// Side Effects (I/O operations)
// ============================================================================

fn print_error(error: &str) {
    eprintln!("\n❌ Error: {}", error);
    eprintln!("💡 Tip: Good small primes: 3, 5, 7, 11, 13, 17, 19, 23, 29, 31");
}

fn print_demo_header(message: u64, p: u64, q: u64) {
    println!("🔐 RSA Encryption Algorithm Demo");
    println!("Message (m): {}", message);
    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
}

fn print_text_demo_header(text: &str, message: u64, p: u64, q: u64) {
    println!("🔐 RSA Text Encryption Demo");
    println!("Text: \"{}\"", text);
    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
    println!("Text as number: {}", message);
}

fn print_key_generation_steps(key_pair: &RsaKeyPair, p: u64, q: u64, phi_n: u64) {
    print_header("RSA Key Generation");
    print_step("Step 1 - Prime p", &p.to_string());
    print_step("Step 2 - Prime q", &q.to_string());
    print_step("Step 3 - Calculate n = p × q", &key_pair.public_key.n.to_string());
    print_step("Step 4 - Calculate φ(n) = (p-1) × (q-1)", &phi_n.to_string());
    print_step("Step 5 - Choose e", &key_pair.public_key.e.to_string());

    println!("  Step 6 - Calculate d = e⁻¹ mod φ(n):");
    println!("    Need: {} × d ≡ 1 (mod {})", key_pair.public_key.e, phi_n);
    println!("    Solution: d = {}", key_pair.private_key.d);
    println!("    Verification: {} × {} mod {} = {}",
             key_pair.public_key.e,
             key_pair.private_key.d,
             phi_n,
             (key_pair.public_key.e as u128 * key_pair.private_key.d as u128 % phi_n as u128));

    print_step("Public Key (n, e)", &format!("({}, {})", key_pair.public_key.n, key_pair.public_key.e));
    print_step("Private Key (n, d)", &format!("({}, {})", key_pair.private_key.n, key_pair.private_key.d));
}

fn print_encryption_steps(message: u64, ciphertext: u64, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Message (m)", &message.to_string());
    print_step("Public key (n)", &public_key.n.to_string());
    print_step("Public key (e)", &public_key.e.to_string());
    print_step("Formula", "c = m^e mod n");

    if public_key.e <= 10 && message < 1000 {
        let power_result = message.pow(public_key.e as u32);
        println!("  Calculation: c = {}^{} mod {}", message, public_key.e, public_key.n);
        println!("    Step 1: {}^{} = {}", message, public_key.e, power_result);
        println!("    Step 2: {} mod {} = {}", power_result, public_key.n, ciphertext);
    }

    print_step("Ciphertext (c)", &ciphertext.to_string());
}

fn print_decryption_steps(ciphertext: u64, decrypted: u64, private_key: &RsaPrivateKey) {
    print_header("RSA Decryption");
    print_step("Ciphertext (c)", &ciphertext.to_string());
    print_step("Private key (n)", &private_key.n.to_string());
    print_step("Private key (d)", &private_key.d.to_string());
    print_step("Formula", "m = c^d mod n");
    println!("  Calculation: m = {}^{} mod {}", ciphertext, private_key.d, private_key.n);
    println!("    (Using modular exponentiation for efficiency)");
    print_step("Decrypted message (m)", &decrypted.to_string());
}

fn print_verification(original: u64, decrypted: u64) {
    print_header("Verification");
    print_step("Original message", &original.to_string());
    print_step("Decrypted message", &decrypted.to_string());
    print_step("Match", &(original == decrypted).to_string());
}

fn print_text_verification(original_text: &str, original_num: u64, decrypted_num: u64, decrypted_text: &str) {
    print_header("Verification");
    print_step("Original text", original_text);
    print_step("Original as number", &original_num.to_string());
    print_step("Decrypted as number", &decrypted_num.to_string());
    print_step("Decrypted text", decrypted_text);
    print_step("Match", &(original_text == decrypted_text && original_num == decrypted_num).to_string());
}

fn print_result(success: bool) {
    if success {
        println!("\n✅ RSA encryption/decryption successful!");
    } else {
        println!("\n❌ RSA encryption/decryption failed!");
    }
}

fn print_mathematical_foundation() {
    println!();
    println!("📚 Mathematical Foundation:");
    println!("RSA works because of Euler's theorem: if gcd(m,n)=1, then m^φ(n) ≡ 1 (mod n)");
    println!("Since e×d ≡ 1 (mod φ(n)), we have: m^(e×d) ≡ m (mod n)");
}

fn print_text_encoding_note() {
    println!();
    println!("💡 Text Encoding: Each character is converted to its byte value,");
    println!("   then combined into a single number for encryption.");
}

// ============================================================================
// Main Entry Points (orchestration)
// ============================================================================

pub fn run_rsa_demo(message: u64, p: u64, q: u64) {
    print_demo_header(message, p, q);

    // Validate inputs
    if let Err(error) = validate_primes(p, q) {
        print_error(&error);
        return;
    }

    // Generate key pair
    println!("\n⏳ Generating RSA key pair...");
    let key_pair = match generate_keypair(p, q) {
        Ok(kp) => kp,
        Err(error) => {
            print_error(&error);
            return;
        }
    };

    // Validate message size
    if let Err(error) = validate_message_size(message, key_pair.public_key.n) {
        eprintln!("\n❌ Error: {}", error);
        eprintln!("💡 Tip: Use larger primes or a smaller message");
        return;
    }

    // Perform encryption/decryption
    let phi_n = (p - 1) * (q - 1);
    let (ciphertext, decrypted) = encrypt_decrypt_cycle(message, &key_pair);

    // Print all results
    print_key_generation_steps(&key_pair, p, q, phi_n);
    print_encryption_steps(message, ciphertext, &key_pair.public_key);
    print_decryption_steps(ciphertext, decrypted, &key_pair.private_key);
    print_verification(message, decrypted);
    print_result(message == decrypted);
    print_mathematical_foundation();
}

pub fn run_rsa_demo_text(text: &str, p: u64, q: u64) {
    // Convert text to number
    let message = match text_to_number(text) {
        Some(num) => num,
        None => {
            eprintln!("\n❌ Error: Text is too long or empty. Maximum 8 characters.");
            return;
        }
    };

    print_text_demo_header(text, message, p, q);

    // Validate inputs
    if let Err(error) = validate_primes(p, q) {
        print_error(&error);
        return;
    }

    // Generate key pair
    println!("\n⏳ Generating RSA key pair...");
    let key_pair = match generate_keypair(p, q) {
        Ok(kp) => kp,
        Err(error) => {
            print_error(&error);
            return;
        }
    };

    // Validate message size
    if let Err(error) = validate_message_size(message, key_pair.public_key.n) {
        eprintln!("\n❌ Error: {}", error);
        eprintln!("💡 Tip: Use larger primes or shorter text");
        return;
    }

    // Perform encryption/decryption
    let phi_n = (p - 1) * (q - 1);
    let (ciphertext, decrypted) = encrypt_decrypt_cycle(message, &key_pair);
    let decrypted_text = number_to_text(decrypted, text.len());

    // Print all results
    print_key_generation_steps(&key_pair, p, q, phi_n);
    print_encryption_steps(message, ciphertext, &key_pair.public_key);
    print_decryption_steps(ciphertext, decrypted, &key_pair.private_key);
    print_text_verification(text, message, decrypted, &decrypted_text);
    print_result(message == decrypted && text == decrypted_text);
    print_mathematical_foundation();
    print_text_encoding_note();
}
