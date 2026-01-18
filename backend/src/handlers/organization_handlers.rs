use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateOrganizationRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::CreateOrganizationResponse;

#[axum::debug_handler]
pub async fn create_organization(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    auth_user: AuthenticatedUser,
) -> Result<AppResponse<CreateOrganizationResponse>, AppError> {
    let request = CreateOrganizationRequest::new(org_name, auth_user.id);
    state
        .org_service
        .create_organization(request)
        .await
        .map_err(AppError::from)
        .map(|ref org| AppResponse::new(StatusCode::CREATED, org.into()))
}
