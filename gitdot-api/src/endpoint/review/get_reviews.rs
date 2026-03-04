use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct GetReviews;

impl Endpoint for GetReviews {
    const PATH: &'static str = "/repository/{owner}/{repo}/reviews";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetReviewsRequest;
    type Response = GetReviewsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetReviewsRequest;

pub type GetReviewsResponse = Vec<ReviewResource>;
