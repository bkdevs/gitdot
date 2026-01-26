use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateQuestionServerRequest {
    pub title: String,
    pub body: String,
}
