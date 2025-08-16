# rsa-encryption-decryption

This repository demonstrates RSA encryption/decryption and digital signature in Rust.  
You can easily switch between encryption and signature features using command-line arguments.

## Features

- RSA encryption and decryption
- RSA digital signature and verification
- Interactive console input for messages
- Modular code structure for clarity

## Usage

### Build & Run

```sh
cargo run -- crypto
```
Runs the encryption/decryption demo.

```sh
cargo run -- signature
```
Runs the signature/verification demo.

## Project Structure

- `src/rsa_crypto.rs` – RSA encryption/decryption logic
- `src/rsa_signature.rs` – RSA signature/verification logic
- `src/main.rs` – Command-line interface
- `src/lib.rs` – Module definitions

## References

- [Implementing RSA encryption in Rust](https://zenn.dev/mameta29/articles/1fa0dd67e18d7e)
- [Implementing RSA signature in Rust](https://zenn.dev/mameta29/articles/25051a3a26c9bd)