use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct SendAuthEmail;

impl Endpoint for SendAuthEmail {
    const PATH: &'static str = "/auth/email/send";
    const METHOD: http::Method = http::Method::POST;

    type Request = SendAuthEmailRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct SendAuthEmailRequest {
    pub email: String,
}
