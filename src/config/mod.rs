/// Configuration management module
///
/// This module provides configuration structures and utilities for
/// managing SDK settings, environment variables, and default values.
use crate::core::{Result, SerperError};
use std::collections::HashMap;
use std::time::Duration;

/// Main SDK configuration
///
/// This struct contains all configuration options for the Serper SDK,
/// with sensible defaults and environment variable support.
#[derive(Debug, Clone)]
pub struct SdkConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub base_url: String,
    /// Request timeout duration
    pub timeout: Duration,
    /// Maximum number of concurrent requests
    pub max_concurrent_requests: usize,
    /// Default headers to include with all requests
    pub default_headers: HashMap<String, String>,
    /// User agent string
    pub user_agent: String,
    /// Enable request/response logging
    pub enable_logging: bool,
}

impl SdkConfig {
    /// Creates a new configuration with the specified API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Serper API key
    ///
    /// # Returns
    ///
    /// A new SdkConfig with default values
    pub fn new(api_key: String) -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_key,
            base_url: "https://google.serper.dev".to_string(),
            timeout: Duration::from_secs(30),
            max_concurrent_requests: 5,
            default_headers,
            user_agent: format!("serper-sdk/{}", env!("CARGO_PKG_VERSION")),
            enable_logging: false,
        }
    }

    /// Creates configuration from environment variables
    ///
    /// Expected environment variables:
    /// - `SERPER_API_KEY` (required)
    /// - `SERPER_BASE_URL` (optional)
    /// - `SERPER_TIMEOUT_SECS` (optional)
    /// - `SERPER_MAX_CONCURRENT` (optional)
    /// - `SERPER_USER_AGENT` (optional)
    /// - `SERPER_ENABLE_LOGGING` (optional)
    ///
    /// # Returns
    ///
    /// Result containing the configuration or an error if required variables are missing
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("SERPER_API_KEY").map_err(|_| {
            SerperError::config_error("SERPER_API_KEY environment variable is required")
        })?;

        let mut config = Self::new(api_key);

        if let Ok(base_url) = std::env::var("SERPER_BASE_URL") {
            config.base_url = base_url;
        }

        if let Ok(timeout_str) = std::env::var("SERPER_TIMEOUT_SECS")
            && let Ok(timeout_secs) = timeout_str.parse::<u64>()
        {
            config.timeout = Duration::from_secs(timeout_secs);
        }

        if let Ok(max_concurrent_str) = std::env::var("SERPER_MAX_CONCURRENT")
            && let Ok(max_concurrent) = max_concurrent_str.parse::<usize>()
        {
            config.max_concurrent_requests = max_concurrent;
        }

        if let Ok(user_agent) = std::env::var("SERPER_USER_AGENT") {
            config.user_agent = user_agent;
        }

        if let Ok(enable_logging_str) = std::env::var("SERPER_ENABLE_LOGGING") {
            config.enable_logging = enable_logging_str.to_lowercase() == "true";
        }

        Ok(config)
    }

    /// Validates the configuration
    ///
    /// # Returns
    ///
    /// Result indicating whether the configuration is valid
    pub fn validate(&self) -> Result<()> {
        if self.api_key.trim().is_empty() {
            return Err(SerperError::config_error("API key cannot be empty"));
        }

        if self.base_url.trim().is_empty() {
            return Err(SerperError::config_error("Base URL cannot be empty"));
        }

        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(SerperError::config_error(
                "Base URL must start with http:// or https://",
            ));
        }

        if self.timeout.as_secs() == 0 {
            return Err(SerperError::config_error("Timeout must be greater than 0"));
        }

        if self.max_concurrent_requests == 0 {
            return Err(SerperError::config_error(
                "Max concurrent requests must be greater than 0",
            ));
        }

        Ok(())
    }

    /// Sets the base URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// Sets the timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the maximum concurrent requests
    pub fn with_max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.max_concurrent_requests = max_concurrent;
        self
    }

    /// Adds a default header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.default_headers.insert(key, value);
        self
    }

    /// Sets the user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Enables or disables logging
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }
}

/// Builder for creating SDK configurations
pub struct SdkConfigBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_concurrent_requests: Option<usize>,
    default_headers: HashMap<String, String>,
    user_agent: Option<String>,
    enable_logging: bool,
}

impl SdkConfigBuilder {
    /// Creates a new configuration builder
    pub fn new() -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            api_key: None,
            base_url: None,
            timeout: None,
            max_concurrent_requests: None,
            default_headers,
            user_agent: None,
            enable_logging: false,
        }
    }

    /// Sets the API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the maximum concurrent requests
    pub fn max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.max_concurrent_requests = Some(max_concurrent);
        self
    }

    /// Adds a default header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// Sets the user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Enables logging
    pub fn enable_logging(mut self) -> Self {
        self.enable_logging = true;
        self
    }

    /// Builds the configuration
    pub fn build(self) -> Result<SdkConfig> {
        let api_key = self
            .api_key
            .ok_or_else(|| SerperError::config_error("API key is required"))?;

        let mut config = SdkConfig::new(api_key);

        if let Some(base_url) = self.base_url {
            config.base_url = base_url;
        }

        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }

        if let Some(max_concurrent) = self.max_concurrent_requests {
            config.max_concurrent_requests = max_concurrent;
        }

        config.default_headers = self.default_headers;

        if let Some(user_agent) = self.user_agent {
            config.user_agent = user_agent;
        }

        config.enable_logging = self.enable_logging;

        config.validate()?;
        Ok(config)
    }
}

impl Default for SdkConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = SdkConfig::new("test-key".to_string());
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.base_url, "https://google.serper.dev");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = SdkConfigBuilder::new()
            .api_key("test-key")
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60))
            .max_concurrent(10)
            .header("Custom", "Value")
            .user_agent("custom-agent")
            .enable_logging()
            .build()
            .unwrap();

        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.base_url, "https://custom.api.com");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_concurrent_requests, 10);
        assert_eq!(config.user_agent, "custom-agent");
        assert!(config.enable_logging);
        assert_eq!(
            config.default_headers.get("Custom"),
            Some(&"Value".to_string())
        );
    }

    #[test]
    fn test_config_validation() {
        // Valid config
        let config = SdkConfig::new("valid-key".to_string());
        assert!(config.validate().is_ok());

        // Invalid API key
        let config = SdkConfig::new("".to_string());
        assert!(config.validate().is_err());

        // Invalid base URL
        let config = SdkConfig::new("key".to_string()).with_base_url("invalid-url".to_string());
        assert!(config.validate().is_err());

        // Invalid timeout
        let config = SdkConfig::new("key".to_string()).with_timeout(Duration::from_secs(0));
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_builder_missing_api_key() {
        let builder = SdkConfigBuilder::new();
        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_fluent_configuration() {
        let config = SdkConfig::new("key".to_string())
            .with_base_url("https://test.com".to_string())
            .with_timeout(Duration::from_secs(45))
            .with_max_concurrent(8)
            .with_header("X-Test".to_string(), "value".to_string())
            .with_user_agent("test-agent".to_string())
            .with_logging(true);

        assert_eq!(config.base_url, "https://test.com");
        assert_eq!(config.timeout, Duration::from_secs(45));
        assert_eq!(config.max_concurrent_requests, 8);
        assert_eq!(config.user_agent, "test-agent");
        assert!(config.enable_logging);
    }
}
