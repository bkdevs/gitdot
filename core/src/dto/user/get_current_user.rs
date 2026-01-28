use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetCurrentUserRequest {
    pub user_id: Uuid,
}

impl GetCurrentUserRequest {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}
