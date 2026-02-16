use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_organization as api;
use gitdot_core::dto::CreateOrganizationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_organization(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<api::CreateOrganizationResponse>, AppError> {
    let request = CreateOrganizationRequest::new(&org_name, auth_user.id)?;
    state
        .org_service
        .create_organization(request)
        .await
        .map_err(AppError::from)
        .map(|org| AppResponse::new(StatusCode::CREATED, org.into_api()))
}
