use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_organizations as api;
use gitdot_core::dto::ListUserOrganizationsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_user_organizations(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<api::ListUserOrganizationsResponse>, AppError> {
    let request = ListUserOrganizationsRequest::new(&user_name)?;
    state
        .user_service
        .list_organizations(request)
        .await
        .map_err(AppError::from)
        .map(|orgs| AppResponse::new(StatusCode::OK, orgs.into_api()))
}
