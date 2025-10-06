use super::constants::COMMON_PUBLIC_EXPONENTS;
use super::math_utils::mod_inverse;
use super::types::{RsaKeyPair, RsaPrivateKey, RsaPublicKey};

/// Finds a suitable public exponent e and calculates private exponent d
fn find_exponent_pair(phi_n: u64) -> Option<(u64, u64)> {
    for e in COMMON_PUBLIC_EXPONENTS {
        if let Some(d) = mod_inverse(e, phi_n) {
            return Some((e, d));
        }
    }
    None
}

/// Generates RSA key pair from two primes
pub fn generate_keypair(p: u64, q: u64) -> Result<RsaKeyPair, String> {
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    let (e, d) = find_exponent_pair(phi_n).ok_or_else(|| {
        "Could not find suitable public exponent e for these primes".to_string()
    })?;

    Ok(RsaKeyPair {
        public_key: RsaPublicKey { n, e },
        private_key: RsaPrivateKey { n, d },
    })
}
