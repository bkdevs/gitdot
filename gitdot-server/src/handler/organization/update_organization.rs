use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::organization::update_organization as api;
use gitdot_core::dto::{OrganizationAuthorizationRequest, UpdateOrganizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_organization(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    Json(request): Json<api::UpdateOrganizationRequest>,
) -> Result<AppResponse<api::UpdateOrganizationResponse>, AppError> {
    let auth_request = OrganizationAuthorizationRequest::new(auth_user.id, &org_name)?;
    state
        .authorization_service
        .verify_authorized_for_organization(auth_request)
        .await?;

    let request = UpdateOrganizationRequest::new(
        &org_name,
        request.location,
        request.readme,
        request.links,
        request.display_name,
    )?;
    state
        .org_service
        .update_organization(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
