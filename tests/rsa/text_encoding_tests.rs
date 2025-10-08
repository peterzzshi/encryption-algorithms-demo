use encryption_demo::rsa::text_encoding::{number_to_text, text_to_number};

#[test]
fn test_text_to_number_basic() {
    let number = text_to_number("Hi").unwrap();
    assert!(number > 0);
}

#[test]
fn test_text_to_number_empty() {
    assert!(text_to_number("").is_none());
}

#[test]
fn test_text_to_number_max_length() {
    let text = "12345678"; // 8 bytes
    assert!(text_to_number(text).is_some());
}

#[test]
fn test_text_to_number_too_long() {
    let text = "123456789"; // 9 bytes
    assert!(text_to_number(text).is_none());
}

#[test]
fn test_text_to_number_uniqueness() {
    let num1 = text_to_number("Hi").unwrap();
    let num2 = text_to_number("Bye").unwrap();
    assert_ne!(num1, num2);
}

#[test]
fn test_number_to_text_basic() {
    let text = "Hi";
    let number = text_to_number(text).unwrap();
    let recovered = number_to_text(number, text.len());
    assert_eq!(text, recovered);
}

#[test]
fn test_text_encoding_case_sensitive() {
    let num1 = text_to_number("Hi").unwrap();
    let num2 = text_to_number("hi").unwrap();
    assert_ne!(num1, num2);
}
