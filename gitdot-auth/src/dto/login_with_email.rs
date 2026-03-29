use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginWithEmailRequest {
    pub email: String,
}
