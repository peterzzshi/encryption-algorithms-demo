#[cfg(test)]
mod tests {
    use crate::rsa::encryption::*;
    use crate::rsa::key_generation::*;
    use crate::rsa::math_utils::*;
    use num_bigint::BigUint;

    #[test]
    fn test_mod_inverse() {
        let a = BigUint::from(3u32);
        let m = BigUint::from(11u32);
        let result = mod_inverse(&a, &m).unwrap();
        assert_eq!(result, BigUint::from(4u32)); // 3 * 4 ≡ 1 (mod 11)
    }

    #[test]
    fn test_mod_pow() {
        let base = BigUint::from(2u32);
        let exp = BigUint::from(3u32);
        let modulus = BigUint::from(5u32);
        let result = mod_pow(&base, &exp, &modulus);
        assert_eq!(result, BigUint::from(3u32)); // 2^3 mod 5 = 8 mod 5 = 3
    }

    #[test]
    fn test_rsa_encryption_decryption() {
        // Use small primes for testing
        let p = BigUint::from(61u32);
        let q = BigUint::from(53u32);
        let n = &p * &q;
        let phi_n = (&p - 1u32) * (&q - 1u32);
        let e = BigUint::from(17u32);
        let d = mod_inverse(&e, &phi_n).unwrap();

        let public_key = RsaPublicKey { n: n.clone(), e };
        let private_key = RsaPrivateKey { n, d };

        let message = BigUint::from(123u32);
        let ciphertext = encrypt(&message, &public_key);
        let decrypted = decrypt(&ciphertext, &private_key);

        assert_eq!(message, decrypted);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(&BigUint::from(2u32)));
        assert!(is_prime(&BigUint::from(3u32)));
        assert!(is_prime(&BigUint::from(5u32)));
        assert!(is_prime(&BigUint::from(7u32)));
        assert!(is_prime(&BigUint::from(11u32)));
        assert!(is_prime(&BigUint::from(13u32)));

        assert!(!is_prime(&BigUint::from(1u32)));
        assert!(!is_prime(&BigUint::from(4u32)));
        assert!(!is_prime(&BigUint::from(6u32)));
        assert!(!is_prime(&BigUint::from(8u32)));
        assert!(!is_prime(&BigUint::from(9u32)));
        assert!(!is_prime(&BigUint::from(10u32)));
    }

    #[test]
    fn test_small_rsa_example() {
        // Test with very small primes like in the README
        let p = BigUint::from(3u32);
        let q = BigUint::from(11u32);
        let n = &p * &q; // 33
        let phi_n = (&p - 1u32) * (&q - 1u32); // 20
        let e = BigUint::from(3u32);
        let d = mod_inverse(&e, &phi_n).unwrap(); // Should be 7

        let public_key = RsaPublicKey { n: n.clone(), e };
        let private_key = RsaPrivateKey { n, d };

        let message = BigUint::from(4u32);
        let ciphertext = encrypt(&message, &public_key);
        assert_eq!(ciphertext, BigUint::from(31u32)); // 4^3 mod 33 = 31

        let decrypted = decrypt(&ciphertext, &private_key);
        assert_eq!(message, decrypted);
    }
}
