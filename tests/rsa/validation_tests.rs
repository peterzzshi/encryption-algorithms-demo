// Note: validation functions are pub(super) so we test them indirectly through the demo module
use encryption_demo::rsa::demo::run_rsa_demo;

#[test]
#[should_panic(expected = "must be prime")]
fn test_validation_non_prime_p() {
    // This should panic because 4 is not prime
    run_rsa_demo(4, 4, 7);
}

#[test]
#[should_panic(expected = "must be prime")]
fn test_validation_non_prime_q() {
    // This should panic because 6 is not prime
    run_rsa_demo(4, 3, 6);
}

#[test]
#[should_panic(expected = "must be different")]
fn test_validation_same_primes() {
    // This should panic because p and q are the same
    run_rsa_demo(4, 5, 5);
}

#[test]
fn test_validation_valid_inputs() {
    // This should not panic
    run_rsa_demo(4, 3, 11);
}
