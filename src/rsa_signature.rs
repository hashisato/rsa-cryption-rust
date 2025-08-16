use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use signature::{Signer, Verifier, SignatureEncoding};
use std::io::{self, Write};

pub fn rsa_console_signature() {
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

    // 2. Generate signature (recommended API)
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key);
    let signature = signing_key.sign(message.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes());

    // Display the signature (ask user for permission)
    print!("Display signature? (y/n): ");
    io::stdout().flush().unwrap();
    let mut show_signature = String::new();
    io::stdin().read_line(&mut show_signature).expect("Failed to read line");
    if show_signature.trim().eq_ignore_ascii_case("y") {
        println!("Signature: {}", encoded_signature);
    }

    // 3. Verify signature
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let decoded_signature = general_purpose::STANDARD
        .decode(&encoded_signature)
        .expect("Failed to decode base64");
    let signature_obj = Signature::try_from(&decoded_signature[..]).expect("Invalid signature format");
    match verifying_key.verify(message.as_bytes(), &signature_obj) {
        Ok(_) => println!("Signature is valid."),
        Err(_) => println!("Signature is invalid."),
    }
}
