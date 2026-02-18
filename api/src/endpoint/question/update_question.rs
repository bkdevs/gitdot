use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::question::QuestionResource};

pub struct UpdateQuestion;

impl Endpoint for UpdateQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateQuestionRequest;
    type Response = UpdateQuestionResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct UpdateQuestionRequest {
    pub title: String,
    pub body: String,
}

pub type UpdateQuestionResponse = QuestionResource;
