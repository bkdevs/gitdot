use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use gitdot_core::dto::{
    DeleteWebhookRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn delete_webhook(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, webhook_id)): Path<(String, String, Uuid)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = DeleteWebhookRequest::new(&owner, &repo, webhook_id)?;
    state.webhook_service.delete_webhook(request).await?;
    Ok(AppResponse::new(StatusCode::OK, ()))
}
