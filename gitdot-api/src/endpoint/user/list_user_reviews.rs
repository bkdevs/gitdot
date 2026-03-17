use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct ListUserReviews;

impl Endpoint for ListUserReviews {
    const PATH: &'static str = "/user/{user_name}/reviews";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserReviewsRequest;
    type Response = ListUserReviewsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListUserReviewsRequest {
    pub status: Option<String>,
    pub owner: Option<String>,
    pub repo: Option<String>,
}

pub type ListUserReviewsResponse = Vec<ReviewResource>;
