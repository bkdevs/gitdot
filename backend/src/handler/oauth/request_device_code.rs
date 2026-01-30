use axum::{extract::State, http::StatusCode};
use serde::Serialize;

use gitdot_core::dto::DeviceCodeRequest;

use crate::app::{AppError, AppResponse, AppState};

#[derive(Serialize, PartialEq)]
pub struct DeviceCodeServerResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[axum::debug_handler]
pub async fn request_device_code(
    State(state): State<AppState>,
) -> Result<AppResponse<DeviceCodeServerResponse>, AppError> {
    let request = DeviceCodeRequest {};
    let verification_uri = &state.settings.oauth_verification_uri;

    state
        .oauth_service
        .request_device_code(request, verification_uri)
        .await
        .map_err(AppError::from)
        .map(|response| {
            AppResponse::new(
                StatusCode::OK,
                DeviceCodeServerResponse {
                    device_code: response.device_code,
                    user_code: response.user_code,
                    verification_uri: response.verification_uri,
                    expires_in: response.expires_in,
                    interval: response.interval,
                },
            )
        })
}
