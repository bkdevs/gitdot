use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct UpdateReview;

impl Endpoint for UpdateReview {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateReviewRequest;
    type Response = UpdateReviewResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateReviewRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

pub type UpdateReviewResponse = ReviewResource;
