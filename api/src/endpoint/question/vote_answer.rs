use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::VoteResource};

pub struct VoteAnswer;

impl Endpoint for VoteAnswer {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/vote";
    const METHOD: http::Method = http::Method::POST;

    type Request = VoteAnswerRequest;
    type Response = VoteAnswerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct VoteAnswerRequest {
    pub value: i16,
}

pub type VoteAnswerResponse = VoteResource;
