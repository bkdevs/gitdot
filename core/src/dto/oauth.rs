mod authorize_device;
mod poll_token;
mod request_device_code;

pub use authorize_device::AuthorizeDeviceRequest;
pub use poll_token::PollTokenRequest;
pub use request_device_code::DeviceCodeRequest;

#[derive(Debug, Clone)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
}
