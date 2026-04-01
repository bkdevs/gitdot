#[derive(Debug, Clone)]
pub struct OAuthRedirectResponse {
    pub authorize_url: String,
    pub state: String,
}
