use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("DNS resolution error: {0}")]
    HickoryError(#[from] hickory_resolver::error::ResolverError),

    #[error("Domain does not exist (NXDOMAIN): {0}")]
    NxDomain(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}
