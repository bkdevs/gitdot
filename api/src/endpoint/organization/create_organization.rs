use crate::{endpoint::Endpoint, resource::OrganizationResource};
use serde::{Deserialize, Serialize};

pub struct CreateOrganization;

impl Endpoint for CreateOrganization {
    const PATH: &'static str = "/organization/{org_name}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateOrganizationRequest;
    type Response = CreateOrganizationResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrganizationRequest {}

pub type CreateOrganizationResponse = OrganizationResource;
