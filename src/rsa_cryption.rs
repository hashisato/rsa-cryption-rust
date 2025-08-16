use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::Pkcs1v15Encrypt;
use rand::rngs::OsRng;

/// Encrypts data using the given RSA public key.
/// Returns the encrypted data as a Vec<u8> on success.
pub fn encrypt(data: &[u8], public_key: &RsaPublicKey) -> Result<Vec<u8>, rsa::errors::Error> {
    let mut rng = OsRng;
    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)
}

/// Decrypts data using the given RSA private key.
/// Returns the decrypted data as a Vec<u8> on success.
pub fn decrypt(encrypted_data: &[u8], private_key: &RsaPrivateKey) -> Result<Vec<u8>, rsa::errors::Error> {
    private_key.decrypt(Pkcs1v15Encrypt, encrypted_data)
}

/// Provides a simple console-based demonstration of RSA encryption and decryption.
///
/// This function performs the following steps:
/// 1. Prompts the user to input a message.
/// 2. Generates a 2048-bit RSA key pair.
/// 3. Optionally displays the generated keys.
/// 4. Encrypts the input message using the public key.
/// 5. Optionally displays the encrypted message (base64 encoded).
/// 6. Decrypts the encrypted message using the private key.
/// 7. Displays the decrypted message.
///
/// # Example
/// ```
/// rsa_console_cryption();
/// ```
///
/// # Panics
/// This function will panic if key generation, encryption, decryption,
/// base64 encoding/decoding, or UTF-8 conversion fails.
pub fn rsa_console_cryption() {
    use base64::{engine::general_purpose, Engine as _};
    use std::io::{self, Write};

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
    let encrypted_data = encrypt(message.as_bytes(), &public_key).expect("Failed to encrypt");
    // Encode the encrypted data to base64 for display
    let encoded_encrypted_data = general_purpose::STANDARD.encode(&encrypted_data);

    // Display the encrypted message (ask user for permission)
    print!("Display encrypted message? (y/n): ");
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
    let decrypted_data = decrypt(&decoded_encrypted_data, &private_key).expect("Failed to decrypt");

    // Display the decrypted message
    let decrypted_message = String::from_utf8(decrypted_data).expect("Failed to convert to string");
    println!("Decrypted message: {}", decrypted_message);
}
