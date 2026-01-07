use std::env;

#[derive(Debug)]
pub struct Settings {
    pub server_host: String,
    pub server_port: u16,

    pub git_project_root: String,
}

impl Settings {
    pub fn new() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),

            git_project_root: env::var("GIT_PROJECT_ROOT")
                .unwrap_or_else(|_| "/srv/git".to_string()),
        })
    }

    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
