/// Common public exponents to try when generating RSA keys
/// These are small primes commonly used in RSA implementations
/// Ordered by preference: smaller exponents are faster for encryption
pub const COMMON_PUBLIC_EXPONENTS: [u64; 8] = [3, 5, 7, 11, 13, 17, 257, 65537];

/// Maximum text length (8 bytes to fit in u64)
pub const MAX_TEXT_LENGTH: usize = 8;
