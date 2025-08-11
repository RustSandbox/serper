# HTTP Module Documentation

## Module Purpose

The `http` module provides HTTP transport layer abstractions and high-level client functionality for interacting with the Serper API. It separates transport concerns from business logic, enabling easy testing, middleware insertion, and transport layer customization. The module is designed with clean abstractions that make it easy to swap underlying HTTP implementations or add cross-cutting concerns like retry logic, rate limiting, or request logging.

## API Reference

### Transport Module (`http::transport`)

#### `TransportConfig`

Configuration for HTTP transport behavior.

```rust
pub struct TransportConfig {
    pub timeout: Duration,
    pub default_headers: HashMap<String, String>,
    pub user_agent: String,
}
```

**Methods:**

- `new() -> Self`
  - Creates a new transport configuration with default values
  - **Returns:** `TransportConfig` with 30s timeout, JSON content-type header
  - **Default timeout:** 30 seconds
  - **Default headers:** `Content-Type: application/json`
  - **Default user agent:** `serper-sdk/{version}`

- `with_timeout(self, timeout: Duration) -> Self`
  - Sets the request timeout (builder pattern)
  - **Parameters:** `timeout` - Request timeout duration
  - **Returns:** Self for method chaining

- `with_header(self, key: String, value: String) -> Self`
  - Adds a default header (builder pattern)
  - **Parameters:** 
    - `key` - Header name
    - `value` - Header value
  - **Returns:** Self for method chaining

- `with_user_agent(self, user_agent: String) -> Self`
  - Sets the user agent (builder pattern)
  - **Parameters:** `user_agent` - User agent string
  - **Returns:** Self for method chaining

#### `HttpTransport`

Low-level HTTP transport implementation handling request/response operations.

```rust
pub struct HttpTransport { /* private fields */ }
```

**Methods:**

- `new() -> Result<Self>`
  - Creates a new HTTP transport with default configuration
  - **Returns:** `Result<HttpTransport, SerperError>`
  - **Errors:** Returns error if HTTP client creation fails

- `with_config(config: TransportConfig) -> Result<Self>`
  - Creates a new HTTP transport with custom configuration
  - **Parameters:** `config` - Transport configuration
  - **Returns:** `Result<HttpTransport, SerperError>`
  - **Errors:** Returns error if HTTP client creation fails

- `post_json<T: Serialize>(&self, url: &str, api_key: &ApiKey, body: &T) -> Result<Response>`
  - Makes a POST request with JSON body
  - **Parameters:**
    - `url` - The request URL
    - `api_key` - API key for authentication
    - `body` - Request body (must implement Serialize)
  - **Returns:** `Result<reqwest::Response, SerperError>`
  - **Errors:** Returns Request error for HTTP failures, Api error for non-success status codes

- `get(&self, url: &str, api_key: &ApiKey) -> Result<Response>`
  - Makes a GET request
  - **Parameters:**
    - `url` - The request URL  
    - `api_key` - API key for authentication
  - **Returns:** `Result<reqwest::Response, SerperError>`
  - **Errors:** Returns Request error for HTTP failures, Api error for non-success status codes

- `parse_json<T>(&self, response: Response) -> Result<T>`
  - Parses a response as JSON
  - **Parameters:** `response` - HTTP response to parse
  - **Returns:** `Result<T, SerperError>` where T implements DeserializeOwned
  - **Errors:** Returns Request error if JSON parsing fails

- `config(&self) -> &TransportConfig`
  - Gets the current transport configuration
  - **Returns:** Reference to the transport configuration

#### `HttpTransportBuilder`

Builder for creating HTTP transports with custom configuration.

```rust
pub struct HttpTransportBuilder { /* private fields */ }
```

**Methods:**

- `new() -> Self`
  - Creates a new transport builder
  - **Returns:** `HttpTransportBuilder` instance

- `timeout(self, timeout: Duration) -> Self`
  - Sets the request timeout (builder pattern)
  - **Parameters:** `timeout` - Request timeout duration
  - **Returns:** Self for method chaining

- `header(self, key: impl Into<String>, value: impl Into<String>) -> Self`
  - Adds a default header (builder pattern)
  - **Parameters:**
    - `key` - Header name
    - `value` - Header value
  - **Returns:** Self for method chaining

- `user_agent(self, user_agent: impl Into<String>) -> Self`
  - Sets the user agent (builder pattern)
  - **Parameters:** `user_agent` - User agent string
  - **Returns:** Self for method chaining

- `build(self) -> Result<HttpTransport>`
  - Builds the HTTP transport
  - **Returns:** `Result<HttpTransport, SerperError>`
  - **Errors:** Returns error if transport creation fails

### Client Module (`http::client`)

#### `SerperHttpClient`

High-level HTTP client combining transport with Serper API-specific operations.

```rust
pub struct SerperHttpClient { /* private fields */ }
```

**Methods:**

- `new(api_key: ApiKey) -> Result<Self>`
  - Creates a new HTTP client with the specified API key
  - **Parameters:** `api_key` - The Serper API key
  - **Returns:** `Result<SerperHttpClient, SerperError>`
  - **Errors:** Returns error if client creation fails

