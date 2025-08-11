# Configuration Module Documentation

## Module Purpose

The `config` module provides comprehensive configuration management for the Serper SDK. It handles SDK settings, environment variable integration, validation, and provides builder patterns for flexible configuration. The module centralizes all configuration concerns and provides a unified interface for managing SDK behavior across different deployment environments.

## API Reference

### `SdkConfig`

Main SDK configuration structure containing all configurable options.

```rust
pub struct SdkConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub max_concurrent_requests: usize,
    pub default_headers: HashMap<String, String>,
    pub user_agent: String,
    pub enable_logging: bool,
}
```

**Fields:**
- `api_key` - API key for authentication (required)
- `base_url` - Base URL for the API (default: "https://google.serper.dev")
- `timeout` - Request timeout duration (default: 30 seconds)
- `max_concurrent_requests` - Maximum concurrent requests (default: 5)
- `default_headers` - Default headers for all requests
- `user_agent` - User agent string (default: "serper-sdk/{version}")
- `enable_logging` - Enable request/response logging (default: false)

**Methods:**

- `new(api_key: String) -> Self`
  - Creates a new configuration with the specified API key and defaults
  - **Parameters:** `api_key` - The Serper API key
  - **Returns:** `SdkConfig` with default values
  - **Default values:**
    - `base_url`: "https://google.serper.dev"
    - `timeout`: 30 seconds
    - `max_concurrent_requests`: 5
    - `default_headers`: {"Content-Type": "application/json"}
    - `user_agent`: "serper-sdk/{version}"
    - `enable_logging`: false

- `from_env() -> Result<Self>`
  - Creates configuration from environment variables
  - **Returns:** `Result<SdkConfig, SerperError>`
  - **Environment Variables:**
    - `SERPER_API_KEY` (required) - API key
    - `SERPER_BASE_URL` (optional) - Base URL
    - `SERPER_TIMEOUT_SECS` (optional) - Timeout in seconds
    - `SERPER_MAX_CONCURRENT` (optional) - Max concurrent requests
    - `SERPER_USER_AGENT` (optional) - User agent string
    - `SERPER_ENABLE_LOGGING` (optional) - "true" to enable logging
  - **Errors:** Returns config error if required environment variables are missing

- `validate(&self) -> Result<()>`
  - Validates the configuration
  - **Returns:** `Result<(), SerperError>`
  - **Validation Rules:**
    - API key cannot be empty
    - Base URL cannot be empty and must start with http:// or https://
    - Timeout must be greater than 0
    - Max concurrent requests must be greater than 0
  - **Errors:** Returns validation error if any rule is violated

- `with_base_url(self, base_url: String) -> Self`
  - Sets the base URL (builder pattern)
  - **Parameters:** `base_url` - The base URL
  - **Returns:** Self for method chaining

- `with_timeout(self, timeout: Duration) -> Self`
  - Sets the timeout (builder pattern)
  - **Parameters:** `timeout` - Timeout duration
  - **Returns:** Self for method chaining

- `with_max_concurrent(self, max_concurrent: usize) -> Self`
  - Sets the maximum concurrent requests (builder pattern)
  - **Parameters:** `max_concurrent` - Maximum concurrent requests
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

- `with_logging(self, enable: bool) -> Self`
  - Enables or disables logging (builder pattern)
  - **Parameters:** `enable` - Whether to enable logging
  - **Returns:** Self for method chaining

### `SdkConfigBuilder`

Builder for creating SDK configurations with validation and flexible construction.

```rust
pub struct SdkConfigBuilder { /* private fields */ }
```

**Methods:**

- `new() -> Self`
  - Creates a new configuration builder
  - **Returns:** `SdkConfigBuilder` instance with default Content-Type header

- `api_key(self, api_key: impl Into<String>) -> Self`
  - Sets the API key (builder pattern)
  - **Parameters:** `api_key` - The API key (accepts anything that converts to String)
  - **Returns:** Self for method chaining

- `base_url(self, base_url: impl Into<String>) -> Self`
  - Sets the base URL (builder pattern)
  - **Parameters:** `base_url` - The base URL (accepts anything that converts to String)
  - **Returns:** Self for method chaining

- `timeout(self, timeout: Duration) -> Self`
  - Sets the timeout (builder pattern)
  - **Parameters:** `timeout` - Timeout duration
  - **Returns:** Self for method chaining

- `max_concurrent(self, max_concurrent: usize) -> Self`
  - Sets the maximum concurrent requests (builder pattern)
  - **Parameters:** `max_concurrent` - Maximum concurrent requests
  - **Returns:** Self for method chaining

