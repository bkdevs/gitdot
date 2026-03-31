use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub port: String,
    pub database_url: String,
    pub resend_api_key: String,
    pub gitdot_private_key: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8081".to_string()),

            // TODO: move it to secrets
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            resend_api_key: env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set"),
            gitdot_private_key: env::var("GITDOT_PRIVATE_KEY")
                .expect("GITDOT_PRIVATE_KEY must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
