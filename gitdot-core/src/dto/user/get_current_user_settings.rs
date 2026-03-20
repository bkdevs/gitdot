use uuid::Uuid;

pub struct GetCurrentUserSettingsRequest {
    pub user_id: Uuid,
}

impl GetCurrentUserSettingsRequest {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}
