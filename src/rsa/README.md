# RSA Algorithm

RSA (Rivest-Shamir-Adleman) is a public-key cryptographic algorithm that relies on the mathematical difficulty of factoring large composite numbers.

## Mathematical Foundation

### 1. Key Generation

Given two distinct prime numbers `p` and `q`:

1. **Calculate modulus**: `n = p × q`
2. **Calculate Euler's totient function**: `φ(n) = (p-1) × (q-1)`
3. **Choose public exponent**: Select `e` such that `1 < e < φ(n)` and `gcd(e, φ(n)) = 1`
4. **Calculate private exponent**: Find `d` such that `e × d ≡ 1 (mod φ(n))`
   - This is computed using the Extended Euclidean Algorithm
   - Equivalently: `d = e^(-1) mod φ(n)`

**Result:**
- Public Key: `(n, e)`
- Private Key: `(n, d)`

### 2. Encryption

To encrypt a message `m` where `0 ≤ m < n`:

```
c = m^e mod n
```

Where:
- `m` is the plaintext message (as an integer)
- `c` is the ciphertext
- `e` and `n` are from the public key

### 3. Decryption

To decrypt a ciphertext `c`:

```
m = c^d mod n
```

Where:
- `c` is the ciphertext
- `m` is the recovered plaintext
- `d` and `n` are from the private key

### 4. Correctness Proof

The RSA algorithm works because of **Euler's theorem**:

If `gcd(m, n) = 1`, then:
```
m^φ(n) ≡ 1 (mod n)
```

Since `e × d ≡ 1 (mod φ(n))`, we can write:
```
e × d = k × φ(n) + 1
```

for some integer `k`.

Therefore:
```
c^d = (m^e)^d = m^(e×d) = m^(k×φ(n)+1) = (m^φ(n))^k × m ≡ 1^k × m = m (mod n)
```

This proves that decryption recovers the original message.

## Worked Example with Small Numbers

### Step 1: Key Generation

1. **Choose primes**: `p = 3`, `q = 11`
2. **Calculate modulus**: `n = 3 × 11 = 33`
3. **Calculate Euler's totient**: `φ(n) = (3-1) × (11-1) = 2 × 10 = 20`
4. **Choose public exponent**: `e = 3` (since `gcd(3, 20) = 1`)
5. **Calculate private exponent**: Find `d` such that `3 × d ≡ 1 (mod 20)`
   - We need to solve: `3d = 20k + 1` for integers `d` and `k`
   - Using the Extended Euclidean Algorithm or by inspection:
   - Testing: `3 × 7 = 21 = 20 × 1 + 1 ≡ 1 (mod 20)`
   - Therefore `d = 7`

**Keys:**
- Public Key: `(n=33, e=3)`
- Private Key: `(n=33, d=7)`

### Step 2: Encryption

Encrypt message `m = 4`:
```
c = m^e mod n = 4^3 mod 33
```

Calculation:
- `4^3 = 64`
- `64 = 1 × 33 + 31`
- Therefore `c = 31`

### Step 3: Decryption

Decrypt ciphertext `c = 31`:
```
m = c^d mod n = 31^7 mod 33
```

Using modular exponentiation with **binary method** (since 7 = 4 + 2 + 1 = 2² + 2¹ + 2⁰):

**Step 3.1: Build powers of 31 (mod 33)**
- `31^1 = 31 mod 33 = 31`
- `31^2 = 961 mod 33 = 4` (since `961 = 29 × 33 + 4`)
- `31^4 = (31^2)^2 = 4^2 mod 33 = 16`

**Step 3.2: Combine using binary representation**
- `31^7 = 31^4 × 31^2 × 31^1`
- `= 16 × 4 × 31 (mod 33)`

**Step 3.3: Calculate step by step**
- `16 × 4 = 64`
- `64 mod 33 = 31` (since `64 = 1 × 33 + 31`)
- `31 × 31 = 961`
- `961 mod 33 = 4` (since `961 = 29 × 33 + 4`)
- Therefore `m = 4` ✓

### Verification

