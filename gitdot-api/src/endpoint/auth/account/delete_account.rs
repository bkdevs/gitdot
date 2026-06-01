use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct DeleteAccount;

impl Endpoint for DeleteAccount {
    const PATH: &'static str = "/auth/account";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteAccountRequest;
    type Response = DeleteAccountResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct DeleteAccountRequest {}

pub type DeleteAccountResponse = ();
