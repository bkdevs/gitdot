use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::BuildResource};

pub struct ListBuilds;

impl Endpoint for ListBuilds {
    const PATH: &'static str = "/repository/{owner}/{repo}/builds";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListBuildsRequest;
    type Response = ListBuildsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListBuildsRequest {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub to: Option<DateTime<Utc>>,
}

pub type ListBuildsResponse = Vec<BuildResource>;
