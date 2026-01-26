use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateQuestionServerRequest {
    pub title: String,
    pub body: String,
}
