use rsa_encryption_decryption::rsa_cryption::rsa_console_cryption;
use rsa_encryption_decryption::rsa_signature::rsa_console_signature;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <cryption|signature>");
        return;
    }

    match args[1].as_str() {
        "cryption" => rsa_console_cryption(),
        "signature" => rsa_console_signature(),
        _ => println!("Unknown command. Use 'cryption' or 'signature'."),
    }
}