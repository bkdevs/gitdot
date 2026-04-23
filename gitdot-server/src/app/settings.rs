use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: String,
    pub git_project_root: String,

    pub database_url: Option<String>,
    pub gcp_project_id: Option<String>,

    pub gitdot_public_key: String,
    pub s2_server_url: String,

    pub vercel_oidc_url: String,

    // TODO: move it to secrets
    pub resend_api_key: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            git_project_root: env::var("GIT_PROJECT_ROOT")
                .unwrap_or_else(|_| "/srv/git".to_string()),

            // Database URL is retrieved from secret manager in production
            // Specify for local development
            database_url: env::var("DATABASE_URL").ok(),

            // GCP_PROJECT_ID is auto populated for Cloud Run
            // Specify for local development
            gcp_project_id: env::var("GCP_PROJECT_ID").ok(),

            gitdot_public_key: env::var("GITDOT_PUBLIC_KEY")
                .expect("GITDOT_PUBLIC_KEY must be set"),
            s2_server_url: env::var("S2_SERVER_URL").expect("S2_SERVER_URL must be set"),

            vercel_oidc_url: env::var("VERCEL_OIDC_URL").expect("VERCEL_OIDC_URL must be set"),

            resend_api_key: env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
