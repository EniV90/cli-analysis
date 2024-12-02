
use thiserror::Error;
// use std::fmt;

/// Custom error types for the application
#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    // #[error("API connection failed: {0}")]
    // ApiConnectionError(String),

    #[error("Invalid token address: {0}")]
    InvalidTokenAddress(String),

    // #[error("Unknown error occurred")]
    // Unknown,
}

pub type Result<T> = std::result::Result<T, AnalyzerError>;


impl AnalyzerError {
    pub fn configuration_error(message: &str) -> Self {
        AnalyzerError::ConfigurationError(message.to_string())
    }

    // pub fn api_error(message: &str) -> Self {
    //     AnalyzerError::ApiConnectionError(message.to_string())
    // }

    pub fn invalid_address(address: &str) -> Self {
        AnalyzerError::InvalidTokenAddress(address.to_string())
    }
}