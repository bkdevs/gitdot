use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::{GetQuestionRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetQuestionServerRequest, QuestionServerResponse};

#[axum::debug_handler]
pub async fn get_question(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(question_id): Path<Uuid>,
    Json(request): Json<GetQuestionServerRequest>,
) -> Result<AppResponse<QuestionServerResponse>, AppError> {
    let auth_request =
        RepositoryAuthorizationRequest::with_id(auth_user.map(|u| u.id), request.repository_id);
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetQuestionRequest::new(question_id);
    state
        .question_service
        .get_question(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::OK, q.into()))
}
