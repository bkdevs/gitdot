use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct GetQuestionsServerRequest {
    pub repository_id: Uuid,
}
