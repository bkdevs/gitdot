use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateOrganizationRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::CreateOrganizationServerResponse;

#[axum::debug_handler]
pub async fn create_organization(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<CreateOrganizationServerResponse>, AppError> {
    let request = CreateOrganizationRequest::new(&org_name, auth_user.id)?;
    state
        .org_service
        .create_organization(request)
        .await
        .map_err(AppError::from)
        .map(|org| AppResponse::new(StatusCode::CREATED, org.into()))
}
