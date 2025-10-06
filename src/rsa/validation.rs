use super::math_utils::is_prime;

/// Validates that both numbers are prime and different
pub(super) fn validate_primes(p: u64, q: u64) -> Result<(), String> {
    if !is_prime(p) || !is_prime(q) {
        return Err("Both p and q must be prime numbers".to_string());
    }
    if p == q {
        return Err("p and q must be different primes".to_string());
    }
    Ok(())
}

/// Validates that message is smaller than modulus
pub(super) fn validate_message_size(message: u64, n: u64) -> Result<(), String> {
    if message >= n {
        Err(format!(
            "Message ({}) must be smaller than modulus n ({})",
            message, n
        ))
    } else {
        Ok(())
    }
}

