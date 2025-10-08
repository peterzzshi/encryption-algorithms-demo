use super::types::Hash;
use super::math_utils::words_to_hex;

pub fn print_header(message_text: Option<&str>, message_bytes: &[u8]) {
    println!("🔐 SHA-256 Hash Algorithm Demo");
    if let Some(text) = message_text {
        println!("Message: \"{}\"", text);
    }
    println!("Message bytes: {:02x?}", message_bytes);
    println!("Message length: {} bytes", message_bytes.len());
}

pub fn print_initial_values() {
    println!("\n=== Initial Hash Values ===");
    println!("  H₀ = 0x6a09e667  (√2)");
    println!("  H₁ = 0xbb67ae85  (√3)");
    println!("  H₂ = 0x3c6ef372  (√5)");
    println!("  H₃ = 0xa54ff53a  (√7)");
    println!("  H₄ = 0x510e527f  (√11)");
    println!("  H₅ = 0x9b05688c  (√13)");
    println!("  H₆ = 0x1f83d9ab  (√17)");
    println!("  H₇ = 0x5be0cd19  (√19)");
}

pub fn print_final_hash(hash: &Hash) {
    println!("\n=== Final Hash ===");
    println!("  SHA-256: {}", words_to_hex(hash));

    println!("\n  Hash breakdown:");
    for (i, word) in hash.iter().enumerate() {
        println!("    H[{}] = 0x{:08x}", i, word);
    }
}

pub fn print_compression_header(block_num: usize, total_blocks: usize) {
    println!("\n=== Compression Function (Block {}/{}) ===", block_num + 1, total_blocks);
}