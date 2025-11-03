pub type IntegrationResult<T> = Result<T, IntegrationError>;

#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Ambiguous timezone: {0}")]
    AmbiguousTimezone(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest middleware error: {0}")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    #[error("Time parse error: {0}")]
    TimeParse(#[from] chrono::ParseError),
    #[error("Timezone parse error: {0}")]
    TimezoneParse(#[from] chrono_tz::ParseError),
}
