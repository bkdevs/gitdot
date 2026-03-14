use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::review::ReviewCommentResource};

pub struct CreateReviewComment;

impl Endpoint for CreateReviewComment {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/comment";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateReviewCommentRequest;
    type Response = CreateReviewCommentResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateReviewCommentRequest {
    pub body: String,
    pub diff_id: Option<Uuid>,
    pub revision_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub file_path: Option<String>,
    pub line_number: Option<i32>,
    pub side: Option<String>,
}

pub type CreateReviewCommentResponse = ReviewCommentResource;
