use super::constants::INITIAL_HASH;
use super::compression::compress_block;
use super::preprocessing::{preprocess_message, print_preprocessing_steps};
use super::output::{print_header, print_initial_values, print_final_hash, print_compression_header};
use super::validation::validate_message;

fn run_sha256_demo_internal(message_text: Option<&str>, message_bytes: &[u8]) {
    print_header(message_text, message_bytes);

    if let Err(e) = validate_message(message_bytes) {
        eprintln!("\n❌ Error: {}", e);
        return;
    }

    print_initial_values();

    // Preprocess message
    let processed = preprocess_message(message_bytes);
    print_preprocessing_steps(message_bytes, &processed);

    // Process each block
    let mut hash = INITIAL_HASH;

    for (i, block) in processed.blocks.iter().enumerate() {
        print_compression_header(i, processed.blocks.len());
        hash = compress_block(hash, block, true);

        println!("\n  Hash after block {}:", i + 1);
        for (j, word) in hash.iter().enumerate() {
            println!("    H[{}] = 0x{:08x}", j, word);
        }
    }

    print_final_hash(&hash);

    println!("\n✅ SHA-256 hash computation completed!");
}

// ============================================================================
// Public API - Clean interface for main.rs
// ============================================================================

pub fn run_sha256_demo(message_bytes: Vec<u8>) {
    run_sha256_demo_internal(None, &message_bytes);
}

pub fn run_sha256_demo_text(text: &str) {
    run_sha256_demo_internal(Some(text), text.as_bytes());
}