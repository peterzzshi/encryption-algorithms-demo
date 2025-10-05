use num_bigint::BigUint;
use num_traits::One;
use crate::rsa::key_generation::*;
use crate::rsa::encryption::*;
use crate::rsa::math_utils::*;

pub fn run_rsa_demo(message: u64, p_val: u64, q_val: u64) {
    println!("🔐 RSA Encryption Algorithm Demo");
    println!("Message (m): {}", message);
    println!("Prime p: {}", p_val);
    println!("Prime q: {}", q_val);

    let p = BigUint::from(p_val);
    let q = BigUint::from(q_val);

    // Validate that both are prime
    if !is_prime(&p) {
        eprintln!("\n❌ Error: p = {} is not a prime number", p_val);
        eprintln!("💡 Tip: Good small primes: 3, 5, 7, 11, 13, 17, 19, 23, 29, 31");
        return;
    }
    if !is_prime(&q) {
        eprintln!("\n❌ Error: q = {} is not a prime number", q_val);
        eprintln!("💡 Tip: Good small primes: 3, 5, 7, 11, 13, 17, 19, 23, 29, 31");
        return;
    }
    if p == q {
        eprintln!("\n❌ Error: p and q must be different primes");
        return;
    }

    println!("\n⏳ Generating RSA key pair...");

    let n = &p * &q;
    let phi_n = (&p - BigUint::one()) * (&q - BigUint::one());

    // Try small public exponents: e=3, 5, 7, 17
    let (e, d) = if let Some(d) = mod_inverse(&BigUint::from(3u32), &phi_n) {
        (BigUint::from(3u32), d)
    } else if let Some(d) = mod_inverse(&BigUint::from(5u32), &phi_n) {
        (BigUint::from(5u32), d)
    } else if let Some(d) = mod_inverse(&BigUint::from(7u32), &phi_n) {
        (BigUint::from(7u32), d)
    } else if let Some(d) = mod_inverse(&BigUint::from(17u32), &phi_n) {
        (BigUint::from(17u32), d)
    } else {
        eprintln!("\n❌ Error: Could not find suitable public exponent e for these primes");
        eprintln!("💡 Tip: Try different primes. Good examples: 3, 5, 7, 11, 13, 17, 19, 23");
        return;
    };

    let key_pair = RsaKeyPair {
        public_key: RsaPublicKey { n: n.clone(), e },
        private_key: RsaPrivateKey { n, d },
    };

    let message_int = BigUint::from(message);

    // Ensure message is smaller than modulus
    if message_int >= key_pair.public_key.n {
        eprintln!("\n❌ Error: Message ({}) must be smaller than modulus n ({})",
                 message, key_pair.public_key.n);
        eprintln!("💡 Tip: Use larger primes or a smaller message");
        return;
    }

    // Print key generation steps
    print_key_generation_steps(&key_pair, &p, &q, &phi_n);

    // Encrypt with detailed steps
    let ciphertext = encrypt(&message_int, &key_pair.public_key);
    print_encryption_steps(&message_int, &ciphertext, &key_pair.public_key);

    // Decrypt with detailed steps
    let decrypted_int = decrypt(&ciphertext, &key_pair.private_key);
    print_decryption_steps(&ciphertext, &decrypted_int, &key_pair.private_key);

    // Verification
    print_header("Verification");
    let success = message_int == decrypted_int;
    print_step("Original message", &message_int.to_string());
    print_step("Decrypted message", &decrypted_int.to_string());
    print_step("Match", &success.to_string());

    if success {
        println!("\n✅ RSA encryption/decryption successful!");
    } else {
        println!("\n❌ RSA encryption/decryption failed!");
    }

    // Educational note
    println!();
    println!("📚 Mathematical Foundation:");
    println!("RSA works because of Euler's theorem: if gcd(m,n)=1, then m^φ(n) ≡ 1 (mod n)");
    println!("Since e×d ≡ 1 (mod φ(n)), we have: m^(e×d) ≡ m (mod n)");
}

fn print_key_generation_steps(key_pair: &RsaKeyPair, p: &BigUint, q: &BigUint, phi_n: &BigUint) {
    print_header("RSA Key Generation");
    print_step("Step 1 - Prime p", &p.to_string());
    print_step("Step 2 - Prime q", &q.to_string());
    print_step("Step 3 - Calculate n = p × q", &key_pair.public_key.n.to_string());
    print_step("Step 4 - Calculate φ(n) = (p-1) × (q-1)", &phi_n.to_string());
    print_step("Step 5 - Choose e", &key_pair.public_key.e.to_string());

    // Show the calculation for finding d
    println!("  Step 6 - Calculate d = e⁻¹ mod φ(n):");
    println!("    Need: {} × d ≡ 1 (mod {})", key_pair.public_key.e, phi_n);
    println!("    Solution: d = {}", key_pair.private_key.d);
    println!("    Verification: {} × {} mod {} = {}",
             key_pair.public_key.e,
             key_pair.private_key.d,
             phi_n,
             (&key_pair.public_key.e * &key_pair.private_key.d) % phi_n);

    print_step("Public Key (n, e)", &format!("({}, {})", key_pair.public_key.n, key_pair.public_key.e));
    print_step("Private Key (n, d)", &format!("({}, {})", key_pair.private_key.n, key_pair.private_key.d));
}

fn print_encryption_steps(message: &BigUint, ciphertext: &BigUint, public_key: &RsaPublicKey) {
    print_header("RSA Encryption");
    print_step("Message (m)", &message.to_string());
    print_step("Public key (n)", &public_key.n.to_string());
    print_step("Public key (e)", &public_key.e.to_string());
    print_step("Formula", "c = m^e mod n");

    // Show intermediate calculation if numbers are small enough
    let e_u64_opt = public_key.e.to_u64_digits();
    if e_u64_opt.len() == 1 && e_u64_opt[0] <= 100 && message.to_string().len() < 20 {
        let e_val = e_u64_opt[0];
        println!("  Calculation: c = {}^{} mod {}", message, e_val, public_key.n);

        // Calculate m^e without mod first if it's reasonable
        if let Some(msg_u64) = message.to_u64_digits().get(0) {
            if *msg_u64 < 1000 && e_val <= 10 {
                let power_result = msg_u64.pow(e_val as u32);
                println!("    Step 1: {}^{} = {}", msg_u64, e_val, power_result);
                println!("    Step 2: {} mod {} = {}", power_result, public_key.n, ciphertext);
            }
        }
    }

    print_step("Ciphertext (c)", &ciphertext.to_string());
}

fn print_decryption_steps(ciphertext: &BigUint, decrypted: &BigUint, private_key: &RsaPrivateKey) {
    print_header("RSA Decryption");
    print_step("Ciphertext (c)", &ciphertext.to_string());
    print_step("Private key (n)", &private_key.n.to_string());
    print_step("Private key (d)", &private_key.d.to_string());
    print_step("Formula", "m = c^d mod n");
    println!("  Calculation: m = {}^{} mod {}", ciphertext, private_key.d, private_key.n);
    println!("    (Using modular exponentiation for efficiency)");
    print_step("Decrypted message (m)", &decrypted.to_string());
}
