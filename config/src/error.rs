#[derive(Debug, thiserror::Error)]
pub enum CiConfigError {
    #[error("failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("config validation failed:\n{}", .0.iter().map(|e| format!("  - {e}")).collect::<Vec<_>>().join("\n"))]
    Validation(Vec<String>),

    #[error("no build config found for trigger '{0}'")]
    NoMatchingBuild(String),
}
