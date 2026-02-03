use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserServerRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}
