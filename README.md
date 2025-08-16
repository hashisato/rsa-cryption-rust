# rsa-encryption-decryption

## Overview

This repository provides sample implementations of RSA encryption/decryption and digital signature in Rust.  
You can switch between encryption and signature features using command-line arguments.

## Directory Structure

- `src/rsa_crypto.rs`  
  RSA encryption and decryption logic
- `src/rsa_signature.rs`  
  RSA digital signature and verification logic
- `src/main.rs`  
  Command-line interface for feature selection
- `src/lib.rs`  
  Module definitions

## Usage

1. Make sure you have Rust installed on your system.
2. Clone this repository and navigate to the project directory.
3. Run the following commands:

   - To run the encryption/decryption demo:
     ```sh
     cargo run -- crypto
     ```

   - To run the signature/verification demo:
     ```sh
     cargo run -- signature
     ```

## Purpose

- To learn the basics of RSA cryptography in Rust
- To practice modular Rust programming
- To provide interactive console-based demos for encryption and digital signatures

## References

- [RSA暗号の仕組みとRustの実装](https://zenn.dev/mameta29/articles/1fa0dd67e18d7e)
- [RSAの暗号と署名の違い ～Rustで実装してみた～](https://zenn.dev/mameta29/articles/25051a3a26c9bd)

## License

This repository is licensed under the MIT License.