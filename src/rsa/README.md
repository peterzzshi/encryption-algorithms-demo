# RSA Algorithm

RSA (Rivest-Shamir-Adleman) is a public-key cryptographic algorithm that relies on the mathematical difficulty of factoring large composite numbers.

## Mathematical Steps

### 1. Key Generation

1. **Generate two large prime numbers**: `p` and `q`
2. **Calculate modulus**: `n = p × q`
3. **Calculate Euler's totient**: `φ(n) = (p-1) × (q-1)`
4. **Choose public exponent**: `e` such that `1 < e < φ(n)` and `gcd(e, φ(n)) = 1`
    - Common choice: `e = 65537`
5. **Calculate private exponent**: `d = e⁻¹ mod φ(n)`

**Public Key**: `(n, e)`  
**Private Key**: `(n, d)`

### 2. Encryption

To encrypt a message `m`:
```
c = m^e mod n
```

Where:
- `m` is the plaintext message (as integer)
- `c` is the ciphertext
- `e` and `n` are from the public key

### 3. Decryption

To decrypt ciphertext `c`:
```
m = c^d mod n
```

Where:
- `c` is the ciphertext
- `m` is the recovered plaintext
- `d` and `n` are from the private key

## Security

The security of RSA relies on the computational difficulty of:
- **Integer factorization**: Finding `p` and `q` given `n = p × q`
- **RSA problem**: Computing `m` from `c = m^e mod n` without knowing `d`

## Implementation Details

### Key Components

- **Prime Generation**: Uses trial division for primality testing
- **Modular Exponentiation**: Efficient computation using binary exponentiation
- **Extended Euclidean Algorithm**: For computing modular inverse
- **Message Encoding**: Converts strings to integers using big-endian byte representation