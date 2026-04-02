use crate::dto::IpAddress;

#[derive(Debug, Clone)]
pub struct ExchangeGitHubCodeRequest {
    pub code: String,
    pub state: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpAddress>,
}

impl ExchangeGitHubCodeRequest {
    pub fn new(
        code: String,
        state: String,
        user_agent: Option<String>,
        ip_address: Option<&str>,
    ) -> Self {
        Self {
            code,
            state,
            user_agent,
            ip_address: ip_address.and_then(|ip| IpAddress::try_new(ip).ok()),
        }
    }
}
