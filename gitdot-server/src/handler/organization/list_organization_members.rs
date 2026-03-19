use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_organization_members as api;
use gitdot_core::dto::ListMembersRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_organization_members(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
    Query(params): Query<api::ListOrganizationMembersRequest>,
) -> Result<AppResponse<api::ListOrganizationMembersResponse>, AppError> {
    let request = ListMembersRequest::new(&org_name, params.role.as_deref())?;
    state
        .org_service
        .list_members(request)
        .await
        .map_err(AppError::from)
        .map(|members| AppResponse::new(StatusCode::OK, members.into_api()))
}
