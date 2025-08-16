use rsa_encryption_decryption::rsa_crypto::rsa_console_crypto;
use rsa_encryption_decryption::rsa_signature::rsa_console_signature;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <crypto|signature>");
        return;
    }

    match args[1].as_str() {
        "crypto" => rsa_console_crypto(),
        "signature" => rsa_console_signature(),
        _ => println!("Unknown command. Use 'crypto' or 'signature'."),
    }
}