- `header(self, key: impl Into<String>, value: impl Into<String>) -> Self`
  - Adds a default header (builder pattern)
  - **Parameters:**
    - `key` - Header name (accepts anything that converts to String)
    - `value` - Header value (accepts anything that converts to String)
  - **Returns:** Self for method chaining

- `user_agent(self, user_agent: impl Into<String>) -> Self`
  - Sets the user agent (builder pattern)
  - **Parameters:** `user_agent` - User agent string (accepts anything that converts to String)
  - **Returns:** Self for method chaining

- `enable_logging(self) -> Self`
  - Enables logging (builder pattern)
  - **Returns:** Self for method chaining

- `build(self) -> Result<SdkConfig>`
  - Builds the configuration with validation
  - **Returns:** `Result<SdkConfig, SerperError>`
  - **Errors:** Returns config error if API key is missing or validation fails

## Usage Examples

### Basic Configuration

```rust
use serper_sdk::config::SdkConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create with API key and defaults
    let config = SdkConfig::new("your-api-key-here".to_string());
    println!("Base URL: {}", config.base_url);
    println!("Timeout: {:?}", config.timeout);
    
    // Validate configuration
    config.validate()?;
    
    Ok(())
}
```

### Configuration from Environment Variables

```rust
use serper_sdk::config::SdkConfig;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set environment variables
    env::set_var("SERPER_API_KEY", "your-api-key");
    env::set_var("SERPER_TIMEOUT_SECS", "60");
    env::set_var("SERPER_MAX_CONCURRENT", "10");
    env::set_var("SERPER_ENABLE_LOGGING", "true");
    
    // Create configuration from environment
    let config = SdkConfig::from_env()?;
    
    assert_eq!(config.timeout.as_secs(), 60);
    assert_eq!(config.max_concurrent_requests, 10);
    assert!(config.enable_logging);
    
    Ok(())
}
```

### Builder Pattern Configuration

```rust
use serper_sdk::config::SdkConfigBuilder;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SdkConfigBuilder::new()
        .api_key("your-api-key")
        .base_url("https://custom.proxy.com")
        .timeout(Duration::from_secs(90))
        .max_concurrent(15)
        .header("X-Custom-Header", "custom-value")
        .header("Authorization", "Bearer token")
        .user_agent("MyApp/2.0")
        .enable_logging()
        .build()?;
    
    println!("Configuration created successfully");
    println!("User agent: {}", config.user_agent);
    println!("Custom headers: {:?}", config.default_headers);
    
    Ok(())
}
```

### Fluent Configuration

```rust
use serper_sdk::config::SdkConfig;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SdkConfig::new("api-key".to_string())
        .with_base_url("https://api.example.com".to_string())
        .with_timeout(Duration::from_secs(120))
        .with_max_concurrent(20)
        .with_header("X-Client-ID".to_string(), "12345".to_string())
        .with_user_agent("Production-Client/1.0".to_string())
        .with_logging(true);
    
    // Validate the configuration
    config.validate()?;
    
    println!("Fluent configuration completed");
    
    Ok(())
}
```

### Configuration for Different Environments

```rust
use serper_sdk::config::{SdkConfig, SdkConfigBuilder};
use std::time::Duration;

#[derive(Debug)]
enum Environment {
    Development,
    Staging,
    Production,
}

fn create_config_for_env(env: Environment, api_key: &str) -> Result<SdkConfig, Box<dyn std::error::Error>> {
    let mut builder = SdkConfigBuilder::new()
        .api_key(api_key);
    
    match env {
        Environment::Development => {
            builder = builder
                .timeout(Duration::from_secs(10))
                .max_concurrent(2)
                .enable_logging()
                .user_agent("serper-sdk-dev/1.0");
        },
        Environment::Staging => {
            builder = builder
                .base_url("https://staging-api.serper.dev")
                .timeout(Duration::from_secs(30))
                .max_concurrent(5)
                .header("X-Environment", "staging")
                .user_agent("serper-sdk-staging/1.0");
        },
        Environment::Production => {
            builder = builder
                .timeout(Duration::from_secs(60))
                .max_concurrent(20)
                .header("X-Environment", "production")
                .header("X-Client-Version", "1.2.3")
                .user_agent("serper-sdk-prod/1.0");
        },
    }
    
    Ok(builder.build()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev_config = create_config_for_env(Environment::Development, "dev-key")?;
    let prod_config = create_config_for_env(Environment::Production, "prod-key")?;
    
    println!("Dev timeout: {:?}", dev_config.timeout);
    println!("Prod timeout: {:?}", prod_config.timeout);
    
    Ok(())
}
```

### Configuration Validation

