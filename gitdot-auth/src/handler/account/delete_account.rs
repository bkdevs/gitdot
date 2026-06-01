use axum::{extract::State, http::StatusCode};

use gitdot_axum::extract::Principal;
use gitdot_core::dto::DeleteAccountRequest;

use crate::app::{AppError, AppResponse, AppState};

pub async fn delete_account(
    principal: Principal,
    State(state): State<AppState>,
) -> Result<AppResponse<()>, AppError> {
    let request = DeleteAccountRequest::new(principal.id);
    state
        .account_service
        .delete_account(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
