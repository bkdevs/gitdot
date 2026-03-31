use gitdot_api::endpoint::auth::email::verify as api;
use gitdot_core::dto::VerifyAuthCodeResponse;

use super::IntoApi;

impl IntoApi for VerifyAuthCodeResponse {
    type ApiType = api::VerifyAuthCodeResponse;
    fn into_api(self) -> Self::ApiType {
        api::VerifyAuthCodeResponse {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
        }
    }
}
