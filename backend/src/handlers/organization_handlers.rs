use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateOrganizationRequest;
use gitdot_core::services::OrganizationService;

use crate::app::{AppError, AppResponse};
use crate::dto::CreateOrganizationResponse;

pub async fn create_organization(
    State(org_service): State<Arc<dyn OrganizationService>>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<CreateOrganizationResponse>, AppError> {
    let request = CreateOrganizationRequest::new(org_name);
    org_service
        .create_organization(request)
        .await
        .map_err(AppError::from)
        .map(|ref org| AppResponse::new(StatusCode::CREATED, org.into()))
}
