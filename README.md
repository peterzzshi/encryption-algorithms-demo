# Encryption Algorithms Demo

A hands-on educational project demonstrating cryptographic algorithms in Rust with step-by-step mathematical explanations.

## 🎯 Overview

This project implements cryptographic algorithms from scratch, showing each computational step to help understand how they work under the hood. Perfect for learning cryptography fundamentals.

**Algorithms Implemented:**
- **RSA** - Public-key encryption (encryption/decryption)
- **SHA-256** - Cryptographic hash function

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/peterzzshi/encryption-algorithms-demo.git
cd encryption-algorithms-demo
cargo build --release
```

### Running Demos

#### RSA Encryption

```bash
# Encrypt a number
cargo run -- rsa --message 42 -p 61 -q 53

# Encrypt text
cargo run -- rsa --message "Hi" -p 251 -q 241

# Larger primes for better security demo
cargo run -- rsa --message "Hello" -p 65521 -q 65519
```

**Parameters:**
- `-m, --message` - Message to encrypt (number or text)
- `-p` - First prime number
- `-q` - Second prime number

#### SHA-256 Hashing

```bash
# Hash text
cargo run -- sha256 --message "Hello, World!"

# Hash hex bytes
cargo run -- sha256 --message "48656c6c6f"
```

**Parameters:**
- `-m, --message` - Message to hash (text or hex string)

## 📁 Project Structure

```
encryption-algorithms-demo/
├── src/
│   ├── common/              # Shared utilities
│   │   ├── output.rs        # Hex formatting
│   │   └── validation.rs    # Input validation
│   ├── rsa/                 # RSA implementation
│   │   ├── constants.rs     # Public exponents
│   │   ├── encryption.rs    # Encrypt/decrypt
│   │   ├── key_generation.rs
│   │   ├── math_utils.rs    # Modular arithmetic
│   │   ├── text_encoding.rs
│   │   ├── types.rs         # RSA types
│   │   ├── validation.rs    # Prime validation
│   │   ├── output.rs        # Display formatting
│   │   ├── demo.rs          # Demo runner
│   │   └── README.md        # RSA explanation
│   ├── sha256/              # SHA-256 implementation
│   │   ├── constants.rs     # K constants, initial hash
│   │   ├── compression.rs   # Compression function
│   │   ├── preprocessing.rs # Message padding
│   │   ├── math_utils.rs    # Bitwise operations
│   │   ├── types.rs         # SHA types
│   │   ├── validation.rs    # Message validation
│   │   ├── output.rs        # Display formatting
│   │   ├── demo.rs          # Demo runner
│   │   └── README.md        # SHA-256 explanation
│   ├── lib.rs               # Library entry
│   └── main.rs              # CLI entry
├── tests/
│   ├── common/              # Common utility tests (5 tests)
│   └── rsa/                 # RSA tests (33 tests)
└── README.md                # This file
```

## 🏗️ Architecture

### Design Principles

This project follows **functional programming principles**:
- ✅ Pure calculation functions (no mutations)
- ✅ Side effects (printing) separated from calculations
- ✅ Algorithm-specific code isolated in modules
- ✅ Minimal shared utilities in `common/`

### Module Organization

```
┌─────────────────────────────────────────────────────────┐
│  src/common/                                            │
│  • Shared utilities (validation, hex formatting)       │
└─────────────────────────────────────────────────────────┘
                         ▲          ▲
                         │          │
            ┌────────────┴──────────┴────────────┐
            │                                    │
┌───────────▼─────────────┐        ┌─────────────▼───────────┐
│  src/rsa/               │        │  src/sha256/            │
│  • Modular arithmetic   │        │  • Bitwise operations   │
│  • Key generation       │        │  • Message padding      │
│  • Encrypt/decrypt      │        │  • Compression function │
│  • Text encoding        │        │  • Hash computation     │
└─────────────────────────┘        └─────────────────────────┘
```

### What's Common vs Algorithm-Specific

**Common (`src/common/`):**
- `format_bytes_as_hex()` - Hex string conversion
- `validate_non_empty_message()` - Empty checks
- `validate_message_length()` - Length validation
- `is_printable_ascii()` - ASCII validation

**Algorithm-Specific (stays separate):**
- `math_utils.rs` - Different per algorithm (modular arithmetic vs bitwise ops)
- `constants.rs` - Algorithm parameters (exponents vs K values)
- `types.rs` - Algorithm-specific types (KeyPair vs Hash)
- `validation.rs` - Algorithm rules (prime checks vs message format)

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Run specific test suites:
```bash
cargo test --test rsa_tests        # RSA tests only
cargo test --test common_tests     # Common utility tests
cargo test rsa::math_utils_tests   # Specific module
```

## 📚 Documentation

Each algorithm has detailed documentation:

- **[RSA README](src/rsa/README.md)** - Complete mathematical walkthrough with worked examples
- **[SHA-256 README](src/sha256/README.md)** - Hash algorithm explanation

## 🔑 Key Insights

### RSA
- **Demo uses small primes** (3, 11, 61, etc.) for educational clarity
- **Real-world uses 2048-bit primes** (617+ digits) - computationally infeasible to crack
- Demonstrates asymmetric cryptography: public key encrypts, private key decrypts

### SHA-256
- **One-way function** - cannot reverse the hash to get original message
- **Fixed output size** - always 256 bits (64 hex characters)
- Used in Bitcoin, TLS, digital signatures

## 📝 License

MIT License - See [LICENSE](LICENSE) file for details
