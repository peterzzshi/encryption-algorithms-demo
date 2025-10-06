use super::constants::COMMON_PUBLIC_EXPONENTS;
use super::math_utils::mod_inverse;
use super::types::{RsaKeyPair, RsaPrivateKey, RsaPublicKey};


fn find_exponent_pair(phi_n: u64) -> Option<(u64, u64)> {
    COMMON_PUBLIC_EXPONENTS
        .iter()
        .find_map(|&e| mod_inverse(e, phi_n).map(|d| (e, d)))
}

pub fn generate_keypair(p: u64, q: u64) -> RsaKeyPair {
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    let (e, d) = find_exponent_pair(phi_n).unwrap_or_else(|| {
        panic!("Could not find suitable public exponent for φ(n)={}. Try different primes.", phi_n)
    });

    RsaKeyPair {
        public_key: RsaPublicKey { n, e },
        private_key: RsaPrivateKey { n, d },
    }
}
