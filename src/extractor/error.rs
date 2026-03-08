use thiserror::Error;

/// Errors that can occur during YouTube video ID extraction
#[derive(Debug, Error)]
pub enum ExtractionError {
    /// The provided URL is not a valid YouTube URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// The video ID has an invalid length (should be exactly 11 characters)
    #[error("Invalid video ID length: expected {expected}, got {got}")]
    InvalidVideoIdLength { expected: usize, got: usize },
}
