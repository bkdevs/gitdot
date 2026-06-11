use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct UnfollowOrganization;

impl Endpoint for UnfollowOrganization {
    const PATH: &'static str = "/organization/{org_name}/follow";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = UnfollowOrganizationRequest;
    type Response = UnfollowOrganizationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UnfollowOrganizationRequest {}

pub type UnfollowOrganizationResponse = ();
