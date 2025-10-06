#[cfg(test)]
mod tests {
    use crate::rsa::encryption::*;
    use crate::rsa::key_generation::*;
    use crate::rsa::math_utils::*;

    #[test]
    fn test_mod_inverse() {
        let a = 3u64;
        let m = 11u64;
        let result = mod_inverse(a, m).unwrap();
        assert_eq!(result, 4); // 3 * 4 ≡ 1 (mod 11)
    }

    #[test]
    fn test_mod_pow() {
        let base = 2u64;
        let exp = 3u64;
        let modulus = 5u64;
        let result = mod_pow(base, exp, modulus);
        assert_eq!(result, 3); // 2^3 mod 5 = 8 mod 5 = 3
    }

    #[test]
    fn test_rsa_encryption_decryption() {
        // Use small primes for testing
        let p = 61u64;
        let q = 53u64;
        let n = p * q;
        let phi_n = (p - 1) * (q - 1);
        let e = 17u64;
        let d = mod_inverse(e, phi_n).unwrap();

        let public_key = RsaPublicKey { n, e };
        let private_key = RsaPrivateKey { n, d };

        let message = 123u64;
        let ciphertext = encrypt(message, &public_key);
        let decrypted = decrypt(ciphertext, &private_key);

        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));

        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
    }

    #[test]
    fn test_small_rsa_example() {
        // Test with very small primes like in the README
        let p = 3u64;
        let q = 11u64;
        let n = p * q; // 33
        let phi_n = (p - 1) * (q - 1); // 20
        let e = 3u64;
        let d = mod_inverse(e, phi_n).unwrap(); // Should be 7

        let public_key = RsaPublicKey { n, e };
        let private_key = RsaPrivateKey { n, d };

        let message = 4u64;
        let ciphertext = encrypt(message, &public_key);
        assert_eq!(ciphertext, 31); // 4^3 mod 33 = 31

        let decrypted = decrypt(ciphertext, &private_key);
        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_text_to_number() {
        let text = "Hi";
        let number = text_to_number(text).unwrap();
        let recovered = number_to_text(number, text.len());
        assert_eq!(text, recovered);
    }

    #[test]
    fn test_text_encryption() {
        // Use larger primes for text encryption
        let p = 251u64;
        let q = 241u64;
        let n = p * q;
        let phi_n = (p - 1) * (q - 1);
        let e = 7u64;
        let d = mod_inverse(e, phi_n).unwrap();

        let public_key = RsaPublicKey { n, e };
        let private_key = RsaPrivateKey { n, d };

        let text = "Hi";
        let message = text_to_number(text).unwrap();
        let ciphertext = encrypt(message, &public_key);
        let decrypted = decrypt(ciphertext, &private_key);
        let recovered_text = number_to_text(decrypted, text.len());

        assert_eq!(message, decrypted);
        assert_eq!(text, recovered_text);
    }
}
