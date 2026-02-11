use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: String,
    pub database_url: String,
    pub supabase_jwt_public_key: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8081".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            supabase_jwt_public_key: env::var("SUPABASE_JWT_PUBLIC_KEY")
                .expect("SUPABASE_JWT_PUBLIC_KEY must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
