use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::list_organizations as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_organizations(
    State(state): State<AppState>,
) -> Result<AppResponse<api::ListOrganizationsResponse>, AppError> {
    state
        .org_service
        .list_organizations()
        .await
        .map_err(AppError::from)
        .map(|orgs| AppResponse::new(StatusCode::OK, orgs.into_api()))
}
