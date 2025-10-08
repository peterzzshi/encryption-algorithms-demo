# RSA Algorithm - Step-by-Step Mathematical Demonstration

This document explains the RSA encryption algorithm through concrete examples, showing the exact calculations at each step.

## Core Concept

RSA is a public-key cryptographic algorithm where:
- **Anyone can encrypt** using the public key `(n, e)`
- **Only you can decrypt** using the private key `(n, d)`
- Security relies on the difficulty of factoring large numbers

## The RSA Process

### Step 1: Key Generation

Given two prime numbers `p` and `q`:

1. **Calculate modulus**: `n = p × q`
2. **Calculate Euler's totient**: `φ(n) = (p-1) × (q-1)`
3. **Choose public exponent**: Pick `e` where `gcd(e, φ(n)) = 1`
4. **Calculate private exponent**: Find `d` where `e × d ≡ 1 (mod φ(n))`

**Result:**
- Public Key: `(n, e)` - shared with everyone
- Private Key: `(n, d)` - kept secret

### Step 2: Encryption

To encrypt message `m`:
```
c = m^e mod n
```

### Step 3: Decryption

To decrypt ciphertext `c`:
```
m = c^d mod n
```

---

## Complete Worked Example: Encrypting "4"

### 📝 Step 1: Key Generation

**Given:**
- `p = 3` (first prime)
- `q = 11` (second prime)

**Calculate modulus n:**
```
n = p × q = 3 × 11 = 33
```

**Calculate Euler's totient φ(n):**
```
φ(n) = (p-1) × (q-1) = (3-1) × (11-1) = 2 × 10 = 20
```

**Choose public exponent e:**

We need find `e`, such that `gcd(e, φ(n)) = gcd(e, 20) = 1`; `e = 3` suffices.

**Calculate private exponent d:**

Find `d` where `3 × d ≡ 1 (mod 20)` => `3d = 20k + 1` for some integer `k`, by testing all values up to `φ(n) - 1 = 19`.

**Try all values systematically:**

- `d = 1`: `3 × 1 mod 20 = 3` ✗
- `d = 2`: `3 × 2 mod 20 = 6` ✗
- `d = 3`: `3 × 3 mod 20 = 9` ✗
- `d = 4`: `3 × 4 mod 20 = 12` ✗
- `d = 5`: `3 × 5 mod 20 = 15` ✗
- `d = 6`: `3 × 6 mod 20 = 18` ✗
- `d = 7`: `3 × 7 mod 20 = 1` ✓

**Verify:** `(3 × 7) mod 20 = 21 mod 20 = 1` ✓

**Keys Generated:**
- 🔓 Public Key: `(n=33, e=3)`
- 🔐 Private Key: `(n=33, d=7)`

---

### 🔒 Step 2: Encryption

- **Message:** `m = 4`
- **Formula:** `c = m^e mod n = 4^3 mod 33`
- **Ciphertext:** `c = 4^3 mod 33 = 31`

---

### 🔓 Step 3: Decryption

**Ciphertext:** `c = 31`

**Formula:** `m = c^d mod n = 31^7 mod 33`

**Decrypted message:** `m = 31^7 mod 33 = 27,512,614,111 mod 33 = 4` ✓

---

## Mathematical Proof

**Euler's Theorem states:**
If `gcd(m, n) = 1`, then `m^φ(n) ≡ 1 (mod n)`

**In our case:**
- We chose `d` such that `e × d ≡ 1 (mod φ(n))`
- This means `e × d = k × φ(n) + 1` for some integer `k`

**The proof:**
```
c^d = (m^e)^d           [by definition of c]
    = m^(e×d)           [power rule]
    = m^(k×φ(n) + 1)    [substituting e×d]
    = m^(k×φ(n)) × m    [exponent rule]
    = (m^φ(n))^k × m    [power rule]
    ≡ 1^k × m           [by Euler's theorem]
    = m (mod n)         [proven!]
```

**Verify with our example:**
```
e × d = 3 × 7 = 21 = 1 × 20 + 1 = 1 × φ(n) + 1
k = 1

31^7 = 4^(3×7) = 4^21 = 4^(20+1) = 4^20 × 4 ≡ 1 × 4 = 4 (mod 33)
```

## Key Insights

RSA's security relies on a fundamental asymmetry: multiplying primes is instant (`3 × 11 = 33`), but factoring the product back is extremely hard.

Our example uses small primes (3, 11) for demonstration purposes. In real-world applications, **RSA uses 2048-bit primes or larger** (617+ digit numbers), making it computationally infeasible to crack with current technology.

---

## Try It Yourself

Run the demo program to see these calculations:

```bash
# Example 1: Encrypt the number 4
cargo run -- rsa --message 4 -p 3 -q 11

# Example 2: Encrypt text "Hi"
cargo run -- rsa --message "Hi" -p 251 -q 241

# Example 3: Larger numbers
cargo run -- rsa --message 123 -p 61 -q 53
```

---

## Tests

Unit and integration tests can be found in `/tests/rsa/`:

```bash
cargo test                                      # Run all tests
cargo test --test rsa_tests                     # Run RSA tests only
cargo test rsa::math_utils_tests                # Run specific module tests
```
