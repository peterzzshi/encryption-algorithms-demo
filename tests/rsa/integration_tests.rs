// Integration tests for the complete RSA workflow
use encryption_demo::rsa::encryption::{decrypt, encrypt};
use encryption_demo::rsa::key_generation::generate_keypair;
use encryption_demo::rsa::text_encoding::{number_to_text, text_to_number};

#[test]
fn test_complete_text_encryption_workflow() {
    // Generate keys
    let p = 251u64;
    let q = 241u64;
    let keypair = generate_keypair(p, q);

    // Encrypt text
    let text = "Hi";
    let message = text_to_number(text).unwrap();
    let ciphertext = encrypt(message, &keypair.public_key);

    // Decrypt and verify
    let decrypted = decrypt(ciphertext, &keypair.private_key);
    let recovered_text = number_to_text(decrypted, text.len());

    assert_eq!(text, recovered_text);
}

#[test]
fn test_complete_numeric_workflow() {
    // Generate keys
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);

    // Encrypt number
    let message = 42u64;
    let ciphertext = encrypt(message, &keypair.public_key);

    // Decrypt and verify
    let decrypted = decrypt(ciphertext, &keypair.private_key);

    assert_eq!(message, decrypted);
}

#[test]
fn test_multiple_text_messages() {
    // Use very large primes to handle longer texts
    let p = 4294967291u64; // Large prime near u32::MAX
    let q = 4294967279u64; // Large prime near u32::MAX
    let keypair = generate_keypair(p, q);

    let texts = vec!["Hi", "RSA", "Test", "Hello"];

    for text in texts {
        let message = text_to_number(text).unwrap();
        assert!(message < keypair.public_key.n, "Message {} >= n {} for text '{}'", message, keypair.public_key.n, text);
        let ciphertext = encrypt(message, &keypair.public_key);
        let decrypted = decrypt(ciphertext, &keypair.private_key);
        let recovered = number_to_text(decrypted, text.len());
        assert_eq!(text, recovered, "Failed for text: {}", text);
    }
}

#[test]
fn test_same_message_encrypts_consistently() {
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);

    let message = 100u64;
    let ciphertext1 = encrypt(message, &keypair.public_key);
    let ciphertext2 = encrypt(message, &keypair.public_key);

    // Same message should always encrypt to the same ciphertext
    assert_eq!(ciphertext1, ciphertext2);
}

#[test]
fn test_different_messages_encrypt_differently() {
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);

    let message1 = 100u64;
    let message2 = 101u64;

    let ciphertext1 = encrypt(message1, &keypair.public_key);
    let ciphertext2 = encrypt(message2, &keypair.public_key);

    // Different messages should encrypt to different ciphertexts
    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn test_keypair_independence() {
    // Generate two different keypairs
    let keypair1 = generate_keypair(61, 53);
    let keypair2 = generate_keypair(67, 71);

    let message = 100u64;

    // Encrypt with first keypair
    let ciphertext = encrypt(message, &keypair1.public_key);

    // Decrypt with first keypair should work
    let decrypted1 = decrypt(ciphertext, &keypair1.private_key);
    assert_eq!(message, decrypted1);

    // Decrypt with second keypair should NOT recover the original message
    // (unless by extreme coincidence)
    let decrypted2 = decrypt(ciphertext, &keypair2.private_key);
    assert_ne!(message, decrypted2);
}
