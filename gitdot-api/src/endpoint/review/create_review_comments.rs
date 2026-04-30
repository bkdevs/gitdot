use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct CreateReviewComments;

impl Endpoint for CreateReviewComments {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/diff/{position}/comments";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateReviewCommentsRequest;
    type Response = CreateReviewCommentsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateReviewCommentsRequest {
    pub comments: Vec<ReviewCommentInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewCommentInput {
    pub revision_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub start_character: Option<i32>,
    pub end_character: Option<i32>,
    pub side: Option<String>,
}

pub type CreateReviewCommentsResponse = ReviewResource;
