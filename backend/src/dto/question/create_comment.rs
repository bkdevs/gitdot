use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentServerRequest {
    pub parent_id: Uuid,
    pub body: String,
}
