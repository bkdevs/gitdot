use gitdot_api::resource::oauth as api;
use gitdot_core::dto::{DeviceCodeResponse, TokenResponse};

use super::IntoApi;

impl IntoApi for DeviceCodeResponse {
    type ApiType = api::DeviceCodeResource;
    fn into_api(self) -> Self::ApiType {
        api::DeviceCodeResource {
            device_code: self.device_code,
            user_code: self.user_code,
            verification_uri: self.verification_uri,
            expires_in: self.expires_in,
            interval: self.interval,
        }
    }
}

impl IntoApi for TokenResponse {
    type ApiType = api::TokenResource;
    fn into_api(self) -> Self::ApiType {
        api::TokenResource {
            access_token: self.access_token,
            user_name: self.user_name,
            user_email: self.user_email,
        }
    }
}
