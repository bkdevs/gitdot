use anyhow::Result;

use crate::keychain;

pub async fn get_status() -> Result<()> {
    match keychain::get_token()? {
        Some(_) => {
            println!("Logged in");
            println!();
            println!("Token is stored in your system keychain.");
        }
        None => {
            println!("Not logged in");
            println!();
            println!("Run 'gitdot auth login' to authenticate.");
        }
    }

    Ok(())
}
