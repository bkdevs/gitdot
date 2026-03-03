use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use base64ct::LineEnding;
use ed25519_dalek::{
    SigningKey,
    pkcs8::{EncodePrivateKey, spki::EncodePublicKey},
};

fn main() {
    if !Path::new("Cargo.toml").exists() {
        eprintln!("Error: must be run from the repository root (Cargo.toml not found)");
        std::process::exit(1);
    }

    let server_env = Path::new("gitdot-server/.env");
    let s2_env = Path::new("s2-server/.env");

    if server_env.exists() {
        let contents = fs::read_to_string(server_env).expect("failed to read gitdot-server/.env");
        if contents.contains("GITDOT_PRIVATE_KEY") || contents.contains("GITDOT_PUBLIC_KEY") {
            eprintln!(
                "Error: GITDOT_PRIVATE_KEY or GITDOT_PUBLIC_KEY already exists in gitdot-server/.env"
            );
            std::process::exit(1);
        }
    }
    if s2_env.exists() {
        let contents = fs::read_to_string(s2_env).expect("failed to read s2-server/.env");
        if contents.contains("GITDOT_PUBLIC_KEY") {
            eprintln!("Error: GITDOT_PUBLIC_KEY already exists in s2-server/.env");
            std::process::exit(1);
        }
    }

    let seed: [u8; 32] = rand::random();
    let signing_key = SigningKey::from_bytes(&seed);

    let private_pem = signing_key
        .to_pkcs8_pem(LineEnding::LF)
        .expect("failed to encode private key");
    let public_pem = signing_key
        .verifying_key()
        .to_public_key_pem(LineEnding::LF)
        .expect("failed to encode public key");

    let mut server_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(server_env)
        .expect("failed to open gitdot-server/.env");
    writeln!(
        server_file,
        "GITDOT_PRIVATE_KEY=\"{}\"",
        private_pem.as_str().trim_end()
    )
    .expect("failed to write to gitdot-server/.env");
    writeln!(
        server_file,
        "GITDOT_PUBLIC_KEY=\"{}\"",
        public_pem.trim_end()
    )
    .expect("failed to write to gitdot-server/.env");

    let mut s2_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(s2_env)
        .expect("failed to open s2-server/.env");
    writeln!(s2_file, "GITDOT_PUBLIC_KEY=\"{}\"", public_pem.trim_end())
        .expect("failed to write to s2-server/.env");

    println!("Public key written to s2-server/.env:");
    println!("Private key written to gitdot-server/.env");
}
