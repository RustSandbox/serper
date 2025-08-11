/// Core data types for the Serper SDK
/// 
/// This module defines the fundamental data structures used throughout the SDK,
/// including API keys, URLs, and common identifiers.
use serde::{Deserialize, Serialize};

/// Represents a Serper API key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiKey(String);

impl ApiKey {
    /// Creates a new API key from a string
    /// 
    /// # Arguments
    /// 
    /// * `key` - The API key string
    /// 
    /// # Returns
    /// 
    /// Result containing the ApiKey or an error if invalid
    pub fn new(key: String) -> Result<Self, crate::core::error::SerperError> {
        if key.trim().is_empty() {
            return Err(crate::core::error::SerperError::InvalidApiKey);
        }
        Ok(ApiKey(key))
    }

    /// Returns the API key as a string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the ApiKey and returns the inner string
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Represents a base URL for API requests
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseUrl(String);

impl BaseUrl {
    /// Creates a new base URL
    pub fn new(url: String) -> Self {
        BaseUrl(url)
    }

    /// Returns the URL as a string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the default Serper API base URL
    pub fn new_default() -> Self {
        BaseUrl("https://google.serper.dev".to_string())
    }
}

impl Default for BaseUrl {
    fn default() -> Self {
        BaseUrl("https://google.serper.dev".to_string())
    }
}

/// Represents pagination parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    /// Page number (1-based)
    pub page: Option<u32>,
    /// Number of results per page
    pub num_results: Option<u32>,
}

impl Pagination {
    /// Creates new pagination with default values
    pub fn new() -> Self {
        Self {
            page: None,
            num_results: None,
        }
    }

    /// Sets the page number
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page
    pub fn with_num_results(mut self, num: u32) -> Self {
        self.num_results = Some(num);
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents geographical location parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// Human-readable location (e.g., "Paris, France")
    pub location: Option<String>,
    /// Country code (e.g., "fr")
    pub country_code: Option<String>,
    /// Language code (e.g., "en")
    pub language_code: Option<String>,
}

impl Location {
    /// Creates a new location with no parameters set
    pub fn new() -> Self {
        Self {
            location: None,
            country_code: None,
            language_code: None,
        }
    }

    /// Sets the location string
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the country code
    pub fn with_country(mut self, country: String) -> Self {
        self.country_code = Some(country);
        self
    }

    /// Sets the language code
    pub fn with_language(mut self, language: String) -> Self {
        self.language_code = Some(language);
        self
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_creation() {
        let key = ApiKey::new("test-key".to_string()).unwrap();
        assert_eq!(key.as_str(), "test-key");
    }

    #[test]
    fn test_api_key_empty_fails() {
        let result = ApiKey::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_base_url() {
        let url = BaseUrl::new("https://example.com".to_string());
        assert_eq!(url.as_str(), "https://example.com");
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new()
            .with_page(2)
            .with_num_results(20);
        
        assert_eq!(pagination.page, Some(2));
        assert_eq!(pagination.num_results, Some(20));
    }

    #[test]
    fn test_location() {
        let location = Location::new()
            .with_location("Paris".to_string())
            .with_country("fr".to_string())
            .with_language("en".to_string());
        
        assert_eq!(location.location, Some("Paris".to_string()));
        assert_eq!(location.country_code, Some("fr".to_string()));
        assert_eq!(location.language_code, Some("en".to_string()));
    }
}