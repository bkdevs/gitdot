use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReviewCommentAuthorizationRequest {
    pub user_id: Uuid,
    pub comment_id: Uuid,
}

impl ReviewCommentAuthorizationRequest {
    pub fn new(user_id: Uuid, comment_id: Uuid) -> Self {
        Self {
            user_id,
            comment_id,
        }
    }
}
