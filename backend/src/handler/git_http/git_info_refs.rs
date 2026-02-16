use axum::extract::{Path, Query, State};

use gitdot_core::dto::{GitHttpAuthorizationRequest, InfoRefsRequest};

use crate::{
    app::{AppError, AppState},
    dto::{GitHttpServerResponse, InfoRefsQuery},
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn git_info_refs(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let auth_request =
        GitHttpAuthorizationRequest::for_info_refs(user_id, &owner, &repo, &params.service)?;
    state
        .auth_service
        .verify_authorized_for_git_http(auth_request)
        .await?;

    let request = InfoRefsRequest::new(&owner, &repo, &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;
    Ok(response.into())
}
