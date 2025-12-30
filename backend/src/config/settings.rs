use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub database_url: String,

    pub server_host: String,
    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,

            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        })
    }
}
