use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAnswerServerRequest {
    pub body: String,
}
