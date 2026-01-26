use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnswerServerRequest {
    pub body: String,
}
