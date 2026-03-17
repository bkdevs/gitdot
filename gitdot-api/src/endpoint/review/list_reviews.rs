use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct ListReviews;

impl Endpoint for ListReviews {
    const PATH: &'static str = "/repository/{owner}/{repo}/reviews";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListReviewsRequest;
    type Response = ListReviewsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListReviewsRequest;

pub type ListReviewsResponse = Vec<ReviewResource>;
