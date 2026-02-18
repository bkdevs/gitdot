use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::question::QuestionResource};

pub struct CreateQuestion;

impl Endpoint for CreateQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateQuestionRequest;
    type Response = CreateQuestionResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct CreateQuestionRequest {
    pub title: String,
    pub body: String,
}

pub type CreateQuestionResponse = QuestionResource;
