use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::add_member as api;
use gitdot_core::dto::{AddMemberRequest, OrganizationAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn add_member(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    Json(request): Json<api::AddMemberRequest>,
) -> Result<AppResponse<api::AddMemberResponse>, AppError> {
    let auth_request = OrganizationAuthorizationRequest::new(auth_user.id, &org_name)?;
    state
        .auth_service
        .verify_authorized_for_organization(auth_request)
        .await?;

    let request = AddMemberRequest::new(&org_name, &request.user_name, &request.role)?;
    state
        .org_service
        .add_member(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
