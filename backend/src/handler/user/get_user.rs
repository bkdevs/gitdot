use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::GetUserRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::UserServerResponse;

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<UserServerResponse>, AppError> {
    let request = GetUserRequest::new(&user_name)?;
    state
        .user_service
        .get_user(request)
        .await
        .map_err(AppError::from)
        .map(|user| AppResponse::new(StatusCode::OK, user.into()))
}
