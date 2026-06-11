use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::FollowOrganizationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn follow_organization(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<()>, AppError> {
    let request = FollowOrganizationRequest::new(auth_user.id, &org_name)?;
    state.org_service.follow_organization(request).await?;

    Ok(AppResponse::new(StatusCode::OK, ()))
}
