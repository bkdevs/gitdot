use crate::endpoint::Endpoint;
use crate::resource::question::VoteResource;
use serde::{Deserialize, Serialize};

pub struct VoteAnswer;

impl Endpoint for VoteAnswer {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/vote";
    const METHOD: http::Method = http::Method::POST;

    type Request = VoteAnswerRequest;
    type Response = VoteAnswerResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteAnswerRequest {
    pub value: i16,
}

pub type VoteAnswerResponse = VoteResource;
