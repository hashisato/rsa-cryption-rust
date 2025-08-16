# rsa-cryption-rust

## Overview

This repository provides sample implementations of RSA encryption/decryption, digital signature, and secure server-client communication in Rust.  
You can try encryption, signature, or TCP-based secure message exchange using command-line arguments.

## Directory Structure

- `src/rsa_cryption.rs`  
  RSA encryption and decryption logic (library API)
- `src/rsa_signature.rs`  
  RSA digital signature and verification logic (library API)
- `src/rsa_server_client.rs`  
  TCP server-client communication using RSA encryption
- `src/main.rs`  
  Command-line interface for feature selection
- `src/lib.rs`  
  Module definitions and documentation

## Usage

1. Make sure you have Rust installed.
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

   - To run the server-client RSA communication:
     - Start the server in one terminal:
       ```sh
       cargo run -- server
       ```
     - Start the client in another terminal:
       ```sh
       cargo run -- client
       ```

## Features

- Learn the basics of RSA cryptography in Rust
- Practice modular Rust programming
- Interactive console-based demos for encryption, digital signatures, and secure TCP communication

## References

- [RSA encryption and Rust implementation (Japanese)](https://zenn.dev/mameta29/articles/1fa0dd67e18d7e)
- [Difference between RSA encryption and signature, with Rust implementation (Japanese)](https://zenn.dev/mameta29/articles/25051a3a26c9bd)

## License

This repository is licensed under the MIT License.