**Method 1: Direct calculation**
- `31^7 = 27,512,614,111`
- `27,512,614,111 = 833,715,276 × 33 + 4`
- Remainder is `4`, confirming our result

**Method 2: Using Euler's theorem**
- We have `e × d = 3 × 7 = 21 = 1 × 20 + 1 = 1 × φ(n) + 1`
- Therefore: `c^d = (m^e)^d = m^(e×d) = m^(φ(n)+1) = m^φ(n) × m ≡ 1 × m = m (mod n)`
- Since `gcd(4, 33) = 1`, Euler's theorem guarantees `4^20 ≡ 1 (mod 33)`

## Running the Demo Program

This repository includes a Rust implementation that demonstrates the RSA algorithm with user-provided parameters.

### Usage

**Number Encryption (for mathematical demonstration):**
```bash
cargo run -- rsa --message <M> -p <P> -q <Q>
```

**Text Encryption (for practical demonstration):**
```bash
cargo run -- rsa-text --text <TEXT> -p <P> -q <Q>
```

Where:
- `<M>` is the message as a number
- `<TEXT>` is a text string (maximum 8 characters)
- `<P>` is the first prime number
- `<Q>` is the second prime number

### Examples

**1. Running the worked example from above:**
```bash
cargo run -- rsa --message 4 -p 3 -q 11
```

**2. Slightly larger numbers:**
```bash
cargo run -- rsa --message 8 -p 5 -q 7
```

**3. Text encryption with small message:**
```bash
cargo run -- rsa-text --text "Hi" -p 251 -q 241
```

**4. Text encryption with longer message:**
```bash
cargo run -- rsa-text --text "Hello" -p 997 -q 991
```

**Note:** 
- For number encryption: message must be smaller than `n = p × q`
- For text encryption: text is converted to a number, which must also be smaller than `n`
- Larger primes are needed for text encryption (e.g., 251, 241, 499, 503, 997, 991)

## Text Encoding

### How Text is Converted to Numbers

RSA operates on numbers, so text must be encoded:

1. **Character to Bytes**: Each character is converted to its ASCII/UTF-8 byte value
   - Example: 'H' = 72, 'i' = 105

2. **Bytes to Number**: Bytes are combined into a single number using bit shifting
   - "Hi" = (72 << 8) | 105 = 18432 + 105 = 18537

3. **Encryption**: The number is encrypted using RSA
   - `c = 18537^e mod n`

4. **Decryption**: The number is decrypted
   - `m = c^d mod n = 18537`

5. **Number to Text**: The number is converted back to text
   - 18537 → [72, 105] → "Hi"

**Limitations:**
- Maximum 8 characters (64 bits for u64)
- Message number must be smaller than modulus `n`

## Security Assumptions

RSA security relies on the computational hardness of:

1. **Integer Factorization Problem**: Given `n = p × q`, it is computationally infeasible to find `p` and `q` for sufficiently large primes (typically 1024+ bits)
2. **RSA Problem**: Computing the `d`-th root of `c` modulo `n` without knowing the factorization of `n`

### Why RSA is Secure

The security of RSA depends on the fact that:
- **Public information**: `n` and `e` are publicly known
- **Computing `d` is hard**: To compute `d = e^(-1) mod φ(n)`, you need to know `φ(n)`
- **Computing `φ(n)` is hard**: To compute `φ(n) = (p-1)(q-1)`, you need to know `p` and `q`
- **Factoring `n` is hard**: Finding `p` and `q` given only `n = pq` is computationally infeasible for large primes

**Note:** The small numbers in the examples above are for educational purposes only. Production RSA implementations use primes of at least 1024 bits (preferably 2048 or 4096 bits) to ensure security.

## Implementation Details

This implementation uses:
- **Native Rust types**: `u64` for prime numbers and messages, `u128` for intermediate calculations
- **Zero external dependencies** (except `clap` for CLI): No BigInt library needed for educational examples
- **Pure functions**: Core logic separated from I/O for clarity and testability
- **Trial division**: Simple primality testing suitable for small educational primes
