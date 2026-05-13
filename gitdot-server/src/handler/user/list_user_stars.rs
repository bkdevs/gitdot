use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_stars as api;
use gitdot_core::dto::ListUserStarsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_user_stars(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<api::ListUserStarsResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserStarsRequest::new(&user_name, viewer_id)?;
    state
        .user_service
        .list_stars(request)
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
