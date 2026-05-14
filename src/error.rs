#[derive(Debug, thiserror::Error)]
pub enum ApiError{
    #[error("config load error: {0}")]
    ConfigError(#[from] config::ConfigError),
}
