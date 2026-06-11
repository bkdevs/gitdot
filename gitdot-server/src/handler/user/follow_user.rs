use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::FollowUserRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn follow_user(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<()>, AppError> {
    let request = FollowUserRequest::new(auth_user.id, &user_name)?;
    state.user_service.follow_user(request).await?;

    Ok(AppResponse::new(StatusCode::OK, ()))
}
