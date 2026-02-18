use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_api::endpoint::oauth::authorize_device as api;
use gitdot_core::dto::AuthorizeDeviceRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::extract::{Principal, User};

#[axum::debug_handler]
pub async fn authorize_device(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Json(body): Json<api::AuthorizeDeviceRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = AuthorizeDeviceRequest::new(&body.user_code, auth_user.id)?;
    state
        .oauth_service
        .authorize_device(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
