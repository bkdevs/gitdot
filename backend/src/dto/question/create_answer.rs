use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnswerServerRequest {
    pub question_id: Uuid,
    pub body: String,
}
