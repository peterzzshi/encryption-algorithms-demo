// ============================================================================
// WebAssembly Bindings
// ============================================================================
// This module exposes the encryption algorithms to JavaScript via WASM

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::rsa::encryption::{decrypt, encrypt};
use crate::rsa::key_generation::generate_keypair;
use crate::rsa::math_utils::is_prime;
use crate::rsa::text_encoding::{number_to_text, text_to_number};
use crate::sha256::compression::compress_block;
use crate::sha256::constants::INITIAL_HASH;
use crate::sha256::math_utils::words_to_hex;
use crate::sha256::preprocessing::preprocess_message;

// ============================================================================
// RSA Types for WASM
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct RsaKeyPairResult {
    pub n: u64,
    pub e: u64,
    pub d: u64,
    pub p: u64,
    pub q: u64,
    pub phi_n: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RsaStep {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub formula: String,
    pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct RsaDemoResult {
    pub success: bool,
    pub error: Option<String>,
    pub original_message: String,
    pub message_number: u64,
    pub key_pair: Option<RsaKeyPairResult>,
    pub ciphertext: u64,
    pub decrypted_number: u64,
    pub decrypted_text: Option<String>,
    pub steps: Vec<RsaStep>,
}

// ============================================================================
// SHA-256 Types for WASM
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct Sha256Step {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Sha256DemoResult {
    pub success: bool,
    pub error: Option<String>,
    pub original_message: String,
    pub message_bytes: Vec<u8>,
    pub hash: String,
    pub hash_words: Vec<String>,
    pub steps: Vec<Sha256Step>,
}

// ============================================================================
// RSA WASM Functions
// ============================================================================

#[wasm_bindgen]
pub fn rsa_demo_number(message: u64, p: u64, q: u64) -> String {
    let result = run_rsa_demo_internal(None, message, p, q);
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn rsa_demo_text(text: &str, p: u64, q: u64) -> String {
    match text_to_number(text) {
        Some(number) => {
            let result = run_rsa_demo_internal(Some(text.to_string()), number, p, q);
            serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
        }
        None => {
            let result = RsaDemoResult {
                success: false,
                error: Some("Text is too long or empty. Maximum 8 characters.".to_string()),
                original_message: text.to_string(),
                message_number: 0,
                key_pair: None,
                ciphertext: 0,
                decrypted_number: 0,
                decrypted_text: None,
                steps: vec![],
            };
            serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

#[wasm_bindgen]
pub fn validate_prime(n: u64) -> bool {
    is_prime(n)
}

fn run_rsa_demo_internal(message_text: Option<String>, message_number: u64, p: u64, q: u64) -> RsaDemoResult {
    let mut steps = Vec::new();

    // Validate primes
    if !is_prime(p) {
        return RsaDemoResult {
            success: false,
            error: Some(format!("{} is not a prime number", p)),
            original_message: message_text.unwrap_or_else(|| message_number.to_string()),
            message_number,
            key_pair: None,
            ciphertext: 0,
            decrypted_number: 0,
            decrypted_text: None,
            steps: vec![],
        };
    }

    if !is_prime(q) {
        return RsaDemoResult {
            success: false,
            error: Some(format!("{} is not a prime number", q)),
            original_message: message_text.unwrap_or_else(|| message_number.to_string()),
            message_number,
            key_pair: None,
            ciphertext: 0,
            decrypted_number: 0,
            decrypted_text: None,
            steps: vec![],
        };
    }

    if p == q {
        return RsaDemoResult {
            success: false,
            error: Some("p and q must be different prime numbers".to_string()),
            original_message: message_text.unwrap_or_else(|| message_number.to_string()),
            message_number,
            key_pair: None,
            ciphertext: 0,
            decrypted_number: 0,
            decrypted_text: None,
            steps: vec![],
        };
    }

    // Step 1: Calculate n
    let n = p * q;
    steps.push(RsaStep {
        step_number: 1,
        title: "Calculate n (modulus)".to_string(),
        description: "Multiply the two prime numbers to get n".to_string(),
        formula: format!("n = p × q = {} × {}", p, q),
        result: n.to_string(),
    });

    // Step 2: Calculate φ(n)
    let phi_n = (p - 1) * (q - 1);
    steps.push(RsaStep {
        step_number: 2,
        title: "Calculate φ(n) (Euler's totient)".to_string(),
        description: "Calculate the totient function φ(n) = (p-1)(q-1)".to_string(),
        formula: format!("φ(n) = (p-1) × (q-1) = ({}-1) × ({}-1) = {} × {}", p, q, p-1, q-1),
        result: phi_n.to_string(),
    });

    // Generate key pair
    let key_pair = generate_keypair(p, q);

    // Step 3: Select e
    steps.push(RsaStep {
        step_number: 3,
        title: "Select public exponent e".to_string(),
        description: "Choose e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1".to_string(),
        formula: format!("e = {} (coprime with φ(n) = {})", key_pair.public_key.e, phi_n),
        result: key_pair.public_key.e.to_string(),
    });

    // Step 4: Calculate d
    steps.push(RsaStep {
        step_number: 4,
        title: "Calculate private exponent d".to_string(),
        description: "Find d such that e × d ≡ 1 (mod φ(n))".to_string(),
        formula: format!("d = e⁻¹ mod φ(n) = {}⁻¹ mod {}", key_pair.public_key.e, phi_n),
        result: key_pair.private_key.d.to_string(),
    });

    // Verify e × d ≡ 1 (mod φ(n))
    let check = (key_pair.public_key.e as u128 * key_pair.private_key.d as u128) % phi_n as u128;
    steps.push(RsaStep {
        step_number: 5,
        title: "Verify key pair".to_string(),
        description: "Confirm that e × d ≡ 1 (mod φ(n))".to_string(),
        formula: format!("{} × {} mod {} = {}", key_pair.public_key.e, key_pair.private_key.d, phi_n, check),
        result: if check == 1 { "✓ Valid" } else { "✗ Invalid" }.to_string(),
    });

    // Check message size
    if message_number >= n {
        return RsaDemoResult {
            success: false,
            error: Some(format!("Message ({}) must be smaller than modulus n ({})", message_number, n)),
            original_message: message_text.unwrap_or_else(|| message_number.to_string()),
            message_number,
            key_pair: Some(RsaKeyPairResult {
                n,
                e: key_pair.public_key.e,
                d: key_pair.private_key.d,
                p,
                q,
                phi_n,
            }),
            ciphertext: 0,
            decrypted_number: 0,
            decrypted_text: None,
            steps,
        };
    }

    // Step 6: Encrypt
    let ciphertext = encrypt(message_number, &key_pair.public_key);
    steps.push(RsaStep {
        step_number: 6,
        title: "Encryption".to_string(),
        description: "Calculate ciphertext using public key".to_string(),
        formula: format!("c = m^e mod n = {}^{} mod {}", message_number, key_pair.public_key.e, n),
        result: ciphertext.to_string(),
    });

    // Step 7: Decrypt
    let decrypted_number = decrypt(ciphertext, &key_pair.private_key);
    steps.push(RsaStep {
        step_number: 7,
        title: "Decryption".to_string(),
        description: "Recover original message using private key".to_string(),
        formula: format!("m = c^d mod n = {}^{} mod {}", ciphertext, key_pair.private_key.d, n),
        result: decrypted_number.to_string(),
    });

    // Step 8: Verification
    let decrypted_text = message_text.as_ref().map(|t| number_to_text(decrypted_number, t.len()));
    let success = message_number == decrypted_number;

    steps.push(RsaStep {
        step_number: 8,
        title: "Verification".to_string(),
        description: "Compare original and decrypted messages".to_string(),
        formula: format!("Original: {} == Decrypted: {}", message_number, decrypted_number),
        result: if success { "✓ Success!" } else { "✗ Failed!" }.to_string(),
    });

    RsaDemoResult {
        success,
        error: None,
        original_message: message_text.unwrap_or_else(|| message_number.to_string()),
        message_number,
        key_pair: Some(RsaKeyPairResult {
            n,
            e: key_pair.public_key.e,
            d: key_pair.private_key.d,
            p,
            q,
            phi_n,
        }),
        ciphertext,
        decrypted_number,
        decrypted_text,
        steps,
    }
}

// ============================================================================
// SHA-256 WASM Functions
// ============================================================================

#[wasm_bindgen]
pub fn sha256_demo_text(text: &str) -> String {
    let result = run_sha256_demo_internal(Some(text.to_string()), text.as_bytes());
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn sha256_demo_bytes(bytes: &[u8]) -> String {
    let result = run_sha256_demo_internal(None, bytes);
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

fn run_sha256_demo_internal(message_text: Option<String>, message_bytes: &[u8]) -> Sha256DemoResult {
    let mut steps = Vec::new();

    // Step 1: Initial hash values
    steps.push(Sha256Step {
        step_number: 1,
        title: "Initial Hash Values".to_string(),
        description: "First 32 bits of fractional parts of square roots of first 8 primes".to_string(),
        data: vec![
            format!("H₀ = 0x{:08x} (√2)", INITIAL_HASH[0]),
            format!("H₁ = 0x{:08x} (√3)", INITIAL_HASH[1]),
            format!("H₂ = 0x{:08x} (√5)", INITIAL_HASH[2]),
            format!("H₃ = 0x{:08x} (√7)", INITIAL_HASH[3]),
            format!("H₄ = 0x{:08x} (√11)", INITIAL_HASH[4]),
            format!("H₅ = 0x{:08x} (√13)", INITIAL_HASH[5]),
            format!("H₆ = 0x{:08x} (√17)", INITIAL_HASH[6]),
            format!("H₇ = 0x{:08x} (√19)", INITIAL_HASH[7]),
        ],
    });

    // Step 2: Preprocessing
    let processed = preprocess_message(message_bytes);
    steps.push(Sha256Step {
        step_number: 2,
        title: "Message Preprocessing".to_string(),
        description: format!(
            "Original: {} bytes → Padded: {} bytes ({} block(s))",
            message_bytes.len(),
            processed.blocks.len() * 64,
            processed.blocks.len()
        ),
        data: vec![
            format!("Original bytes: {:02x?}", message_bytes),
            format!("Original length: {} bytes ({} bits)", message_bytes.len(), message_bytes.len() * 8),
            format!("After padding: {} blocks of 512 bits each", processed.blocks.len()),
        ],
    });

    // Step 3: Show first block words
    if !processed.blocks.is_empty() {
        let block_words: Vec<String> = processed.blocks[0]
            .iter()
            .enumerate()
            .map(|(i, word)| format!("W[{:2}] = 0x{:08x}", i, word))
            .collect();

        steps.push(Sha256Step {
            step_number: 3,
            title: "First Block (16 × 32-bit words)".to_string(),
            description: "Message bytes converted to 32-bit words".to_string(),
            data: block_words,
        });
    }

    // Process each block
    let mut hash = INITIAL_HASH;

    for (i, block) in processed.blocks.iter().enumerate() {
        hash = compress_block(hash, block, false);

        steps.push(Sha256Step {
            step_number: (4 + i) as u32,
            title: format!("Compression Function (Block {})", i + 1),
            description: format!("64 rounds of compression for block {}", i + 1),
            data: hash.iter().enumerate()
                .map(|(j, word)| format!("H[{}] = 0x{:08x}", j, word))
                .collect(),
        });
    }

    // Final hash
    let final_hash = words_to_hex(&hash);

    steps.push(Sha256Step {
        step_number: (4 + processed.blocks.len()) as u32,
        title: "Final Hash".to_string(),
        description: "Concatenate all hash words".to_string(),
        data: vec![final_hash.clone()],
    });

    Sha256DemoResult {
        success: true,
        error: None,
        original_message: message_text.unwrap_or_else(|| hex::encode(message_bytes)),
        message_bytes: message_bytes.to_vec(),
        hash: final_hash,
        hash_words: hash.iter().map(|w| format!("0x{:08x}", w)).collect(),
        steps,
    }
}

