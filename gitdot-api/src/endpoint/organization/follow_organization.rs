use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct FollowOrganization;

impl Endpoint for FollowOrganization {
    const PATH: &'static str = "/organization/{org_name}/follow";
    const METHOD: http::Method = http::Method::POST;

    type Request = FollowOrganizationRequest;
    type Response = FollowOrganizationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct FollowOrganizationRequest {}

pub type FollowOrganizationResponse = ();
