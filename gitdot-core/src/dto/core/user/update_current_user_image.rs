use bytes::Bytes;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateCurrentUserImageRequest {
    pub user_id: Uuid,
    pub bytes: Bytes,
}

impl UpdateCurrentUserImageRequest {
    pub fn new(user_id: Uuid, bytes: Bytes) -> Self {
        Self { user_id, bytes }
    }
}
