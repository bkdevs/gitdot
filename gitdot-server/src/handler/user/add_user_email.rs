use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::user::add_user_email as api;
use gitdot_core::dto::AddUserEmailRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn add_user_email(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Json(request): Json<api::AddUserEmailRequest>,
) -> Result<AppResponse<api::AddUserEmailResponse>, AppError> {
    let request = AddUserEmailRequest::new(auth_user.id, &request.email)?;
    state
        .user_service
        .add_email(request)
        .await
        .map_err(AppError::from)
        .map(|email| AppResponse::new(StatusCode::CREATED, email.into_api()))
}
