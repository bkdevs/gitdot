use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::TaskTokenResource};

pub struct IssueTaskToken;

impl Endpoint for IssueTaskToken {
    const PATH: &'static str = "/ci/task/{id}/token";
    const METHOD: http::Method = http::Method::POST;

    type Request = IssueTaskTokenRequest;
    type Response = IssueTaskTokenResponse;
}

#[derive(ApiRequest, Debug, Clone, Serialize, Deserialize)]
pub struct IssueTaskTokenRequest {}

pub type IssueTaskTokenResponse = TaskTokenResource;
