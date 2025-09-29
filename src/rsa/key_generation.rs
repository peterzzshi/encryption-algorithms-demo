use num_bigint::BigUint;
use num_traits::One;
use crate::rsa::math_utils::*;

#[derive(Clone)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

#[derive(Clone)]
pub struct RsaPublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[derive(Clone)]
pub struct RsaPrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

pub fn generate_keypair(key_size: usize) -> RsaKeyPair {
    let p = generate_prime(key_size / 2);
    let q = generate_prime(key_size / 2);
    let n = &p * &q;
    let phi_n = (&p - BigUint::one()) * (&q - BigUint::one());
    let e = BigUint::from(65537u32); // Common choice for e
    let d = mod_inverse(&e, &phi_n).expect("Failed to compute modular inverse");

    RsaKeyPair {
        public_key: RsaPublicKey { n: n.clone(), e },
        private_key: RsaPrivateKey { n, d },
    }
}

fn truncate_large_number(num: &BigUint, max_digits: usize) -> String {
    let num_str = num.to_string();
    if num_str.len() <= max_digits {
        num_str
    } else {
        format!("{}...{} ({} digits)",
                &num_str[..10],
                &num_str[num_str.len()-10..],
                num_str.len())
    }
}

pub fn print_key_generation_steps(key_pair: &RsaKeyPair, p: &BigUint, q: &BigUint, phi_n: &BigUint) {
    print_header("RSA Key Generation");
    print_step("Step 1 - Generate prime p", &truncate_large_number(p, 50));
    print_step("Step 2 - Generate prime q", &truncate_large_number(q, 50));
    print_step("Step 3 - Calculate n = p × q", &truncate_large_number(&key_pair.public_key.n, 50));
    print_step("Step 4 - Calculate φ(n) = (p-1) × (q-1)", &truncate_large_number(phi_n, 50));
    print_step("Step 5 - Choose e (commonly 65537)", &key_pair.public_key.e.to_string());
    print_step("Step 6 - Calculate d = e⁻¹ mod φ(n)", &truncate_large_number(&key_pair.private_key.d, 50));
    print_step("Public Key (n, e)", &format!("({}, {})",
              truncate_large_number(&key_pair.public_key.n, 30),
              key_pair.public_key.e));
    print_step("Private Key (n, d)", &format!("({}, {})",
              truncate_large_number(&key_pair.private_key.n, 30),
              truncate_large_number(&key_pair.private_key.d, 30)));
}