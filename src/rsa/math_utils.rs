pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (gcd, x, _) = extended_gcd(a as i128, m as i128);

    if gcd == 1 {
        let result = if x < 0 {
            x + m as i128
        } else {
            x
        };
        Some(result as u64)
    } else {
        None
    }
}

pub fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result: u128 = 1;
    let mut base: u128 = (base as u128) % (modulus as u128);
    let mut exp = exp;
    let modulus = modulus as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result as u64
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3u64;
    let sqrt_n = (n as f64).sqrt() as u64;

    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

pub fn print_step(step: &str, value: &str) {
    println!("  {}: {}", step, value);
}

pub fn print_header(header: &str) {
    println!("\n=== {} ===", header);
}
