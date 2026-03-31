use crate::{endpoint::Endpoint, resource::auth::AuthTokensResource};

pub struct RefreshSession;

impl Endpoint for RefreshSession {
    const PATH: &'static str = "/auth/refresh";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = AuthTokensResource;
}
