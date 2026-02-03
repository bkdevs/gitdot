use axum::{Json, extract::State, http::StatusCode};

use gitdot_core::dto::CreateUserRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::{CreateUserServerRequest, UserServerResponse};

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserServerRequest>,
) -> Result<AppResponse<UserServerResponse>, AppError> {
    let create_request = CreateUserRequest::new(&request.name, &request.email, &request.password)?;
    state
        .user_service
        .create_user(create_request)
        .await
        .map_err(AppError::from)
        .map(|user| AppResponse::new(StatusCode::CREATED, user.into()))
}
