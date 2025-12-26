use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
