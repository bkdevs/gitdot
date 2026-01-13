use axum::{Json, extract::Path, http::StatusCode};

use crate::domains::repository::dto::create_repository::{
    CreateRepositoryRequest, CreateRepositoryResponse,
};

pub async fn create_repository(
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<CreateRepositoryRequest>,
) -> Result<Json<CreateRepositoryResponse>, StatusCode> {
    Ok(Json(CreateRepositoryResponse {
        owner: owner,
        name: repo,
        default_branch: request.default_branch,
    }))
}
