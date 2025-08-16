use rsa_encryption_decryption::rsa_cryption::rsa_console_cryption;
use rsa_encryption_decryption::rsa_signature::rsa_console_signature;
use rsa_encryption_decryption::rsa_server_client::{rsa_server, rsa_client};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <cryption|signature|server|client>");
        return;
    }

    match args[1].as_str() {
        "cryption" => rsa_console_cryption(),
        "signature" => rsa_console_signature(),
        "server" => rsa_server(),
        "client" => rsa_client(),
        _ => println!("Unknown command. Use 'cryption', 'signature', 'server' or 'client'."),
    }
}
