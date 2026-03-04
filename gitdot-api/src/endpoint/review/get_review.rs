use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct GetReview;

impl Endpoint for GetReview {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetReviewRequest;
    type Response = GetReviewResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetReviewRequest;

pub type GetReviewResponse = ReviewResource;
