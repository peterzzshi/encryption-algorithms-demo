use num_bigint::{BigUint, BigInt};
use num_traits::{Zero, One, Signed};
use rand::{rng, RngCore};

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

pub fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (gcd, x1, y1) = extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let a_signed = BigInt::from(a.clone());
    let m_signed = BigInt::from(m.clone());
    
    let (gcd, x, _) = extended_gcd(&a_signed, &m_signed);
    
    if gcd == BigInt::one() {
        let result = if x.is_negative() {
            x + &m_signed
        } else {
            x
        };
        Some(result.magnitude().clone())
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

pub fn miller_rabin_test(n: &BigUint, k: usize) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n <= &BigUint::from(3u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    // Write n-1 as d * 2^r
    let n_minus_1 = n - BigUint::one();
    let mut r = 0;
    let mut d = n_minus_1.clone();
    
    while &d % 2u32 == BigUint::zero() {
        d >>= 1;
        r += 1;
    }

    let mut rng = rng();
    
    // Perform k rounds of testing
    for _ in 0..k {
        // Generate random a in range [2, n-2]
        let a = loop {
            let bytes_len = ((n.bits() + 7) / 8) as usize;
            let mut bytes = vec![0u8; bytes_len];
            rng.fill_bytes(&mut bytes);
            let candidate = BigUint::from_bytes_be(&bytes) % (n - 3u32) + 2u32;
            if candidate >= BigUint::from(2u32) && candidate <= n - 2u32 {
                break candidate;
            }
        };

        let mut x = mod_pow(&a, &d, n);
        
        if x == BigUint::one() || x == n_minus_1 {
            continue;
        }

        let mut composite = true;
        for _ in 0..r-1 {
            x = mod_pow(&x, &BigUint::from(2u32), n);
            if x == n_minus_1 {
                composite = false;
                break;
            }
        }

        if composite {
            return false;
        }
    }

    true
}

pub fn is_prime(n: &BigUint) -> bool {
    miller_rabin_test(n, 10) // 10 rounds gives very high confidence
}

pub fn generate_prime(bits: usize) -> BigUint {
    let mut rng = rng();
    loop {
        // Generate random bytes for the specified bit length
        let byte_count = (bits + 7) / 8; // Round up to nearest byte
        let mut bytes = vec![0u8; byte_count];
        rng.fill_bytes(&mut bytes);

        // Ensure the number has the correct bit length
        if bits % 8 != 0 {
            let excess_bits = 8 - (bits % 8);
            bytes[0] >>= excess_bits;
        }

        // Set the most significant bit to ensure correct bit length
        if !bytes.is_empty() && bits > 0 {
            bytes[0] |= 1 << ((bits - 1) % 8);
        }

        // Ensure the number is odd (except for 2)
        if byte_count > 0 && bits > 1 {
            bytes[byte_count - 1] |= 1;
        }

        let candidate = BigUint::from_bytes_be(&bytes);

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
