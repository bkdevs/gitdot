use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct PublishReview;

impl Endpoint for PublishReview {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/publish";
    const METHOD: http::Method = http::Method::POST;

    type Request = PublishReviewRequest;
    type Response = PublishReviewResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PublishReviewRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub diffs: Option<Vec<DiffUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdate {
    pub position: i32,
    pub message: Option<String>,
}

pub type PublishReviewResponse = ReviewResource;
