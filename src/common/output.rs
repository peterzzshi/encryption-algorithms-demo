/// Common output formatting utilities

/// Format bytes as hex string (e.g., [0xAB, 0xCD] -> "abcd")
pub fn format_bytes_as_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
