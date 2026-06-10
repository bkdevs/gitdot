use ipnetwork::IpNetwork;

use crate::{
    dto::{Email, UserCode},
    error::{InputError, SessionError},
};

#[derive(Debug, Clone)]
pub struct VerifyAuthCodeRequest {
    pub email: Email,
    pub code: UserCode,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
}

impl VerifyAuthCodeRequest {
    pub fn new(
        email: &str,
        code: &str,
        user_agent: Option<String>,
        ip_address: Option<&str>,
    ) -> Result<Self, SessionError> {
        Ok(Self {
            email: Email::parse(email, "email")?,
            code: UserCode::try_new(code).map_err(|e| InputError::new("code", e.to_string()))?,
            user_agent,
            ip_address: ip_address.and_then(|ip| ip.parse().ok()),
        })
    }
}
