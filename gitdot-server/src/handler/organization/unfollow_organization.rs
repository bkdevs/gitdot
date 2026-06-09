use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::UnfollowOrganizationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn unfollow_organization(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<()>, AppError> {
    let request = UnfollowOrganizationRequest::new(auth_user.id, &org_name)?;
    state.org_service.unfollow_organization(request).await?;

    Ok(AppResponse::new(StatusCode::OK, ()))
}
