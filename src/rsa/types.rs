// ============================================================================
// RSA Cryptographic Types
// ============================================================================

/// RSA key pair containing both public and private keys
#[derive(Clone)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

/// RSA public key used for encryption
#[derive(Clone)]
pub struct RsaPublicKey {
    pub n: u64,
    pub e: u64,
}

/// RSA private key used for decryption
#[derive(Clone)]
pub struct RsaPrivateKey {
    pub n: u64,
    pub d: u64,
}
