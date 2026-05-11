use std::env;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Settings {
    // infra
    pub port: String,
    pub database_url: String,

    // app secrets
    pub gitdot_public_key: String,
    pub gitdot_private_key: String,
    pub gitdot_slack_secret: String,

    // app urls
    pub gitdot_web_url: String,
    pub gitdot_slack_bot_server_url: String,
    pub gitdot_oauth_device_verification_url: String,

    // github
    pub github_app_id: u64,
    pub github_app_private_key: String,
    pub github_client_id: String,
    pub github_client_secret: String,

    // cloudflare
    pub cloudflare_account_id: String,
    pub cloudflare_r2_bucket_name: String,
    pub cloudflare_r2_access_key_id: String,
    pub cloudflare_r2_secret_access_key: String,

    // resend
    pub resend_api_key: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8082".to_string()),
            database_url: required("DATABASE_URL")?,

            gitdot_public_key: required("GITDOT_PUBLIC_KEY")?,
            gitdot_private_key: required("GITDOT_PRIVATE_KEY")?,
            gitdot_slack_secret: required("GITDOT_SLACK_SECRET")?,

            gitdot_web_url: env::var("GITDOT_WEB_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            gitdot_slack_bot_server_url: env::var("GITDOT_SLACK_BOT_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            gitdot_oauth_device_verification_url: env::var("GITDOT_OAUTH_DEVICE_VERIFICATION_URL")
                .unwrap_or_else(|_| "http://localhost:3000/oauth/device".to_string()),

            github_app_id: required("GITHUB_APP_ID")?
                .parse::<u64>()
                .context("GITHUB_APP_ID must be a u64")?,
            github_app_private_key: required("GITHUB_APP_PRIVATE_KEY")?,
            github_client_id: required("GITHUB_CLIENT_ID")?,
            github_client_secret: required("GITHUB_CLIENT_SECRET")?,

            cloudflare_account_id: required("CLOUDFLARE_ACCOUNT_ID")?,
            cloudflare_r2_bucket_name: required("CLOUDFLARE_R2_BUCKET_NAME")?,
            cloudflare_r2_access_key_id: required("CLOUDFLARE_R2_ACCESS_KEY_ID")?,
            cloudflare_r2_secret_access_key: required("CLOUDFLARE_R2_SECRET_ACCESS_KEY")?,

            resend_api_key: required("RESEND_API_KEY")?,
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

fn required(name: &'static str) -> anyhow::Result<String> {
    env::var(name).with_context(|| format!("{name} is required"))
}
