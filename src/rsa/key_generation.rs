#[derive(Clone)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

#[derive(Clone)]
pub struct RsaPublicKey {
    pub n: u64,
    pub e: u64,
}

#[derive(Clone)]
pub struct RsaPrivateKey {
    pub n: u64,
    pub d: u64,
}
