use base64::{engine::general_purpose, Engine as _};
use biscuit_auth::KeyPair;

use super::CliArgs;

impl CliArgs {
    /// If the generate_keypair flag is passed, generate a new keypair and print it to the console.
    pub fn generate_keypair(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.generate_keypair {
            let keypair = KeyPair::new();
            let private_key = general_purpose::URL_SAFE_NO_PAD.encode(keypair.private().to_bytes());
            let public_key = general_purpose::URL_SAFE_NO_PAD.encode(keypair.public().to_bytes());

            println!("Base64 Key Pair Generated");
            println!("Private Key: {:?}", private_key);
            println!("Public Key: {:?}", public_key);
        }
        Ok(())
    }
}
