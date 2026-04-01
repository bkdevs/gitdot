use crate::endpoint::Endpoint;

pub struct RedirectToGitHubAuth;

impl Endpoint for RedirectToGitHubAuth {
    const PATH: &'static str = "/auth/github/redirect";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = ();
}
