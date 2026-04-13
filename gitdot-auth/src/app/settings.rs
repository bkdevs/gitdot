use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub port: String,
    pub database_url: String,
    pub resend_api_key: String,
    pub gitdot_private_key: String,
    pub gitdot_public_key: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_app_id: u64,
    pub github_app_private_key: String,
    pub oauth_device_verification_uri: String,
    pub gcp_project_id: Option<String>,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8082".to_string()),

            // TODO: move it to secrets
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            resend_api_key: env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set"),
            gitdot_private_key: env::var("GITDOT_PRIVATE_KEY")
                .expect("GITDOT_PRIVATE_KEY must be set"),
            gitdot_public_key: env::var("GITDOT_PUBLIC_KEY")
                .expect("GITDOT_PUBLIC_KEY must be set"),
            github_client_id: env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET must be set"),
            github_app_id: env::var("GITHUB_APP_ID")
                .expect("GITHUB_APP_ID must be set")
                .parse()
                .expect("GITHUB_APP_ID must be a number"),
            github_app_private_key: env::var("GITHUB_APP_PRIVATE_KEY")
                .expect("GITHUB_APP_PRIVATE_KEY must be set"),
            oauth_device_verification_uri: env::var("OAUTH_DEVICE_VERIFICATION_URI")
                .expect("OAUTH_DEVICE_VERIFICATION_URI must be set"),
            gcp_project_id: env::var("GCP_PROJECT_ID").ok(),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
