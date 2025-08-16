use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::Pkcs1v15Encrypt;
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine as _};
use std::io::{self, Write};

pub fn rsa_console_crypto() {
    // 0. Get the message from user input
    print!("Input message: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim();

    // 1. Generate keys
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Display public and private keys (ask user for permission)
    print!("Display generated keys? (y/n): ");
    io::stdout().flush().unwrap();
    let mut show_keys = String::new();
    io::stdin().read_line(&mut show_keys).expect("Failed to read line");
    if show_keys.trim().eq_ignore_ascii_case("y") {
        println!("Private Key: {:?}", private_key);
        println!("Public Key: {:?}", public_key);
    }

    // 2. Encrypt message
    let encrypted_data = public_key.encrypt(
        &mut rng,
        Pkcs1v15Encrypt,
        message.as_bytes(),
    ).expect("Failed to encrypt");
    // Encode the encrypted data to base64 for display
    let encoded_encrypted_data = general_purpose::STANDARD.encode(&encrypted_data);
    print!("Displayed encrypted message? (y/n): ");
    io::stdout().flush().unwrap();
    let mut show_encrypted = String::new();
    io::stdin().read_line(&mut show_encrypted).expect("Failed to read line");
    if show_encrypted.trim().eq_ignore_ascii_case("y") {
        println!("Encrypted message: {}", encoded_encrypted_data);
    }

    // 3. Decrypt the ciphertext
    let decoded_encrypted_data = general_purpose::STANDARD
        .decode(&encoded_encrypted_data)
        .expect("Failed to decode base64");
    let decrypted_data = private_key.decrypt(
        Pkcs1v15Encrypt,
        &decoded_encrypted_data,
    ).expect("Failed to decrypt");

    // Display the decrypted message
    let decrypted_message = String::from_utf8(decrypted_data).expect("Failed to convert to string");
    println!("Decrypted message: {}", decrypted_message);
}
