use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthorizeDeviceServerRequest {
    pub user_code: String,
}
