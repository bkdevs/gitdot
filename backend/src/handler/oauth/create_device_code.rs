use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

use gitdot_api::endpoint::oauth::create_device_code as api;
use gitdot_core::dto::DeviceCodeRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_device_code(
    State(state): State<AppState>,
    Json(body): Json<api::CreateDeviceCodeRequest>,
) -> Result<AppResponse<api::CreateDeviceCodeResponse>, AppError> {
    let request = DeviceCodeRequest {
        client_id: body.client_id,
        verification_uri: state.settings.oauth_device_verification_uri.clone(),
    };
    state
        .oauth_service
        .request_device_code(request)
        .await
        .map_err(AppError::from)
        .map(|code| AppResponse::new(StatusCode::CREATED, code.into_api()))
}
