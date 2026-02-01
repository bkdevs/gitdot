use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::AuthorizeDeviceRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::AuthorizeDeviceServerRequest;

#[axum::debug_handler]
pub async fn authorize_device(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(body): Json<AuthorizeDeviceServerRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = AuthorizeDeviceRequest {
        user_code: body.user_code,
        user_id: auth_user.id,
    };
    state
        .oauth_service
        .authorize_device(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
