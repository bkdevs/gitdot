use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: String,
    pub git_project_root: String,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            git_project_root: env::var("GIT_PROJECT_ROOT")
                .unwrap_or_else(|_| "/srv/git".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
