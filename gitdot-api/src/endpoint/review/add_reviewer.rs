use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewerResource};

pub struct AddReviewer;

impl Endpoint for AddReviewer {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/reviewer";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddReviewerRequest;
    type Response = AddReviewerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AddReviewerRequest {
    pub user_name: String,
}

pub type AddReviewerResponse = ReviewerResource;
