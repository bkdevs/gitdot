use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::question::AnswerResource};

pub struct CreateAnswer;

impl Endpoint for CreateAnswer {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/answer";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateAnswerRequest;
    type Response = CreateAnswerResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct CreateAnswerRequest {
    pub body: String,
}

pub type CreateAnswerResponse = AnswerResource;
