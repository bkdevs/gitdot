use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::device::authorize_device as api;
use gitdot_core::dto::AuthorizeDeviceRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::Principal,
};

pub async fn authorize_device(
    principal: Principal,
    State(state): State<AppState>,
    Json(body): Json<api::AuthorizeDeviceRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = AuthorizeDeviceRequest::new(&body.user_code, principal.id)?;
    state
        .authentication_service
        .authorize_device(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
