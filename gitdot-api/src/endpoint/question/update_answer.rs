use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::AnswerResource};

pub struct UpdateAnswer;

impl Endpoint for UpdateAnswer {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateAnswerRequest;
    type Response = UpdateAnswerResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateAnswerRequest {
    pub body: String,
}

pub type UpdateAnswerResponse = AnswerResource;
