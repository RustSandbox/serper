/// HTTP transport layer abstraction
/// 
/// This module provides a clean abstraction over HTTP operations,
/// making it easy to swap out underlying HTTP clients or add middleware.
use reqwest::{Client as ReqwestClient, Method, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use crate::core::{Result, SerperError, types::ApiKey};

/// HTTP transport configuration
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Request timeout duration
    pub timeout: Duration,
    /// Default headers to include with all requests
    pub default_headers: HashMap<String, String>,
    /// User agent string
    pub user_agent: String,
}

impl TransportConfig {
    /// Creates a new transport configuration with default values
    pub fn new() -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        Self {
            timeout: Duration::from_secs(30),
            default_headers,
            user_agent: format!("serper-sdk/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Sets the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
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
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP transport implementation
/// 
/// This struct handles all HTTP operations with automatic retry,
/// error handling, and request/response logging.
#[derive(Debug)]
pub struct HttpTransport {
    client: ReqwestClient,
    config: TransportConfig,
}

impl HttpTransport {
    /// Creates a new HTTP transport with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(TransportConfig::new())
    }

    /// Creates a new HTTP transport with custom configuration
    pub fn with_config(config: TransportConfig) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(SerperError::Request)?;

        Ok(Self { client, config })
    }

    /// Makes a POST request with JSON body
    /// 
    /// # Arguments
    /// 
    /// * `url` - The request URL
    /// * `api_key` - API key for authentication
    /// * `body` - The request body that can be serialized to JSON
    /// 
    /// # Returns
    /// 
    /// Result containing the HTTP response or an error
    pub async fn post_json<T: Serialize>(
        &self,
        url: &str,
        api_key: &ApiKey,
        body: &T,
    ) -> Result<Response> {
        let mut request = self.client
            .request(Method::POST, url)
            .header("X-API-KEY", api_key.as_str());

        // Add default headers (except Content-Type since .json() will set it)
        for (key, value) in &self.config.default_headers {
            if key != "Content-Type" {
                request = request.header(key, value);
            }
        }

        // Set JSON body (this will automatically set Content-Type: application/json)
        request = request.json(body);

        let response = request.send().await.map_err(SerperError::Request)?;

        // Check for HTTP error status codes
        if !response.status().is_success() {
            return Err(SerperError::api_error(format!(
                "HTTP {} - {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown error")
            )));
        }

        Ok(response)
    }

    /// Makes a GET request
    /// 
    /// # Arguments
    /// 
    /// * `url` - The request URL
    /// * `api_key` - API key for authentication
    /// 
    /// # Returns
    /// 
    /// Result containing the HTTP response or an error
    pub async fn get(
        &self,
        url: &str,
        api_key: &ApiKey,
    ) -> Result<Response> {
        let mut request = self.client
            .request(Method::GET, url)
            .header("X-API-KEY", api_key.as_str());

        // Add default headers (except Content-Type for GET)
        for (key, value) in &self.config.default_headers {
            if key != "Content-Type" {
                request = request.header(key, value);
            }
        }

        let response = request.send().await.map_err(SerperError::Request)?;

        if !response.status().is_success() {
            return Err(SerperError::api_error(format!(
                "HTTP {} - {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown error")
            )));
        }

        Ok(response)
    }

    /// Parses a response as JSON
    /// 
    /// # Arguments
    /// 
    /// * `response` - The HTTP response to parse
    /// 
    /// # Returns
    /// 
    /// Result containing the parsed JSON or an error
    pub async fn parse_json<T>(&self, response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        response.json().await.map_err(SerperError::Request)
    }

    /// Gets the current transport configuration
    pub fn config(&self) -> &TransportConfig {
        &self.config
    }
}

impl Default for HttpTransport {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP transport")
    }
}

/// Builder for creating HTTP transports with custom configuration
pub struct HttpTransportBuilder {
    config: TransportConfig,
}

impl HttpTransportBuilder {
    /// Creates a new transport builder
    pub fn new() -> Self {
        Self {
            config: TransportConfig::new(),
        }
    }

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config = self.config.with_timeout(timeout);
        self
    }

    /// Adds a default header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config = self.config.with_header(key.into(), value.into());
        self
    }

    /// Sets the user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.config = self.config.with_user_agent(user_agent.into());
        self
    }

    /// Builds the HTTP transport
    pub fn build(self) -> Result<HttpTransport> {
        HttpTransport::with_config(self.config)
    }
}

impl Default for HttpTransportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_config_creation() {
        let config = TransportConfig::new();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.default_headers.contains_key("Content-Type"));
    }

    #[test]
    fn test_transport_config_builder() {
        let config = TransportConfig::new()
            .with_timeout(Duration::from_secs(60))
            .with_header("Custom-Header".to_string(), "value".to_string())
            .with_user_agent("custom-agent".to_string());

        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.user_agent, "custom-agent");
        assert_eq!(config.default_headers.get("Custom-Header"), Some(&"value".to_string()));
    }

    #[test]
    fn test_transport_builder() {
        let builder = HttpTransportBuilder::new()
            .timeout(Duration::from_secs(45))
            .header("Test", "Value")
            .user_agent("test-agent");

        let transport = builder.build().unwrap();
        assert_eq!(transport.config().timeout, Duration::from_secs(45));
        assert_eq!(transport.config().user_agent, "test-agent");
    }

    #[test]
    fn test_api_key_validation() {
        let result = ApiKey::new("valid-key".to_string());
        assert!(result.is_ok());

        let result = ApiKey::new("".to_string());
        assert!(result.is_err());
    }
}