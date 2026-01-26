use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    pub author_id: Uuid,
    pub parent_id: Uuid,
    pub body: String,
}

impl CreateCommentRequest {
    pub fn new(author_id: Uuid, parent_id: Uuid, body: String) -> Self {
        Self {
            author_id,
            parent_id,
            body,
        }
    }
}
