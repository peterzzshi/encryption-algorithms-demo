use encryption_demo::rsa::math_utils::{extended_gcd, is_prime, mod_inverse, mod_pow};

// Extended GCD tests
#[test]
fn test_extended_gcd_basic() {
    let (gcd, x, y) = extended_gcd(3, 11);
    assert_eq!(gcd, 1);
    assert_eq!(3 * x + 11 * y, 1);
}

#[test]
fn test_extended_gcd_with_zero() {
    let (gcd, x, y) = extended_gcd(5, 0);
    assert_eq!(gcd, 5);
    assert_eq!(x, 1);
    assert_eq!(y, 0);
}

// Modular inverse tests
#[test]
fn test_mod_inverse_exists() {
    let result = mod_inverse(3, 11).unwrap();
    assert_eq!(result, 4);
    assert_eq!((3 * result) % 11, 1);
}

#[test]
fn test_mod_inverse_does_not_exist() {
    let result = mod_inverse(4, 8);
    assert!(result.is_none());
}

// Modular exponentiation tests
#[test]
fn test_mod_pow_basic() {
    assert_eq!(mod_pow(2, 3, 5), 3); // 2^3 mod 5 = 3
}

#[test]
fn test_mod_pow_zero_exponent() {
    assert_eq!(mod_pow(5, 0, 10), 1);
}

#[test]
fn test_mod_pow_large_numbers() {
    let result = mod_pow(2, 100, 1000);
    assert!(result < 1000);
}

// Prime checking tests
#[test]
fn test_is_prime_small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
}

#[test]
fn test_is_prime_non_primes() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(9));
}

#[test]
fn test_is_prime_larger_numbers() {
    assert!(is_prime(97));
    assert!(is_prime(251));
    assert!(!is_prime(100));
    assert!(!is_prime(256));
}
