use anyhow::Result;

use crate::api;
use crate::config::Config;
use crate::keychain;

pub async fn login() -> Result<()> {
    let config = Config::load()?;

    // Check if already logged in
    if let Some(_token) = keychain::get_token()? {
        println!("You are already logged in. Use 'gitdot auth logout' to log out first.");
        return Ok(());
    }

    // Request device code
    println!("Requesting device code...");
    let device = api::request_device_code(&config.api_url).await?;

    // Display instructions
    println!();
    println!("To authenticate, visit:");
    println!("  {}?code={}", device.verification_uri, device.user_code);
    println!();
    println!("And enter this code: {}", device.user_code);
    println!();

    // Try to open browser
    let full_url = format!("{}?code={}", device.verification_uri, device.user_code);
    if open::that(&full_url).is_ok() {
        println!("Opening browser...");
    } else {
        println!("Could not open browser automatically. Please visit the URL above.");
    }

    println!();
    println!("Waiting for authorization...");

    // Poll for token
    let token =
        api::poll_until_authorized(&config.api_url, &device.device_code, device.interval).await?;

    // Store token in keychain
    keychain::store_token(&token.access_token)?;

    println!();
    println!("Successfully logged in!");

    Ok(())
}
