use axum::extract::{Path, Query, State};

use gitdot_core::dto::{InfoRefsRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppState},
    dto::{GitHttpServerResponse, InfoRefsQuery},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn git_info_refs(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let permission = match params.service.as_str() {
        "git-receive-pack" => RepositoryPermission::Write,
        _ => RepositoryPermission::Read,
    };
    let auth_request = RepositoryAuthorizationRequest::new(user_id, &owner, &repo, permission)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = InfoRefsRequest::new(&owner, &repo, &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;
    Ok(response.into())
}
