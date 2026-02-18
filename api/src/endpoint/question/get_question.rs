use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::question::QuestionResource};

pub struct GetQuestion;

impl Endpoint for GetQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetQuestionRequest;
    type Response = GetQuestionResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct GetQuestionRequest;

pub type GetQuestionResponse = QuestionResource;
