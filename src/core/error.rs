/// Error handling module for the Serper SDK
///
/// This module defines all error types that can occur within the SDK,
/// providing comprehensive error handling with detailed context.
use thiserror::Error;

/// Main error type for the Serper SDK
///
/// This enum covers all possible error conditions that can occur
/// when using the SDK, from network issues to API-specific errors.
#[derive(Error, Debug)]
pub enum SerperError {
    /// HTTP request failed
    ///
    /// This error wraps underlying HTTP transport errors
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON parsing failed
    ///
    /// This error occurs when the API returns invalid JSON
    /// or when serialization/deserialization fails
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    ///
    /// This error represents HTTP error status codes and API-specific errors
    #[error("API error: {message}")]
    Api {
        /// The error message from the API or HTTP status description
        message: String,
    },

    /// Invalid API key provided
    ///
    /// This error occurs when the API key is empty, malformed, or rejected
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Configuration error
    ///
    /// This error occurs when SDK configuration is invalid
    #[error("Configuration error: {message}")]
    Config {
        /// Description of the configuration issue
        message: String,
    },

    /// Validation error
    ///
    /// This error occurs when input parameters are invalid
    #[error("Validation error: {message}")]
    Validation {
        /// Description of the validation issue
        message: String,
    },
}

impl SerperError {
    /// Creates a new API error with a custom message
    pub fn api_error(message: impl Into<String>) -> Self {
        Self::Api {
            message: message.into(),
        }
    }

    /// Creates a new configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Creates a new validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Checks if the error is related to authentication
    pub fn is_auth_error(&self) -> bool {
        matches!(self, SerperError::InvalidApiKey)
    }

    /// Checks if the error is related to network/transport
    pub fn is_network_error(&self) -> bool {
        matches!(self, SerperError::Request(_))
    }

    /// Checks if the error is related to data parsing
    pub fn is_parse_error(&self) -> bool {
        matches!(self, SerperError::Json(_))
    }

    /// Checks if the error is an API error
    pub fn is_api_error(&self) -> bool {
        matches!(self, SerperError::Api { .. })
    }
}

/// Type alias for Results using SerperError
pub type Result<T> = std::result::Result<T, SerperError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_api_key_error_display() {
        let error = SerperError::InvalidApiKey;
        assert_eq!(error.to_string(), "Invalid API key");
        assert!(error.is_auth_error());
    }

    #[test]
    fn test_api_error_display() {
        let error = SerperError::Api {
            message: "Rate limit exceeded".to_string(),
        };
        assert_eq!(error.to_string(), "API error: Rate limit exceeded");
        assert!(error.is_api_error());
    }

    #[test]
    fn test_config_error() {
        let error = SerperError::config_error("Invalid timeout");
        match error {
            SerperError::Config { message } => {
                assert_eq!(message, "Invalid timeout");
            }
            _ => panic!("Expected Config error"),
        }
    }

    #[test]
    fn test_validation_error() {
        let error = SerperError::validation_error("Empty query string");
        match error {
            SerperError::Validation { message } => {
                assert_eq!(message, "Empty query string");
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<i32>("invalid json");
        match json_error {
            Err(e) => {
                let serper_error: SerperError = e.into();
                assert!(serper_error.is_parse_error());
            }
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_error_variants() {
        let api_key_error = SerperError::InvalidApiKey;
        let api_error = SerperError::Api {
            message: "test".to_string(),
        };

        // Test that we can match on error variants
        match api_key_error {
            SerperError::InvalidApiKey => {}
            _ => panic!("Expected InvalidApiKey variant"),
        }

        match api_error {
            SerperError::Api { message } => {
                assert_eq!(message, "test");
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_result_type_alias() {
        let success_result: Result<i32> = Ok(42);
        assert_eq!(success_result.unwrap(), 42);

        let error_result: Result<i32> = Err(SerperError::InvalidApiKey);
        assert!(error_result.is_err());
    }

    #[test]
    fn test_error_classification() {
        let auth_error = SerperError::InvalidApiKey;
        let parse_error = SerperError::Json(
            serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err(),
        );
        let api_error = SerperError::api_error("Not found");

        assert!(auth_error.is_auth_error());
        assert!(parse_error.is_parse_error());
        assert!(api_error.is_api_error());
    }
}
