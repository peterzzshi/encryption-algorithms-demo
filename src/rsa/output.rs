use super::math_utils::{print_header, print_step};
use super::types::{MessageFormat, RsaKeyPair, RsaPrivateKey, RsaPublicKey};

pub(super) fn print_error(error: &str) {
    eprintln!("\n❌ Error: {}", error);
    eprintln!("💡 Tip: Good small primes: 3, 5, 7, 11, 13, 17, 19, 23, 29, 31");
}

pub(super) fn print_demo_header(format: &MessageFormat, message_number: u64, p: u64, q: u64) {
    match format {
        MessageFormat::Number => {
            println!("🔐 RSA Encryption Algorithm Demo");
            println!("Message (m): {}", message_number);
        }
        MessageFormat::Text(text) => {
            println!("🔐 RSA Text Encryption Demo");
            println!("Text: \"{}\"", text);
            println!("Text converted to number: {}", message_number);
            println!("  (RSA operates on numbers - text must be converted first)");
        }
    }
    println!("Prime p: {}", p);
    println!("Prime q: {}", q);
}

pub(super) fn print_key_generation_steps(key_pair: &RsaKeyPair, p: u64, q: u64, phi_n: u64) {
    print_header("RSA Key Generation");
    print_step("Step 1 - Prime p", &p.to_string());
    print_step("Step 2 - Prime q", &q.to_string());
    print_step(
        "Step 3 - Calculate n = p × q",
        &key_pair.public_key.n.to_string(),
    );
    print_step(
        "Step 4 - Calculate φ(n) = (p-1) × (q-1)",
        &phi_n.to_string(),
    );
    print_step("Step 5 - Choose e", &key_pair.public_key.e.to_string());

    println!("  Step 6 - Calculate d = e⁻¹ mod φ(n):");
    println!(
        "    Need: {} × d ≡ 1 (mod {})",
        key_pair.public_key.e, phi_n
    );
    println!("    Solution: d = {}", key_pair.private_key.d);
    println!(
        "    Verification: {} × {} mod {} = {}",
        key_pair.public_key.e,
        key_pair.private_key.d,
        phi_n,
        key_pair.public_key.e as u128 * key_pair.private_key.d as u128 % phi_n as u128
    );

    print_step(
        "Public Key (n, e)",
        &format!("({}, {})", key_pair.public_key.n, key_pair.public_key.e),
    );
    print_step(
        "Private Key (n, d)",
        &format!("({}, {})", key_pair.private_key.n, key_pair.private_key.d),
    );
}

pub(super) fn print_encryption_steps(message: u64, ciphertext: u64, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Message as number (m)", &message.to_string());
    print_step("Public key (n)", &public_key.n.to_string());
    print_step("Public key (e)", &public_key.e.to_string());
    print_step("Formula", "c = m^e mod n");

    if public_key.e <= 10 && message < 1000 {
        let power_result = message.pow(public_key.e as u32);
        println!(
            "  Calculation: c = {}^{} mod {}",
            message, public_key.e, public_key.n
        );
        println!("    Step 1: {}^{} = {}", message, public_key.e, power_result);
        println!(
            "    Step 2: {} mod {} = {}",
            power_result, public_key.n, ciphertext
        );
    }

    print_step("Ciphertext (c)", &ciphertext.to_string());
}

pub(super) fn print_decryption_steps(ciphertext: u64, decrypted: u64, private_key: &RsaPrivateKey) {
    print_header("RSA Decryption");
    print_step("Ciphertext (c)", &ciphertext.to_string());
    print_step("Private key (n)", &private_key.n.to_string());
    print_step("Private key (d)", &private_key.d.to_string());
    print_step("Formula", "m = c^d mod n");
    println!(
        "  Calculation: m = {}^{} mod {}",
        ciphertext, private_key.d, private_key.n
    );
    println!("    (Using modular exponentiation for efficiency)");
    print_step("Decrypted number (m)", &decrypted.to_string());
}

pub(super) fn print_verification(
    format: &MessageFormat,
    original_number: u64,
    decrypted_number: u64,
    decrypted_text: Option<&str>,
) {
    print_header("Verification");

    if let Some(original_text) = format.as_text() {
        // Text input case
        print_step("Original text", original_text);
        print_step("Original as number", &original_number.to_string());
        print_step("Decrypted as number", &decrypted_number.to_string());
        if let Some(dec_text) = decrypted_text {
            print_step("Decrypted back to text", dec_text);
        }
        let match_result =
            original_text == decrypted_text.unwrap_or("") && original_number == decrypted_number;
        print_step("Match", &match_result.to_string());
    } else {
        // Number input case
        print_step("Original number", &original_number.to_string());
        print_step("Decrypted number", &decrypted_number.to_string());
        print_step(
            "Match",
            &(original_number == decrypted_number).to_string(),
        );
    }
}

pub(super) fn print_result(success: bool) {
    if success {
        println!("\n✅ RSA encryption/decryption successful!");
    } else {
        println!("\n❌ RSA encryption/decryption failed!");
    }
}

pub(super) fn print_mathematical_foundation() {
    println!();
    println!("📚 Mathematical Foundation:");
    println!("RSA works because of Euler's theorem: if gcd(m,n)=1, then m^φ(n) ≡ 1 (mod n)");
    println!("Since e×d ≡ 1 (mod φ(n)), we have: m^(e×d) ≡ m (mod n)");
}

pub(super) fn print_text_encoding_note() {
    println!();
    println!("💡 Text Encoding Explained:");
    println!("   RSA is a mathematical algorithm that ONLY works with numbers.");
    println!("   Any text must be converted to a number first:");
    println!("   1. Each character → byte value (H=72, i=105)");
    println!("   2. Bytes combined into one number: (72 << 8) | 105 = 18537");
    println!("   3. RSA encrypts the NUMBER: 18537^e mod n");
    println!("   4. After decryption, convert the number back to text");
}
