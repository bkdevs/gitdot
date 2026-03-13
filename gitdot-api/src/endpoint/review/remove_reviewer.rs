use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct RemoveReviewer;

impl Endpoint for RemoveReviewer {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = RemoveReviewerRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct RemoveReviewerRequest;
