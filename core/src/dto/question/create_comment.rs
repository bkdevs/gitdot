use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    pub parent_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
}

impl CreateCommentRequest {
    pub fn new(parent_id: Uuid, author_id: Uuid, body: String) -> Self {
        Self {
            parent_id,
            author_id,
            body,
        }
    }
}
