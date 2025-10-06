use crate::rsa::key_generation::*;
use crate::rsa::encryption::*;
use crate::rsa::math_utils::*;

// ============================================================================
// Pure Functions (no side effects)
// ============================================================================

/// Input type for RSA - everything must eventually become a number
enum MessageInput {
    Number(u64),
    Text(String),
}

impl MessageInput {
    /// Convert any input to a number (core requirement for RSA)
    fn to_number(&self) -> Result<u64, String> {
        match self {
            MessageInput::Number(n) => Ok(*n),
            MessageInput::Text(text) => {
                text_to_number(text)
                    .ok_or_else(|| "Text is too long or empty. Maximum 8 characters.".to_string())
            }
        }
    }

    fn is_text(&self) -> bool {
        matches!(self, MessageInput::Text(_))
    }

    fn as_text(&self) -> Option<&str> {
        match self {
            MessageInput::Text(text) => Some(text),
            _ => None,
        }
    }
}

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

/// Core RSA operation: encrypt then decrypt
/// This is where the actual mathematical work happens
fn encrypt_decrypt_cycle(message_number: u64, key_pair: &RsaKeyPair) -> (u64, u64) {
    let ciphertext = encrypt(message_number, &key_pair.public_key);
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

fn print_demo_header(input: &MessageInput, message_number: u64, p: u64, q: u64) {
    match input {
        MessageInput::Number(_) => {
            println!("🔐 RSA Encryption Algorithm Demo");
            println!("Message (m): {}", message_number);
        }
        MessageInput::Text(text) => {
            println!("🔐 RSA Text Encryption Demo");
            println!("Text: \"{}\"", text);
            println!("Text converted to number: {}", message_number);
            println!("  (RSA operates on numbers - text must be converted first)");
        }
    }
    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
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
             key_pair.public_key.e as u128 * key_pair.private_key.d as u128 % phi_n as u128);

    print_step("Public Key (n, e)", &format!("({}, {})", key_pair.public_key.n, key_pair.public_key.e));
    print_step("Private Key (n, d)", &format!("({}, {})", key_pair.private_key.n, key_pair.private_key.d));
}

fn print_encryption_steps(message: u64, ciphertext: u64, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Message as number (m)", &message.to_string());
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
    print_step("Decrypted number (m)", &decrypted.to_string());
}

fn print_verification(input: &MessageInput, original_number: u64, decrypted_number: u64, decrypted_text: Option<&str>) {
    print_header("Verification");

    if let Some(original_text) = input.as_text() {
        // Text input case
        print_step("Original text", original_text);
        print_step("Original as number", &original_number.to_string());
        print_step("Decrypted as number", &decrypted_number.to_string());
        if let Some(dec_text) = decrypted_text {
            print_step("Decrypted back to text", dec_text);
        }
        let match_result = original_text == decrypted_text.unwrap_or("") && original_number == decrypted_number;
        print_step("Match", &match_result.to_string());
    } else {
        // Number input case
        print_step("Original number", &original_number.to_string());
        print_step("Decrypted number", &decrypted_number.to_string());
        print_step("Match", &(original_number == decrypted_number).to_string());
    }
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
    println!("💡 Text Encoding Explained:");
    println!("   RSA is a mathematical algorithm that ONLY works with numbers.");
    println!("   Any text must be converted to a number first:");
    println!("   1. Each character → byte value (H=72, i=105)");
    println!("   2. Bytes combined into one number: (72 << 8) | 105 = 18537");
    println!("   3. RSA encrypts the NUMBER: 18537^e mod n");
    println!("   4. After decryption, convert the number back to text");
}

// ============================================================================
// Main Entry Point (orchestration) - SINGLE UNIFIED FUNCTION
// ============================================================================

/// Core RSA demo function that handles ANY input type
/// This demonstrates that RSA always operates on numbers, regardless of input
fn run_rsa_demo_internal(input: MessageInput, p: u64, q: u64) {
    // Step 1: Convert input to number (required for RSA)
    let message_number = match input.to_number() {
        Ok(num) => num,
        Err(error) => {
            eprintln!("\n❌ Error: {}", error);
            return;
        }
    };

    print_demo_header(&input, message_number, p, q);

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
    let (ciphertext, decrypted_number) = encrypt_decrypt_cycle(message_number, &key_pair);

    // Step 6: If input was text, convert the decrypted number back to text
    let decrypted_text = input.as_text().map(|text| number_to_text(decrypted_number, text.len()));

    // Step 7: Determine success
    let success = match (&input, decrypted_text.as_deref()) {
        (MessageInput::Text(original_text), Some(dec_text)) => {
            message_number == decrypted_number && original_text == dec_text
        }
        _ => message_number == decrypted_number,
    };

    // Step 8: Print all results
    print_key_generation_steps(&key_pair, p, q, phi_n);
    print_encryption_steps(message_number, ciphertext, &key_pair.public_key);
    print_decryption_steps(ciphertext, decrypted_number, &key_pair.private_key);
    print_verification(&input, message_number, decrypted_number, decrypted_text.as_deref());
    print_result(success);
    print_mathematical_foundation();

    if input.is_text() {
        print_text_encoding_note();
    }
}

// ============================================================================
// Public API - Simple wrappers
// ============================================================================

pub fn run_rsa_demo(message: u64, p: u64, q: u64) {
    run_rsa_demo_internal(MessageInput::Number(message), p, q);
}

pub fn run_rsa_demo_text(text: &str, p: u64, q: u64) {
    run_rsa_demo_internal(MessageInput::Text(text.to_string()), p, q);
}
