/// Common public exponents to try when generating RSA keys
///
/// These values are NOT arbitrary - they are mathematically significant:
/// - Small primes (3, 5, 7, 11, 13, 17): Fast encryption, common in embedded systems
/// - 257 (2^8 + 1): Fermat prime F3, efficient binary exponentiation
/// - 65537 (2^16 + 1): Fermat prime F4, THE INDUSTRY STANDARD
///   * Default in OpenSSL, GPG, SSH, TLS
///   * Perfect balance of speed and security
///   * Most widely used RSA exponent in production
///
/// Ordered by preference: smaller exponents = faster encryption
pub const COMMON_PUBLIC_EXPONENTS: [u64; 8] = [3, 5, 7, 11, 13, 17, 257, 65537];

/// Maximum text length (8 bytes to fit in u64)
pub const MAX_TEXT_LENGTH: usize = 8;
