use crate::{endpoint::Endpoint, resource::repository::RepositoryPermissionResource};

pub struct GetPermission;

impl Endpoint for GetPermission {
    const PATH: &'static str = "/repository/{owner}/{repo}/permission";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = GetPermissionResponse;
}

pub type GetPermissionResponse = RepositoryPermissionResource;
