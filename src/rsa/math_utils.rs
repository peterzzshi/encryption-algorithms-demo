pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

/// Calculate modular inverse: a^(-1) mod m
/// Returns None if inverse doesn't exist (when gcd(a, m) ≠ 1)
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (gcd, x, _) = extended_gcd(a as i128, m as i128);

    if gcd == 1 {
        let result = if x < 0 { x + m as i128 } else { x };
        Some(result as u64)
    } else {
        None
    }
}

pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    // Use immutable shadowing for functional style while keeping efficiency
    let modulus = modulus as u128;
    let mut result = 1u128;
    let mut base = (base as u128) % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }

    result as u64
}

pub fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        n => is_prime_helper(n, 3, (n as f64).sqrt() as u64),
    }
}

fn is_prime_helper(n: u64, divisor: u64, sqrt_n: u64) -> bool {
    if divisor > sqrt_n {
        true
    } else if n % divisor == 0 {
        false
    } else {
        is_prime_helper(n, divisor + 2, sqrt_n)
    }
}
