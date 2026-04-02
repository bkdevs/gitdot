use crate::dto::IpAddress;

#[derive(Debug, Clone)]
pub struct VerifyAuthCodeRequest {
    pub code: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpAddress>,
}

impl VerifyAuthCodeRequest {
    pub fn new(code: String, user_agent: Option<String>, ip_address: Option<&str>) -> Self {
        Self {
            code,
            user_agent,
            ip_address: ip_address.and_then(|ip| IpAddress::try_new(ip).ok()),
        }
    }
}
