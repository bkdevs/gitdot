use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateCommentRequest {
    pub id: Uuid,
    pub body: String,
}

impl UpdateCommentRequest {
    pub fn new(id: Uuid, body: String) -> Self {
        Self { id, body }
    }
}
