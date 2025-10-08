# SHA-256 Algorithm

SHA-256 (Secure Hash Algorithm 256-bit) is a cryptographic hash function that produces a 256-bit (32-byte) hash value, typically rendered as a 64-character hexadecimal string.

## Mathematical Steps

### 1. Message Preprocessing

1. **Padding**: Append a single '1' bit to the message
2. **Zero Padding**: Add zeros until message length ≡ 448 (mod 512) bits
3. **Length Field**: Append original message length as 64-bit big-endian integer
4. **Block Division**: Divide into 512-bit (64-byte) blocks

### 2. Initial Hash Values

Eight 32-bit words derived from fractional parts of square roots of first 8 primes:
```
H₀ = 0x6a09e667  (√2)
H₁ = 0xbb67ae85  (√3)
H₂ = 0x3c6ef372  (√5)
H₃ = 0xa54ff53a  (√7)
H₄ = 0x510e527f  (√11)
H₅ = 0x9b05688c  (√13)
H₆ = 0x1f83d9ab  (√17)
H₇ = 0x5be0cd19  (√19)
```

### 3. Message Schedule

For each 512-bit block, create 64 32-bit words W[0..63]:
- First 16 words come directly from the block
- Remaining 48 words: `W[t] = γ₁(W[t-2]) + W[t-7] + γ₀(W[t-15]) + W[t-16]`

### 4. Compression Function

For each block, perform 64 rounds using:
- **Constants**: K[0..63] (first 32 bits of fractional parts of cube roots of first 64 primes)
-

**Working Variables**: a, b, c, d, e, f, g, h (initialized to current hash)

**Round Function**:
```
T₁ = h + Σ₁(e) + Ch(e,f,g) + K[t] + W[t]
T₂ = Σ₀(a) + Maj(a,b,c)

h = g, g = f, f = e, e = d + T₁
d = c, c = b, b = a, a = T₁ + T₂
```

**Logical Functions**:
- `Ch(x,y,z) = (x ∧ y) ⊕ (¬x ∧ z)`
- `Maj(x,y,z) = (x ∧ y) ⊕ (x ∧ z) ⊕ (y ∧ z)`
- `Σ₀(x) = ROTR²(x) ⊕ ROTR¹³(x) ⊕ ROTR²²(x)`
- `Σ₁(x) = ROTR⁶(x) ⊕ ROTR¹¹(x) ⊕ ROTR²⁵(x)`
- `γ₀(x) = ROTR⁷(x) ⊕ ROTR¹⁸(x) ⊕ SHR³(x)`
- `γ₁(x) = ROTR¹⁷(x) ⊕ ROTR¹⁹(x) ⊕ SHR¹⁰(x)`

### 5. Final Hash

After processing all blocks:
```
H₀ = H₀ + a, H₁ = H₁ + b, H₂ = H₂ + c, H₃ = H₃ + d
H₄ = H₄ + e, H₅ = H₅ + f, H₆ = H₆ + g, H₇ = H₇ + h
```

Final hash = H₀ || H₁ || H₂ || H₃ || H₄ || H₅ || H₆ || H₇

## Properties

- **Deterministic**: Same input always produces same output
- **Fixed Output**: Always 256 bits regardless of input size
- **Avalanche Effect**: Small input change drastically changes output
- **One-Way**: Computationally infeasible to reverse
- **Collision Resistant**: Hard to find two inputs with same hash

## Security

SHA-256 is cryptographically secure and widely used in:
- Bitcoin blockchain
- Digital signatures
- Password hashing (with salt)
- Data integrity verification
- TLS/SSL certificates

## Implementation Notes

- Uses big-endian byte ordering
- All arithmetic is modulo 2³²
- Round constants derived from cube roots of first 64 primes
- Optimized for 32-bit processors


## Usage Examples

```bash
# SHA-256 with text
cargo run -- sha256 --message "hello"

# SHA-256 with raw bytes (hex)
cargo run -- sha256 --bytes "48656c6c6f"

# RSA (existing)
cargo run -- rsa --message "Hi" --p 61 --q 53
```
