//! # RSA Server-Client Communication
//!
//! This module provides a simple example of secure message exchange between a server and a client using RSA encryption over TCP.
//!
//! ## Features
//! - Server generates an RSA key pair and sends the public key to the client.
//! - Client receives the public key, encrypts a console-input message, and sends it to the server.
//! - Server decrypts and displays the received message.
//!
//! ## Usage
//! Call `rsa_server()` to start the server, and `rsa_client()` to start the client.
//!
//! ## Example
//! ```rust
//! // In main.rs
//! fn main() {
//!     // Run as server
//!     rsa_server_client::rsa_server();
//!     // Or run as client
//!     rsa_server_client::rsa_client();
//! }
//! ```

use crate::rsa_cryption::{encrypt, decrypt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rand::rngs::OsRng;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

/// Serializes the RSA public key to DER bytes.
fn serialize_public_key(public_key: &RsaPublicKey) -> Vec<u8> {
    public_key.to_pkcs1_der().unwrap().as_bytes().to_vec()
}

/// Deserializes the RSA public key from DER bytes.
fn deserialize_public_key(bytes: &[u8]) -> RsaPublicKey {
    RsaPublicKey::from_pkcs1_der(bytes).unwrap()
}

/// Starts the RSA server, sends public key, receives and decrypts an encrypted message.
pub fn rsa_server() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);

    let listener = TcpListener::bind("127.0.0.1:4000").expect("Failed to bind");
    println!("Server listening on 127.0.0.1:4000");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        // 1. Send public key to client
        let pubkey_bytes = serialize_public_key(&public_key);
        let pubkey_len = pubkey_bytes.len() as u32;
        stream.write_all(&pubkey_len.to_be_bytes()).unwrap();
        stream.write_all(&pubkey_bytes).unwrap();

        // 2. Receive encrypted message
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let msg_len = u32::from_be_bytes(len_buf) as usize;
        let mut encrypted_msg = vec![0u8; msg_len];
        stream.read_exact(&mut encrypted_msg).unwrap();

        // 3. Decrypt and print
        let decrypted = decrypt(&encrypted_msg, &private_key).expect("Decryption failed");
        println!("Decrypted message: {}", String::from_utf8_lossy(&decrypted));
        break; // single connection demo
    }
}

/// Starts the RSA client, receives public key, encrypts console input, and sends it to the server.
pub fn rsa_client() {
    let mut stream = TcpStream::connect("127.0.0.1:4000").expect("Failed to connect");

    // 1. Receive public key from server
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();
    let pubkey_len = u32::from_be_bytes(len_buf) as usize;
    let mut pubkey_bytes = vec![0u8; pubkey_len];
    stream.read_exact(&mut pubkey_bytes).unwrap();
    let public_key = deserialize_public_key(&pubkey_bytes);

    // 2. Get message from console input
    print!("Input message to send: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim();

    // 3. Encrypt message
    let encrypted = encrypt(message.as_bytes(), &public_key).expect("Encryption failed");

    // 4. Send encrypted message
    let msg_len = encrypted.len() as u32;
    stream.write_all(&msg_len.to_be_bytes()).unwrap();
    stream.write_all(&encrypted).unwrap();

    println!("Client sent encrypted message.");
}
