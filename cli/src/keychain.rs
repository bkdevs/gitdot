use anyhow::{Context, Result};
use keyring::Entry;

const SERVICE_NAME: &str = "gitdot-cli";
const USERNAME: &str = "access_token";

fn get_entry() -> Result<Entry> {
    Entry::new(SERVICE_NAME, USERNAME).context("Failed to access keychain")
}

pub fn store_token(token: &str) -> Result<()> {
    let entry = get_entry()?;
    entry
        .set_password(token)
        .context("Failed to store token in keychain")
}

pub fn get_token() -> Result<Option<String>> {
    let entry = get_entry()?;
    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e).context("Failed to retrieve token from keychain"),
    }
}

pub fn delete_token() -> Result<()> {
    let entry = get_entry()?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(e).context("Failed to delete token from keychain"),
    }
}
