use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct GetQuestionServerRequest {
    pub repository_id: Uuid,
}
