use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetMigrationRequest {
    pub user_id: Uuid,
    pub number: i32,
}

impl GetMigrationRequest {
    pub fn new(user_id: Uuid, number: i32) -> Self {
        Self { user_id, number }
    }
}
