/// Module verification tests
/// 
/// These tests demonstrate that the modular architecture is working correctly
/// and all modules integrate properly.

#[cfg(test)]
mod verification_tests {
    use crate::core::{SerperError, Result};
    use crate::utils::{string, url, collections};
    use crate::search::{SearchQuery, SearchResponse};
    use crate::config::SdkConfig;
    use std::collections::HashMap;

    #[test]
    fn test_core_module_functionality() {
        // Test error creation and handling
        let error = SerperError::validation_error("test error");
        assert!(matches!(error, SerperError::Validation { .. }));

        // Test result handling
        let success: Result<String> = Ok("success".to_string());
        assert!(success.is_ok());
    }

    #[test] 
    fn test_utils_module_functionality() {
        // Test string utilities
        assert!(string::validate_non_empty("test", "field").is_ok());
        assert!(string::validate_non_empty("", "field").is_err());
        
        let sanitized = string::sanitize("test\x00string");
        assert_eq!(sanitized, "teststring");

        // Test URL utilities
        assert!(url::validate_url("https://example.com").is_ok());
        assert!(url::validate_https("https://example.com").is_ok());
        assert!(url::validate_https("http://example.com").is_err());

        // Test collection utilities
        let mut base = HashMap::new();
        base.insert("a", 1);
        let mut overlay = HashMap::new();
        overlay.insert("b", 2);
        
        let merged = collections::merge_hashmaps(base, overlay);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_search_module_functionality() {
        // Test query creation
        let query = SearchQuery::new("test query".to_string()).unwrap();
        assert_eq!(query.q, "test query");

        // Test response creation
        let response = SearchResponse::new();
        assert!(!response.has_results());
        assert_eq!(response.organic_count(), 0);
    }

    #[test]
    fn test_config_module_functionality() {
        // Test config creation
        let config = SdkConfig::new("test-api-key".to_string());
        assert!(config.validate().is_ok());
        
        // Test validation
        let invalid_config = SdkConfig::new("".to_string());
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_modular_integration() {
        // Test that modules work together
        
        // 1. Create configuration using config module
        let config = SdkConfig::new("test-key".to_string());
        assert!(config.validate().is_ok());

        // 2. Validate input using utils module
        let query_text = "rust programming";
        assert!(string::validate_non_empty(query_text, "query").is_ok());

        // 3. Create query using search module
        let query = SearchQuery::new(query_text.to_string()).unwrap();
        assert_eq!(query.q, query_text);

        // 4. Test error handling from core module
        let empty_query = SearchQuery::new("".to_string());
        assert!(empty_query.is_err());
        assert!(matches!(empty_query.unwrap_err(), SerperError::Validation { .. }));
    }
}