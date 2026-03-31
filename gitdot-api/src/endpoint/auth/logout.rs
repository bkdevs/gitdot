use crate::endpoint::Endpoint;

pub struct Logout;

impl Endpoint for Logout {
    const PATH: &'static str = "/auth/logout";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = ();
}
