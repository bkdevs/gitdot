use ipnetwork::IpNetwork;

#[derive(Debug, Clone)]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
}

impl RefreshSessionRequest {
    pub fn new(
        refresh_token: String,
        user_agent: Option<String>,
        ip_address: Option<&str>,
    ) -> Self {
        Self {
            refresh_token,
            user_agent,
            ip_address: ip_address.and_then(|ip| ip.parse().ok()),
        }
    }
}
