use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: String,
    pub git_project_root: String,

    pub database_url: Option<String>,
    pub gcp_project_id: Option<String>,

    pub gitdot_public_key: String,
    pub supabase_jwt_public_key: String,
    pub oauth_device_verification_uri: String,
    pub s2_server_url: String,

    #[cfg(feature = "otel")]
    pub vercel_oidc_url: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            git_project_root: env::var("GIT_PROJECT_ROOT")
                .unwrap_or_else(|_| "/srv/git".to_string()),

            database_url: env::var("DATABASE_URL").ok(),
            gcp_project_id: env::var("GCP_PROJECT_ID").ok(),

            gitdot_public_key: env::var("GITDOT_PUBLIC_KEY")
                .expect("GITDOT_PUBLIC_KEY must be set"),
            supabase_jwt_public_key: env::var("SUPABASE_JWT_PUBLIC_KEY")
                .expect("SUPABASE_JWT_PUBLIC_KEY must be set"),
            oauth_device_verification_uri: env::var("OAUTH_DEVICE_VERIFICATION_URI")
                .expect("OAUTH_DEVICE_VERIFICATION_URI must be set"),
            s2_server_url: env::var("S2_SERVER_URL").expect("S2_SERVER_URL must be set"),

            #[cfg(feature = "otel")]
            vercel_oidc_url: env::var("VERCEL_OIDC_URL").expect("VERCEL_OIDC_URL must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
