use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use signature::{Signer, Verifier, SignatureEncoding}; // ← 追加
use std::convert::TryFrom;

pub fn rsa_console_signature() {
    // 1. 鍵の生成
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    println!("Private Key: {:?}", private_key);
    println!("Public Key: {:?}", public_key);

    // 2. メッセージ
    let message = "Hello, RSA!";
    println!("Message: {}", message);

    // 3. 署名の生成（推奨API）
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key.clone());
    let signature = signing_key.sign(message.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes());
    println!("Signature: {}", encoded_signature);

    // 4. 署名の検証
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
