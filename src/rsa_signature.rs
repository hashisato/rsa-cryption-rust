use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{SigningKey, VerifyingKey, Signature};
use sha2::Sha256;
use signature::{Signer, Verifier, SignatureEncoding};

/// Signs the given data using the provided RSA private key.
/// Returns the signature as a Vec<u8>.
pub fn sign(data: &[u8], private_key: &RsaPrivateKey) -> Vec<u8> {
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key.clone());
    signing_key.sign(data).to_bytes().to_vec()
}

/// Verifies the signature for the given data using the provided RSA public key.
/// Returns true if the signature is valid, false otherwise.
pub fn verify(data: &[u8], signature: &[u8], public_key: &RsaPublicKey) -> bool {
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key.clone());
    match Signature::try_from(signature) {
        Ok(sig) => verifying_key.verify(data, &sig).is_ok(),
        Err(_) => false,
    }
}

/// Provides a simple console-based demonstration of RSA signing and verification.
///
/// This function performs the following steps:
/// 1. Prompts the user to input a message.
/// 2. Generates a 2048-bit RSA key pair.
/// 3. Optionally displays the generated keys.
/// 4. Signs the input message using the private key.
/// 5. Optionally displays the signature (base64 encoded).
/// 6. Verifies the signature using the public key.
/// 7. Displays whether the signature is valid.
///
/// # Example
/// ```
/// rsa_console_signature();
/// ```
///
/// # Panics
/// This function will panic if key generation, signing, verification,
/// base64 encoding/decoding, or UTF-8 conversion fails.
pub fn rsa_console_signature() {
    use rand::rngs::OsRng;
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

    // 2. Generate signature
    let signature = sign(message.as_bytes(), &private_key);
    let encoded_signature = general_purpose::STANDARD.encode(&signature);

    // Display the signature (ask user for permission)
    print!("Display signature? (y/n): ");
    io::stdout().flush().unwrap();
    let mut show_signature = String::new();
    io::stdin().read_line(&mut show_signature).expect("Failed to read line");
    if show_signature.trim().eq_ignore_ascii_case("y") {
        println!("Signature: {}", encoded_signature);
    }

    // 3. Verify signature
    let decoded_signature = general_purpose::STANDARD
        .decode(&encoded_signature)
        .expect("Failed to decode base64");
    let is_valid = verify(message.as_bytes(), &decoded_signature, &public_key);
    if is_valid {
        println!("Signature is valid.");
    } else {
        println!("Signature is invalid.");
    }
}
