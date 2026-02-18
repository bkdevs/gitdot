use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationResource};

pub struct CreateOrganization;

impl Endpoint for CreateOrganization {
    const PATH: &'static str = "/organization/{org_name}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateOrganizationRequest;
    type Response = CreateOrganizationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {}

pub type CreateOrganizationResponse = OrganizationResource;