- `with_config(api_key: ApiKey, base_url: BaseUrl, config: TransportConfig) -> Result<Self>`
  - Creates a new HTTP client with custom configuration
  - **Parameters:**
    - `api_key` - The Serper API key
    - `base_url` - Custom base URL for the API
    - `config` - Transport configuration
  - **Returns:** `Result<SerperHttpClient, SerperError>`
  - **Errors:** Returns error if client creation fails

- `search(&self, query: &SearchQuery) -> Result<SearchResponse>`
  - Executes a search query
  - **Parameters:** `query` - The search query to execute
  - **Returns:** `Result<SearchResponse, SerperError>`
  - **Errors:** Returns validation error for invalid queries, network errors, or API errors

- `search_multiple(&self, queries: &[SearchQuery]) -> Result<Vec<SearchResponse>>`
  - Executes multiple search queries in sequence
  - **Parameters:** `queries` - Array of search queries to execute
  - **Returns:** `Result<Vec<SearchResponse>, SerperError>`
  - **Errors:** Returns error on first failed request

- `search_concurrent(&self, queries: &[SearchQuery], max_concurrent: usize) -> Result<Vec<SearchResponse>>`
  - Executes multiple search queries concurrently
  - **Parameters:**
    - `queries` - Array of search queries to execute
    - `max_concurrent` - Maximum number of concurrent requests
  - **Returns:** `Result<Vec<SearchResponse>, SerperError>`
  - **Errors:** Returns error if any request fails

- `api_key(&self) -> &ApiKey`
  - Gets the API key (for debugging/logging purposes)
  - **Returns:** Reference to the API key

- `base_url(&self) -> &BaseUrl`
  - Gets the base URL
  - **Returns:** Reference to the base URL

- `transport_config(&self) -> &TransportConfig`
  - Gets the transport configuration
  - **Returns:** Reference to the transport configuration

#### `SerperHttpClientBuilder`

Builder for creating HTTP clients with custom configuration.

```rust
pub struct SerperHttpClientBuilder { /* private fields */ }
```

**Methods:**

- `new() -> Self`
  - Creates a new HTTP client builder
  - **Returns:** `SerperHttpClientBuilder` instance

- `api_key(self, api_key: ApiKey) -> Self`
  - Sets the API key (builder pattern)
  - **Parameters:** `api_key` - The API key
  - **Returns:** Self for method chaining

- `base_url(self, base_url: BaseUrl) -> Self`
  - Sets the base URL (builder pattern)
  - **Parameters:** `base_url` - The base URL
  - **Returns:** Self for method chaining

- `timeout(self, timeout: Duration) -> Self`
  - Sets the request timeout (builder pattern)
  - **Parameters:** `timeout` - Timeout duration
  - **Returns:** Self for method chaining

- `header(self, key: impl Into<String>, value: impl Into<String>) -> Self`
  - Adds a default header (builder pattern)
  - **Parameters:**
    - `key` - Header name
    - `value` - Header value
  - **Returns:** Self for method chaining

- `build(self) -> Result<SerperHttpClient>`
  - Builds the HTTP client
  - **Returns:** `Result<SerperHttpClient, SerperError>`
  - **Errors:** Returns error if API key is missing or client creation fails

## Usage Examples

### Basic HTTP Transport Usage

```rust
use serper_sdk::http::transport::{HttpTransport, TransportConfig};
use serper_sdk::core::types::ApiKey;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create transport with custom configuration
    let config = TransportConfig::new()
        .with_timeout(Duration::from_secs(60))
        .with_header("Custom-Header".to_string(), "value".to_string())
        .with_user_agent("my-app/1.0".to_string());
    
    let transport = HttpTransport::with_config(config)?;
    let api_key = ApiKey::new("your-api-key".to_string())?;
    
    // Make a request
    let response = transport.post_json(
        "https://google.serper.dev/search",
        &api_key,
        &serde_json::json!({"q": "test query"})
    ).await?;
    
    // Parse response
    let search_response: SearchResponse = transport.parse_json(response).await?;
    
    Ok(())
}
```

### HTTP Client with Builder Pattern

```rust
use serper_sdk::http::client::SerperHttpClientBuilder;
use serper_sdk::core::types::{ApiKey, BaseUrl};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = ApiKey::new("your-api-key".to_string())?;
    let base_url = BaseUrl::new("https://custom-proxy.com".to_string());
    
    let client = SerperHttpClientBuilder::new()
        .api_key(api_key)
        .base_url(base_url)
        .timeout(Duration::from_secs(90))
        .header("X-Custom", "value")
        .build()?;
    
    // Use the client
    let query = SearchQuery::new("test".to_string())?;
    let response = client.search(&query).await?;
    
    Ok(())
}
```

### Concurrent Searches with HTTP Client

