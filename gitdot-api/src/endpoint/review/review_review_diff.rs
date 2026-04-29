use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct ReviewReviewDiff;

impl Endpoint for ReviewReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/review";
    const METHOD: http::Method = http::Method::POST;

    type Request = ReviewReviewDiffRequest;
    type Response = ReviewReviewDiffResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewReviewDiffCommentInput {
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

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ReviewReviewDiffRequest {
    pub action: String,
    pub comments: Vec<ReviewReviewDiffCommentInput>,
}

pub type ReviewReviewDiffResponse = ReviewResource;
