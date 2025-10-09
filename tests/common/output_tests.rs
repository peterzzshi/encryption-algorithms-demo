use encryption_demo::common::output::format_bytes_as_hex;

#[test]
fn test_format_bytes_as_hex() {
    let bytes = vec![0x12, 0x34, 0xAB, 0xCD];
    assert_eq!(format_bytes_as_hex(&bytes), "1234abcd");
}

#[test]
fn test_format_bytes_as_hex_empty() {
    assert_eq!(format_bytes_as_hex(&[]), "");
}
