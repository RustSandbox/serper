/// Search service orchestration module
/// 
/// This module provides the main search service that orchestrates
/// query building, HTTP requests, and response processing.
use crate::{
    core::{Result, types::ApiKey, types::BaseUrl},
    http::{SerperHttpClient, TransportConfig},
    search::{SearchQuery, SearchQueryBuilder, SearchResponse},
};
use std::time::Duration;

/// Main search service for the Serper SDK
/// 
/// This service provides the primary interface for search operations,
/// combining query building, HTTP client management, and response processing.
#[derive(Debug)]
pub struct SearchService {
    http_client: SerperHttpClient,
}

impl SearchService {
    /// Creates a new search service with the specified API key
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - The Serper API key
    /// 
    /// # Returns
    /// 
    /// Result containing the search service or an error
    pub fn new(api_key: String) -> Result<Self> {
        let api_key = ApiKey::new(api_key)?;
        let http_client = SerperHttpClient::new(api_key)?;

        Ok(Self { http_client })
    }

    /// Creates a new search service with custom configuration
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - The Serper API key
    /// * `base_url` - Custom base URL for the API
    /// * `config` - Transport configuration
    /// 
    /// # Returns
    /// 
    /// Result containing the search service or an error
    pub fn with_config(
        api_key: String,
        base_url: String,
        config: TransportConfig,
    ) -> Result<Self> {
        let api_key = ApiKey::new(api_key)?;
        let base_url = BaseUrl::new(base_url);
        let http_client = SerperHttpClient::with_config(api_key, base_url, config)?;

        Ok(Self { http_client })
    }

    /// Performs a search with the given query
    /// 
    /// # Arguments
    /// 
    /// * `query` - The search query to execute
    /// 
    /// # Returns
    /// 
    /// Result containing the search response or an error
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        self.http_client.search(query).await
    }

    /// Performs a search with a simple query string
    /// 
    /// # Arguments
    /// 
    /// * `query_string` - The search query string
    /// 
    /// # Returns
    /// 
    /// Result containing the search response or an error
    pub async fn search_simple(&self, query_string: &str) -> Result<SearchResponse> {
        let query = SearchQuery::new(query_string.to_string())?;
        self.search(&query).await
    }

    /// Performs multiple searches in sequence
    /// 
    /// # Arguments
    /// 
    /// * `queries` - The search queries to execute
    /// 
    /// # Returns
    /// 
    /// Result containing a vector of search responses or an error
    pub async fn search_multiple(&self, queries: &[SearchQuery]) -> Result<Vec<SearchResponse>> {
        self.http_client.search_multiple(queries).await
    }

    /// Performs multiple searches concurrently
    /// 
    /// # Arguments
    /// 
    /// * `queries` - The search queries to execute
    /// * `max_concurrent` - Maximum number of concurrent requests (default: 5)
    /// 
    /// # Returns
    /// 
    /// Result containing a vector of search responses or an error
    pub async fn search_concurrent(
        &self,
        queries: &[SearchQuery],
        max_concurrent: Option<usize>,
    ) -> Result<Vec<SearchResponse>> {
        let max_concurrent = max_concurrent.unwrap_or(5);
        self.http_client.search_concurrent(queries, max_concurrent).await
    }

    /// Creates a new query builder
    /// 
    /// # Returns
    /// 
    /// A SearchQueryBuilder instance for fluent query construction
    pub fn query_builder(&self) -> SearchQueryBuilder {
        SearchQueryBuilder::new()
    }

    /// Searches with query builder pattern
    /// 
    /// # Arguments
    /// 
    /// * `builder_fn` - Function to configure the query builder
    /// 
    /// # Returns
    /// 
    /// Result containing the search response or an error
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let response = service.search_with(|builder| {
    ///     builder
    ///         .query("rust programming")
    ///         .location("San Francisco")
    ///         .page(1)
    /// }).await?;
    /// ```
    pub async fn search_with<F>(&self, builder_fn: F) -> Result<SearchResponse>
    where
        F: FnOnce(SearchQueryBuilder) -> SearchQueryBuilder,
    {
        let query = builder_fn(self.query_builder()).build()?;
        self.search(&query).await
    }

    /// Gets information about the current service configuration
    pub fn info(&self) -> SearchServiceInfo {
        SearchServiceInfo {
            base_url: self.http_client.base_url().as_str().to_string(),
            timeout: self.http_client.transport_config().timeout,
            user_agent: self.http_client.transport_config().user_agent.clone(),
        }
    }
}

/// Information about the search service configuration
#[derive(Debug, Clone)]
pub struct SearchServiceInfo {
    /// The base URL being used for API requests
    pub base_url: String,
    /// Request timeout duration
    pub timeout: Duration,
    /// User agent string
    pub user_agent: String,
}

/// Builder for creating search services with custom configuration
pub struct SearchServiceBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    transport_config: TransportConfig,
}

impl SearchServiceBuilder {
    /// Creates a new search service builder
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: None,
            transport_config: TransportConfig::new(),
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

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.transport_config = self.transport_config.with_timeout(timeout);
        self
    }

    /// Adds a default header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.transport_config = self.transport_config.with_header(key.into(), value.into());
        self
    }

    /// Sets the user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.transport_config = self.transport_config.with_user_agent(user_agent.into());
        self
    }

    /// Builds the search service
    pub fn build(self) -> Result<SearchService> {
        let api_key = self.api_key
            .ok_or_else(|| crate::core::SerperError::config_error("API key is required"))?;

        match self.base_url {
            Some(base_url) => SearchService::with_config(api_key, base_url, self.transport_config),
            None => {
                let api_key_obj = ApiKey::new(api_key)?;
                let http_client = SerperHttpClient::with_config(
                    api_key_obj,
                    BaseUrl::default(),
                    self.transport_config,
                )?;
                Ok(SearchService { http_client })
            }
        }
    }
}

impl Default for SearchServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_builder() {
        let builder = SearchServiceBuilder::new()
            .api_key("test-key")
            .timeout(Duration::from_secs(60))
            .user_agent("test-agent");

        let service = builder.build().unwrap();
        let info = service.info();
        
        assert_eq!(info.timeout, Duration::from_secs(60));
        assert_eq!(info.user_agent, "test-agent");
        assert_eq!(info.base_url, "https://google.serper.dev");
    }

    #[test]
    fn test_service_creation() {
        let service = SearchService::new("test-key".to_string()).unwrap();
        let info = service.info();
        
        assert_eq!(info.base_url, "https://google.serper.dev");
        assert_eq!(info.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_builder_missing_api_key() {
        let builder = SearchServiceBuilder::new();
        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_query_builder() {
        let service = SearchService::new("test-key".to_string()).unwrap();
        let builder = service.query_builder();
        
        let query = builder
            .query("test")
            .location("Paris")
            .page(1)
            .build()
            .unwrap();
        
        assert_eq!(query.query(), "test");
        assert_eq!(query.location, Some("Paris".to_string()));
        assert_eq!(query.page, Some(1));
    }
}