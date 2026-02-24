use crate::{dto::CiConfig, error::CiConfigError};

pub trait CiService: Send + Sync + 'static {
    fn from_string(&self, s: &str) -> Result<CiConfig, CiConfigError>;
}

#[derive(Debug, Clone)]
pub struct CiServiceImpl;

impl CiService for CiServiceImpl {
    fn from_string(&self, s: &str) -> Result<CiConfig, CiConfigError> {
        toml::from_str(s).map_err(CiConfigError::ParseError)
    }
}
