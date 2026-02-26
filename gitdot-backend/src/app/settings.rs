use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: String,
    pub git_project_root: String,
    pub database_url: String,
    pub supabase_jwt_public_key: String,
    pub oauth_device_verification_uri: String,
    pub github_app_id: u64,
    pub github_app_private_key: String,
    pub s2_server_url: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            git_project_root: env::var("GIT_PROJECT_ROOT")
                .unwrap_or_else(|_| "/srv/git".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            supabase_jwt_public_key: env::var("SUPABASE_JWT_PUBLIC_KEY")
                .expect("SUPABASE_JWT_PUBLIC_KEY must be set"),
            oauth_device_verification_uri: env::var("OAUTH_DEVICE_VERIFICATION_URI")
                .expect("OAUTH_DEVICE_VERIFICATION_URI must be set"),

            // TODO: migrate to secret client
            github_app_id: env::var("GITHUB_APP_ID")
                .expect("GITHUB_APP_ID must be set")
                .parse()
                .expect("GITHUB_APP_ID must be a valid u64"),
            github_app_private_key: env::var("GITHUB_APP_PRIVATE_KEY")
                .expect("GITHUB_APP_PRIVATE_KEY must be set"),
            s2_server_url: env::var("S2_SERVER_URL").expect("S2_SERVER_URL must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
