use uuid::Uuid;

use crate::{dto::UserCode, error::DeviceError};

#[derive(Debug, Clone)]
pub struct AuthorizeDeviceRequest {
    pub user_code: UserCode,
    pub user_id: Uuid,
}

impl AuthorizeDeviceRequest {
    pub fn new(user_code: &str, user_id: Uuid) -> Result<Self, DeviceError> {
        Ok(Self {
            user_code: UserCode::parse(user_code, "user_code")?,
            user_id,
        })
    }
}
