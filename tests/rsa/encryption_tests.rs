use encryption_demo::rsa::encryption::{decrypt, encrypt};
use encryption_demo::rsa::types::{RsaPrivateKey, RsaPublicKey};

#[test]
fn test_encrypt_basic() {
    let public_key = RsaPublicKey { n: 33, e: 3 };
    let message = 4u64;
    let ciphertext = encrypt(message, &public_key);
    assert_eq!(ciphertext, 31); // 4^3 mod 33 = 31
}

#[test]
fn test_decrypt_basic() {
    let private_key = RsaPrivateKey { n: 33, d: 7 };
    let ciphertext = 31u64;
    let decrypted = decrypt(ciphertext, &private_key);
    assert_eq!(decrypted, 4); // 31^7 mod 33 = 4
}

#[test]
fn test_encrypt_edge_cases() {
    let public_key = RsaPublicKey { n: 3233, e: 17 };

    // Test zero
    assert_eq!(encrypt(0, &public_key), 0);

    // Test one
    assert_eq!(encrypt(1, &public_key), 1);
}
