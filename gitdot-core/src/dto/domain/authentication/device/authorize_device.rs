use uuid::Uuid;

use crate::{
    dto::UserCode,
    error::{DeviceError, InputError},
};

#[derive(Debug, Clone)]
pub struct AuthorizeDeviceRequest {
    pub user_code: UserCode,
    pub user_id: Uuid,
}

impl AuthorizeDeviceRequest {
    pub fn new(user_code: &str, user_id: Uuid) -> Result<Self, DeviceError> {
        Ok(Self {
            user_code: UserCode::try_new(user_code)
                .map_err(|e| InputError::new("user_code", e.to_string()))?,
            user_id,
        })
    }
}
