use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::HasUserRequest;

use crate::app::{AppError, AppResponse, AppState};

pub async fn has_user(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<()>, AppError> {
    let request = HasUserRequest::new(&user_name)?;
    state
        .user_service
        .has_user(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
