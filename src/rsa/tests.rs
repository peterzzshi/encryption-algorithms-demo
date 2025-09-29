#[cfg(test)]
mod tests {
    use crate::rsa::encryption::*;
    use crate::rsa::key_generation::*;
    use crate::rsa::math_utils::*;
    use num_bigint::BigUint;

    #[test]
    fn test_string_to_bigint_conversion() {
        let message = "Hello";
        let bigint = string_to_bigint(message);
        let recovered = bigint_to_string(&bigint);
        assert_eq!(message, recovered);
    }

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
    fn test_gcd() {
        let a = BigUint::from(48u32);
        let b = BigUint::from(18u32);
        let result = gcd(&a, &b);
        assert_eq!(result, BigUint::from(6u32));
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
    fn test_prime_generation() {
        let prime = generate_prime(16);
        assert!(is_prime(&prime));
        assert!(prime.bits() >= 15 && prime.bits() <= 16);
    }
}
