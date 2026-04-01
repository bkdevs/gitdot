use axum::{extract::State, http::StatusCode};

use gitdot_api::resource::auth::GitHubAuthRedirectResource;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

pub async fn redirect_to_github_auth(
    State(state): State<AppState>,
) -> Result<AppResponse<GitHubAuthRedirectResource>, AppError> {
    let response = state.oauth_service.get_github_authorization_url();
    Ok(AppResponse::new(StatusCode::OK, response.into_api()))
}
