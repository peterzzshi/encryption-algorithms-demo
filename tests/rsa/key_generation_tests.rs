use encryption_demo::rsa::key_generation::generate_keypair;

#[test]
fn test_generate_keypair_basic() {
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);

    assert_eq!(keypair.public_key.n, p * q);
    assert_eq!(keypair.private_key.n, p * q);
}

#[test]
fn test_keypair_public_exponent_selection() {
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);

    // Public exponent should be one of the common values
    let common_exponents = [3, 5, 7, 11, 13, 17, 257, 65537];
    assert!(common_exponents.contains(&keypair.public_key.e));
}

#[test]
fn test_keypair_mathematical_consistency() {
    let p = 61u64;
    let q = 53u64;
    let keypair = generate_keypair(p, q);
    let phi_n = (p - 1) * (q - 1);

    // Verify e * d ≡ 1 (mod φ(n))
    let product = (keypair.public_key.e as u128 * keypair.private_key.d as u128) % phi_n as u128;
    assert_eq!(product, 1);
}
