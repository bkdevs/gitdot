use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthorizeDeviceRequest {
    pub user_code: String,
    pub user_id: Uuid,
}

impl AuthorizeDeviceRequest {
    pub fn new(user_code: String, user_id: Uuid) -> Self {
        Self { user_code, user_id }
    }
}
