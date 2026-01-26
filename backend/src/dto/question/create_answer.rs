use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnswerServerRequest {
    pub repository_id: Uuid,
    pub question_id: Uuid,
    pub body: String,
}
