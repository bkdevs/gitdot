use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::DeviceCodeRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::{DeviceCodeServerResponse, GetDeviceCodeServerRequest};

#[axum::debug_handler]
pub async fn get_device_code(
    State(state): State<AppState>,
    Json(body): Json<GetDeviceCodeServerRequest>,
) -> Result<AppResponse<DeviceCodeServerResponse>, AppError> {
    let request = DeviceCodeRequest {
        client_id: body.client_id,
    };
    state
        .oauth_service
        .request_device_code(request)
        .await
        .map_err(AppError::from)
        .map(|response| {
            AppResponse::new(
                StatusCode::OK,
                DeviceCodeServerResponse {
                    device_code: response.device_code,
                    user_code: response.user_code,
                    verification_uri: state.settings.oauth_verification_uri.clone(),
                    expires_in: response.expires_in,
                    interval: response.interval,
                },
            )
        })
}
