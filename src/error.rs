use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerperError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("API error: {message}")]
    Api { message: String },
    
    #[error("Invalid API key")]
    InvalidApiKey,
}

pub type Result<T> = std::result::Result<T, SerperError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_api_key_error_display() {
        let error = SerperError::InvalidApiKey;
        assert_eq!(error.to_string(), "Invalid API key");
    }

    #[test]
    fn test_api_error_display() {
        let error = SerperError::Api { 
            message: "Rate limit exceeded".to_string() 
        };
        assert_eq!(error.to_string(), "API error: Rate limit exceeded");
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<i32>("invalid json");
        match json_error {
            Err(e) => {
                let serper_error: SerperError = e.into();
                match serper_error {
                    SerperError::Json(_) => {},
                    _ => panic!("Expected Json error variant"),
                }
            },
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_error_variants() {
        let api_key_error = SerperError::InvalidApiKey;
        let api_error = SerperError::Api { message: "test".to_string() };
        
        // Test that we can match on error variants
        match api_key_error {
            SerperError::InvalidApiKey => {},
            _ => panic!("Expected InvalidApiKey variant"),
        }
        
        match api_error {
            SerperError::Api { message } => {
                assert_eq!(message, "test");
            },
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        let success_result: Result<i32> = Ok(42);
        assert_eq!(success_result.unwrap(), 42);

        let error_result: Result<i32> = Err(SerperError::InvalidApiKey);
        assert!(error_result.is_err());
    }
}