```rust
use serper_sdk::config::SdkConfig;
use serper_sdk::core::SerperError;
use std::time::Duration;

fn main() {
    // Valid configuration
    let valid_config = SdkConfig::new("valid-key".to_string());
    assert!(valid_config.validate().is_ok());
    
    // Invalid configurations
    let invalid_key_config = SdkConfig::new("".to_string());
    match invalid_key_config.validate() {
        Err(SerperError::Config { message }) => {
            println!("Expected validation error: {}", message);
        },
        _ => panic!("Expected validation error"),
    }
    
    let invalid_url_config = SdkConfig::new("key".to_string())
        .with_base_url("not-a-url".to_string());
    assert!(invalid_url_config.validate().is_err());
    
    let invalid_timeout_config = SdkConfig::new("key".to_string())
        .with_timeout(Duration::from_secs(0));
    assert!(invalid_timeout_config.validate().is_err());
    
    println!("All validation tests passed");
}
```

### Integration with Search Service

```rust
use serper_sdk::{SearchService, config::SdkConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = SdkConfig::new("your-api-key".to_string())
        .with_timeout(Duration::from_secs(45))
        .with_max_concurrent(8)
        .with_user_agent("MySearchApp/1.0".to_string());
    
    // Validate before use
    config.validate()?;
    
    // Use configuration with search service (hypothetical integration)
    // Note: This would require SearchService to accept SdkConfig
    // let service = SearchService::with_config(config)?;
    
    println!("Configuration ready for use");
    
    Ok(())
}
```

## Dependencies

### Internal Dependencies
- `core::error` - For SerperError and Result types

### External Dependencies
- `std::time::Duration` - For timeout configuration
- `std::collections::HashMap` - For storing default headers
- `std::env` - For environment variable access

## Design Principles

1. **Environment Awareness**: First-class support for environment variables
2. **Validation**: Comprehensive validation with clear error messages
3. **Builder Pattern**: Flexible configuration construction
4. **Defaults**: Sensible defaults for all optional settings
5. **Type Safety**: Strong typing for all configuration values
6. **Immutability**: Configuration is immutable after creation

## Environment Variables

The configuration module supports the following environment variables:

| Variable | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `SERPER_API_KEY` | String | Yes | None | Serper API key |
| `SERPER_BASE_URL` | String | No | "https://google.serper.dev" | API base URL |
| `SERPER_TIMEOUT_SECS` | u64 | No | 30 | Request timeout in seconds |
| `SERPER_MAX_CONCURRENT` | usize | No | 5 | Maximum concurrent requests |
| `SERPER_USER_AGENT` | String | No | "serper-sdk/{version}" | User agent string |
| `SERPER_ENABLE_LOGGING` | Boolean | No | false | Enable request/response logging |

### Environment Variable Examples

```bash
# Required
export SERPER_API_KEY="your-actual-api-key"

# Optional customizations
export SERPER_BASE_URL="https://proxy.example.com"
export SERPER_TIMEOUT_SECS="60"
export SERPER_MAX_CONCURRENT="10"
export SERPER_USER_AGENT="MyApp/1.0"
export SERPER_ENABLE_LOGGING="true"
```

## Validation Rules

The configuration validation enforces the following rules:

1. **API Key**: Cannot be empty or contain only whitespace
2. **Base URL**: 
   - Cannot be empty
   - Must start with "http://" or "https://"
   - Must be a valid URL format
3. **Timeout**: Must be greater than 0 seconds
4. **Max Concurrent**: Must be greater than 0
5. **Headers**: Header names and values are not validated (HTTP client handles this)

## Thread Safety

All configuration types are designed to be thread-safe:

- `SdkConfig`: `Send + Sync` - Can be shared across threads
- `SdkConfigBuilder`: `Send` - Can be moved between threads
- All fields use thread-safe types (String, HashMap, primitives)

## Testing

The configuration module includes comprehensive tests covering:

- Default configuration creation
- Environment variable parsing
- Validation rules and error cases
- Builder pattern functionality
- Fluent configuration methods

Run tests with:
```bash
cargo test config::
```

## Best Practices

1. **Environment Variables**: Use environment variables for deployment-specific settings
2. **Validation**: Always call `validate()` after configuration creation
3. **Secrets**: Never hardcode API keys in source code
4. **Timeouts**: Set appropriate timeouts for your use case
5. **Headers**: Use default headers for common request metadata
6. **User Agent**: Set descriptive user agents for API monitoring

## Error Handling

Configuration errors are categorized as `SerperError::Config` and include:

- Missing required environment variables
- Invalid URL formats
- Invalid numeric values in environment variables
- Validation failures (empty fields, zero timeouts, etc.)

All error messages provide clear context about what went wrong and how to fix it.