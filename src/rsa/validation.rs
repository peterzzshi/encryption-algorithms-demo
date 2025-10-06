use super::math_utils::is_prime;

pub(super) fn validate_primes(p: u64, q: u64) {
    assert!(
        is_prime(p) && is_prime(q),
        "Both p={} and q={} must be prime numbers",
        p,
        q
    );
    assert_ne!(p, q, "p and q must be different (got p={}, q={})", p, q);
}


pub(super) fn is_valid_message_size(message: u64, n: u64) -> bool {
    message < n
}
