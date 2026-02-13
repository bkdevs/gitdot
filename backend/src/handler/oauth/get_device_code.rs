use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::oauth::get_device_code as api;
use gitdot_core::dto::DeviceCodeRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_device_code(
    State(state): State<AppState>,
    Query(params): Query<api::GetDeviceCodeRequest>,
) -> Result<AppResponse<api::GetDeviceCodeResponse>, AppError> {
    let request = DeviceCodeRequest {
        client_id: params.client_id,
        verification_uri: state.settings.oauth_device_verification_uri.clone(),
    };
    state
        .token_service
        .request_device_code(request)
        .await
        .map_err(AppError::from)
        .map(|code| AppResponse::new(StatusCode::CREATED, code.into_api()))
}
