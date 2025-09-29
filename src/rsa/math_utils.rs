use num_bigint::{BigUint};
use num_traits::{Zero, One, ToPrimitive};
use rand::{rng};

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

pub fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
    if b.is_zero() {
        (a.clone(), BigUint::one(), BigUint::zero())
    } else {
        let (gcd, x1, y1) = extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = if x1 >= (a / b) * &y1 {
            x1 - (a / b) * y1
        } else {
            // Handle underflow by working in modular arithmetic
            let modulus = &gcd;
            (x1 + modulus - ((a / b) * y1) % modulus) % modulus
        };
        (gcd, x, y)
    }
}

pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd == BigUint::one() {
        Some(x % m)
    } else {
        None
    }
}

pub fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    if modulus == &BigUint::one() {
        return BigUint::zero();
    }

    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();

    while exp > BigUint::zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }

    result
}

pub fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() || n % 3u32 == BigUint::zero() {
        return false;
    }

    let mut i = BigUint::from(5u32);
    while &i * &i <= *n {
        if n % &i == BigUint::zero() || n % (&i + 2u32) == BigUint::zero() {
            return false;
        }
        i += 6u32;
    }

    true
}

pub fn generate_prime(bits: usize) -> BigUint {
    let rng = rng();
    loop {
        let candidate = rand::rng().gen_biguint(bits);
        if is_prime(&candidate) {
            return candidate;
        }
    }
}

pub fn print_step(step: &str, value: &str) {
    println!("  {}: {}", step, value);
}

pub fn print_header(header: &str) {
    println!("\n=== {} ===", header);
}
