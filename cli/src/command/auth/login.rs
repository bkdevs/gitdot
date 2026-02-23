use std::time::{Duration, Instant};

use anyhow::bail;

use gitdot_client::client::GitdotClient;

use crate::{config::Config, store::GitCredentialStore};

pub async fn login(mut config: Config) -> anyhow::Result<()> {
    let api_client = GitdotClient::new("gitdot-cli".to_string());
    let device_code_response = api_client.get_device_code().await?;

    println!("Open the following URL in your browser:");
    println!("{}", device_code_response.verification_uri);
    println!("Enter the code: {}", device_code_response.user_code);

    let interval = Duration::from_secs(device_code_response.interval);
    let expires_in = Duration::from_secs(device_code_response.expires_in);
    let started_at = Instant::now();

    loop {
        tokio::time::sleep(interval).await;

        if started_at.elapsed() >= expires_in {
            bail!("Device code expired. Please try again.");
        }

        match api_client
            .poll_token(&device_code_response.device_code)
            .await
        {
            Ok(response) => {
                config.profile.user_name = response.user_name.clone();
                config.profile.user_email = response.user_email;
                config.save().await?;

                GitCredentialStore::store(
                    api_client.get_public_url(),
                    &response.user_name,
                    &response.access_token,
                )?;

                println!("Successfully logged in!");

                return Ok(());
            }
            Err(_) => {
                continue;
            }
        }
    }
}
