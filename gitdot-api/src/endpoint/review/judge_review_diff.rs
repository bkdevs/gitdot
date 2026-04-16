use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct JudgeReviewDiff;

impl Endpoint for JudgeReviewDiff {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit";
    const METHOD: http::Method = http::Method::POST;

    type Request = JudgeReviewDiffRequest;
    type Response = JudgeReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct JudgeReviewDiffRequest {
    pub action: String,
    pub comments: Vec<ReviewDiffComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDiffComment {
    pub body: String,
    pub parent_id: Option<Uuid>,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub side: Option<String>,
}

pub type JudgeReviewDiffResponse = ReviewResource;
