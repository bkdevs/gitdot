use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub port: String,
    pub database_url: Option<String>,
    pub gcp_project_id: Option<String>,
    pub oauth_device_verification_uri: String,
    pub gitdot_slack_bot_server_url: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8082".to_string()),

            // Database URL is retrieved from secret manager in production
            // Specify for local development
            database_url: env::var("DATABASE_URL").ok(),

            // GCP_PROJECT_ID is auto populated for Cloud Run
            // Specify for local development
            gcp_project_id: env::var("GCP_PROJECT_ID").ok(),

            // Redirect URI for OAuth device verification
            oauth_device_verification_uri: env::var("OAUTH_DEVICE_VERIFICATION_URI")
                .unwrap_or_else(|_| "http://localhost:3000/oauth/device".to_string()),

            // Slack bot server URL
            gitdot_slack_bot_server_url: env::var("GITDOT_SLACK_BOT_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
