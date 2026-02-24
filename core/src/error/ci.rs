use thiserror::Error;

#[derive(Debug, Error)]
pub enum CiConfigError {
    #[error("Failed to parse CI config: {0}")]
    ParseError(#[from] toml::de::Error),
}
