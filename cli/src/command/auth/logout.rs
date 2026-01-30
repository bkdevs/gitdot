use anyhow::Result;

use crate::keychain;

pub async fn logout() -> Result<()> {
    match keychain::get_token()? {
        Some(_) => {
            keychain::delete_token()?;
            println!("Successfully logged out.");
        }
        None => {
            println!("Not logged in.");
        }
    }

    Ok(())
}
