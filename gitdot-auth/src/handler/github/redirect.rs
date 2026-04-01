use axum::extract::State;

use crate::app::{AppResponse, AppState, error::AppError};

pub async fn redirect_to_github_auth(
    State(state): State<AppState>,
) -> Result<AppResponse<()>, AppError> {
    let response = state.oauth_service.get_github_authorization_url();
    Ok(AppResponse::oauth_redirect(response))
}
