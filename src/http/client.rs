/// High-level HTTP client functionality
///
/// This module provides a high-level HTTP client that combines transport
/// layer functionality with Serper API-specific operations.
use crate::{
    core::{
        Result,
        types::{ApiKey, BaseUrl},
    },
    http::transport::{HttpTransport, TransportConfig},
    search::{
        query::SearchQuery,
        response::{ResponseParser, SearchResponse},
    },
};

/// High-level HTTP client for Serper API operations
///
/// This client handles authentication, request formatting, response parsing,
/// and error handling for all Serper API interactions.
#[derive(Debug)]
pub struct SerperHttpClient {
    transport: HttpTransport,
    api_key: ApiKey,
    base_url: BaseUrl,
}

impl SerperHttpClient {
    /// Creates a new HTTP client with the specified API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Serper API key
    ///
    /// # Returns
    ///
    /// Result containing the HTTP client or an error
    pub fn new(api_key: ApiKey) -> Result<Self> {
        let transport = HttpTransport::new()?;
        let base_url = BaseUrl::default();

        Ok(Self {
            transport,
            api_key,
            base_url,
        })
    }

    /// Creates a new HTTP client with custom configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Serper API key
    /// * `base_url` - Custom base URL for the API
    /// * `config` - Transport configuration
    ///
    /// # Returns
    ///
    /// Result containing the HTTP client or an error
    pub fn with_config(
        api_key: ApiKey,
        base_url: BaseUrl,
        config: TransportConfig,
    ) -> Result<Self> {
        let transport = HttpTransport::with_config(config)?;

        Ok(Self {
            transport,
            api_key,
            base_url,
        })
    }

    /// Executes a search query
    ///
    /// # Arguments
    ///
    /// * `query` - The search query to execute
    ///
    /// # Returns
    ///
    /// Result containing the search response or an error
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        // Validate query before sending
        query.validate()?;

        let url = format!("{}/search", self.base_url.as_str());

        let response = self.transport.post_json(&url, &self.api_key, query).await?;

        let search_response = self.transport.parse_json(response).await?;

        // Validate response structure
        ResponseParser::validate_response(&search_response)?;

        Ok(search_response)
    }

    /// Executes multiple search queries in sequence
    ///
    /// # Arguments
    ///
    /// * `queries` - The search queries to execute
    ///
    /// # Returns
    ///
    /// Result containing a vector of search responses or an error
    pub async fn search_multiple(&self, queries: &[SearchQuery]) -> Result<Vec<SearchResponse>> {
        let mut results = Vec::with_capacity(queries.len());

        for query in queries {
            let result = self.search(query).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Executes multiple search queries concurrently
    ///
    /// # Arguments
    ///
    /// * `queries` - The search queries to execute
    /// * `max_concurrent` - Maximum number of concurrent requests
    ///
    /// # Returns
    ///
    /// Result containing a vector of search responses or an error
    pub async fn search_concurrent(
        &self,
        queries: &[SearchQuery],
        max_concurrent: usize,
    ) -> Result<Vec<SearchResponse>> {
        use std::sync::Arc;
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut handles = Vec::new();

        for query in queries {
            let semaphore = Arc::clone(&semaphore);
            let query = query.clone();
            let client = self.clone_for_concurrent();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                client.search(&query).await
            });

            handles.push(handle);
        }

        let mut results = Vec::with_capacity(queries.len());
        for handle in handles {
            let result = handle.await.map_err(|e| {
                crate::core::SerperError::config_error(format!("Task join error: {}", e))
            })??;
            results.push(result);
        }

        Ok(results)
    }

    /// Gets the API key (for debugging/logging purposes)
    pub fn api_key(&self) -> &ApiKey {
        &self.api_key
    }

    /// Gets the base URL
    pub fn base_url(&self) -> &BaseUrl {
        &self.base_url
    }

    /// Gets the transport configuration
    pub fn transport_config(&self) -> &TransportConfig {
        self.transport.config()
    }

    /// Helper method to clone the client for concurrent operations
    ///
    /// This creates a new HTTP transport but reuses the API key and base URL
    fn clone_for_concurrent(&self) -> Self {
        Self {
            transport: HttpTransport::with_config(self.transport.config().clone())
                .expect("Failed to clone transport"),
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

/// Builder for creating HTTP clients with custom configuration
pub struct SerperHttpClientBuilder {
    api_key: Option<ApiKey>,
    base_url: Option<BaseUrl>,
    transport_config: TransportConfig,
}

impl SerperHttpClientBuilder {
    /// Creates a new HTTP client builder
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: None,
            transport_config: TransportConfig::new(),
        }
    }

    /// Sets the API key
    pub fn api_key(mut self, api_key: ApiKey) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the base URL
    pub fn base_url(mut self, base_url: BaseUrl) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Sets the transport configuration
    pub fn transport_config(mut self, config: TransportConfig) -> Self {
        self.transport_config = config;
        self
    }

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.transport_config = self.transport_config.with_timeout(timeout);
        self
    }

    /// Adds a default header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.transport_config = self.transport_config.with_header(key.into(), value.into());
        self
    }

    /// Builds the HTTP client
    pub fn build(self) -> Result<SerperHttpClient> {
        let api_key = self
            .api_key
            .ok_or_else(|| crate::core::SerperError::config_error("API key is required"))?;

        let base_url = self.base_url.unwrap_or_default();

        SerperHttpClient::with_config(api_key, base_url, self.transport_config)
    }
}

impl Default for SerperHttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::ApiKey;

    #[test]
    fn test_client_builder() {
        let api_key = ApiKey::new("test-key".to_string()).unwrap();
        let base_url = BaseUrl::new("https://test.api.com".to_string());

        let builder = SerperHttpClientBuilder::new()
            .api_key(api_key.clone())
            .base_url(base_url.clone())
            .timeout(std::time::Duration::from_secs(60))
            .header("Custom", "Value");

        let client = builder.build().unwrap();
        assert_eq!(client.api_key().as_str(), "test-key");
        assert_eq!(client.base_url().as_str(), "https://test.api.com");
        assert_eq!(
            client.transport_config().timeout,
            std::time::Duration::from_secs(60)
        );
    }

    #[test]
    fn test_client_creation() {
        let api_key = ApiKey::new("test-key".to_string()).unwrap();
        let client = SerperHttpClient::new(api_key).unwrap();

        assert_eq!(client.api_key().as_str(), "test-key");
        assert_eq!(client.base_url().as_str(), "https://google.serper.dev");
    }

    #[test]
    fn test_builder_missing_api_key() {
        let builder = SerperHttpClientBuilder::new();
        let result = builder.build();
        assert!(result.is_err());
    }
}
