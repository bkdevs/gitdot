use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::Endpoint, resource::review::ReviewCommentResource};

pub struct PublishReviewComments;

impl Endpoint for PublishReviewComments {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/comments";
    const METHOD: http::Method = http::Method::POST;

    type Request = PublishReviewCommentsRequest;
    type Response = PublishReviewCommentsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ReviewCommentInput {
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub start_character: Option<i32>,
    pub end_character: Option<i32>,
    pub side: Option<String>,
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PublishReviewCommentsRequest {
    pub comments: Vec<ReviewCommentInput>,
}

pub type PublishReviewCommentsResponse = Vec<ReviewCommentResource>;
