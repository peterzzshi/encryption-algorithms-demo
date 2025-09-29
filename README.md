# Encryption Algorithms Demo

A collection of cryptographic algorithm implementations in Rust, demonstrating step-by-step mathematical operations.

## Algorithms Implemented

- [RSA](./rsa/README.md) - Rivest-Shamir-Adleman public-key encryption

## Usage

```bash
# Run RSA demo
cargo run -- rsa --message "Hello World"

# Run with custom key size
cargo run -- rsa --message "Hello World" --key-size 1024
```

## Principles

This project follows functional programming principles:
- No mutable variables
- Pure calculation functions
- Side effects (printing) separated from calculations