```rust
use serper_sdk::http::client::SerperHttpClient;
use serper_sdk::search::query::SearchQuery;
use serper_sdk::core::types::ApiKey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = ApiKey::new("your-api-key".to_string())?;
    let client = SerperHttpClient::new(api_key)?;
    
    let queries = vec![
        SearchQuery::new("Rust programming".to_string())?,
        SearchQuery::new("Python programming".to_string())?,
        SearchQuery::new("JavaScript programming".to_string())?,
    ];
    
    // Execute up to 2 searches concurrently
    let results = client.search_concurrent(&queries, 2).await?;
    
    for (i, response) in results.iter().enumerate() {
        println!("Query {}: {} results", i + 1, response.organic_count());
    }
    
    Ok(())
}
```

### Custom Transport Configuration

```rust
use serper_sdk::http::transport::HttpTransportBuilder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = HttpTransportBuilder::new()
        .timeout(Duration::from_secs(120))
        .header("X-Request-ID", "12345")
        .header("X-Client-Version", "1.2.3")
        .user_agent("MyApp/1.0 (contact@example.com)")
        .build()?;
    
    // Transport is ready to use
    let config = transport.config();
    println!("Timeout: {:?}", config.timeout);
    println!("User agent: {}", config.user_agent);
    
    Ok(())
}
```

### Error Handling in HTTP Operations

```rust
use serper_sdk::http::client::SerperHttpClient;
use serper_sdk::core::{SerperError, types::ApiKey};

#[tokio::main]
async fn main() {
    let api_key = ApiKey::new("your-api-key".to_string()).unwrap();
    let client = SerperHttpClient::new(api_key).unwrap();
    
    let query = SearchQuery::new("test".to_string()).unwrap();
    
    match client.search(&query).await {
        Ok(response) => {
            println!("Success: {} results", response.organic_count());
        },
        Err(SerperError::Request(e)) => {
            println!("Network error: {}", e);
        },
        Err(SerperError::Api { message }) => {
            println!("API error: {}", message);
        },
        Err(SerperError::Json(e)) => {
            println!("JSON parsing error: {}", e);
        },
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}
```

## Dependencies

### Internal Dependencies
- `core::types` - For ApiKey, BaseUrl types
- `core::error` - For SerperError and Result types
- `search::query` - For SearchQuery type
- `search::response` - For SearchResponse type and ResponseParser

### External Dependencies
- `reqwest` - HTTP client implementation
- `serde` - For serialization traits
- `tokio` - For async runtime and synchronization primitives
- `std::time` - For Duration type
- `std::collections::HashMap` - For headers storage

## Design Principles

1. **Separation of Concerns**: Transport layer separated from business logic
2. **Configurability**: Comprehensive configuration options with sane defaults
3. **Error Transparency**: Clear error propagation and categorization
4. **Async First**: All operations are async with proper cancellation support
5. **Testability**: Clean interfaces for mocking and testing
6. **Performance**: Connection reuse and concurrent execution support

## Thread Safety

All HTTP module types are designed for concurrent use:

- `HttpTransport`: `Send + Sync` - Can be shared across tasks
- `SerperHttpClient`: `Send + Sync` - Internal cloning for concurrent operations
- `TransportConfig`: `Send + Sync` - Immutable configuration
- All builder types: `Send` - Can be moved between tasks

## Error Handling

The HTTP module provides comprehensive error handling:

### Error Types
- **`SerperError::Request`** - Network/transport failures
- **`SerperError::Api`** - HTTP error status codes (4xx, 5xx)
- **`SerperError::Json`** - Response parsing failures
- **`SerperError::Validation`** - Request validation failures

### Error Context
- API errors include HTTP status codes and descriptions
- Network errors preserve underlying reqwest error information
- JSON errors provide parsing context

### Retry Behavior
- No automatic retries at the HTTP layer (handled at higher levels)
- Transient errors are distinguished from permanent failures
- Proper error classification enables retry logic in consumers

## Performance Considerations

### Connection Management
- HTTP clients reuse connections automatically
- Connection pools are managed by underlying reqwest
- Keep-alive is enabled by default

### Concurrent Requests
- Built-in support for concurrent operations
- Semaphore-based concurrency limiting
- Shared transport instances for efficiency

### Memory Management
- Streaming JSON parsing where possible
- Efficient header management with HashMap storage
- Minimal copying of request/response data

## Testing

The HTTP module includes comprehensive tests covering:

- Transport configuration and creation
- HTTP request/response handling
- Error scenarios and status codes
- Concurrent operation behavior
- Builder pattern functionality

Run tests with:
```bash
cargo test http::
```

### Testing with Mocks

The module design enables easy testing with mock servers:

```rust
use mockito::Server;

#[tokio::test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;
    
    let mock = server.mock("POST", "/search")
        .with_status(429)  // Rate limited
        .with_body("Rate limit exceeded")
        .create_async()
        .await;
    
    let api_key = ApiKey::new("test-key".to_string()).unwrap();
    let base_url = BaseUrl::new(server.url());
    let client = SerperHttpClient::with_config(
        api_key,
        base_url,
        TransportConfig::new()
    ).unwrap();
    
    let query = SearchQuery::new("test".to_string()).unwrap();
    let result = client.search(&query).await;
    
    assert!(matches!(result, Err(SerperError::Api { .. })));
    mock.assert_async().await;
}
```