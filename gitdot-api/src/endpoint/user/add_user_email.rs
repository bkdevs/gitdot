use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::user::UserEmailResource};

pub struct AddUserEmail;

impl Endpoint for AddUserEmail {
    const PATH: &'static str = "/user/emails";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddUserEmailRequest;
    type Response = AddUserEmailResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AddUserEmailRequest {
    pub email: String,
}

pub type AddUserEmailResponse = UserEmailResource;
