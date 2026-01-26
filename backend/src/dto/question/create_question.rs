use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateQuestionServerRequest {
    pub repository_id: Uuid,
    pub title: String,
    pub body: String,
}
