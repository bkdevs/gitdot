use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DeleteAccountRequest {
    pub user_id: Uuid,
}

impl DeleteAccountRequest {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}
