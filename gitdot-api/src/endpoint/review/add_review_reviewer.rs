use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewerResource};

pub struct AddReviewReviewer;

impl Endpoint for AddReviewReviewer {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/reviewer";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddReviewReviewerRequest;
    type Response = AddReviewReviewerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AddReviewReviewerRequest {
    pub user_name: String,
}

pub type AddReviewReviewerResponse = ReviewerResource;
