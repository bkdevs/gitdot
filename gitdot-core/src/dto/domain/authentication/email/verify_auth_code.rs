use ipnetwork::IpNetwork;

use crate::{
    dto::UserCode,
    error::{InputError, SessionError},
};

#[derive(Debug, Clone)]
pub struct VerifyAuthCodeRequest {
    pub code: UserCode,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
}

impl VerifyAuthCodeRequest {
    pub fn new(
        code: &str,
        user_agent: Option<String>,
        ip_address: Option<&str>,
    ) -> Result<Self, SessionError> {
        Ok(Self {
            code: UserCode::try_new(code).map_err(|e| InputError::new("code", e.to_string()))?,
            user_agent,
            ip_address: ip_address.and_then(|ip| ip.parse().ok()),
        })
    }
}
