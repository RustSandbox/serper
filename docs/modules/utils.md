# Utils Module Documentation

## Module Purpose

The `utils` module provides common utility functions and helpers used throughout the SDK. It includes validation helpers, formatting utilities, collection manipulation functions, and retry logic for handling transient failures. This module centralizes common functionality to reduce code duplication and ensure consistent behavior across the SDK.

## API Reference

### URL Module (`utils::url`)

Utilities for URL validation and manipulation.

#### Functions

- `validate_url(url: &str) -> Result<()>`
  - Validates that a URL is properly formatted
  - **Parameters:** `url` - The URL string to validate
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if URL is empty or malformed
  - **Example:**
    ```rust
    utils::url::validate_url("https://example.com")?; // OK
    utils::url::validate_url("not-a-url")?; // Error
    ```

- `validate_https(url: &str) -> Result<()>`
  - Validates that a URL uses HTTPS protocol
  - **Parameters:** `url` - The URL string to validate
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if URL doesn't use HTTPS
  - **Example:**
    ```rust
    utils::url::validate_https("https://example.com")?; // OK
    utils::url::validate_https("http://example.com")?; // Error
    ```

- `extract_domain(url: &str) -> Result<String>`
  - Extracts the domain from a URL
  - **Parameters:** `url` - The URL string
  - **Returns:** `Result<String, SerperError>`
  - **Errors:** Returns validation error if URL is invalid or has no domain
  - **Example:**
    ```rust
    let domain = utils::url::extract_domain("https://api.example.com/path")?;
    assert_eq!(domain, "api.example.com");
    ```

### String Module (`utils::string`)

Utilities for string validation and formatting.

#### Functions

- `validate_non_empty(value: &str, field_name: &str) -> Result<()>`
  - Validates that a string is not empty after trimming
  - **Parameters:**
    - `value` - The string to validate
    - `field_name` - Name of the field for error messages
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if string is empty or whitespace-only
  - **Example:**
    ```rust
    utils::string::validate_non_empty("hello", "username")?; // OK
    utils::string::validate_non_empty("  ", "username")?; // Error: "username cannot be empty"
    ```

- `validate_length(value: &str, min_len: Option<usize>, max_len: Option<usize>, field_name: &str) -> Result<()>`
  - Validates string length constraints
  - **Parameters:**
    - `value` - The string to validate
    - `min_len` - Minimum length (optional)
    - `max_len` - Maximum length (optional)
    - `field_name` - Name of the field for error messages
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if length constraints are violated
  - **Example:**
    ```rust
    utils::string::validate_length("hello", Some(3), Some(10), "password")?; // OK
    utils::string::validate_length("hi", Some(3), Some(10), "password")?; // Error: "password must be at least 3 characters"
    ```

- `sanitize(value: &str) -> String`
  - Sanitizes a string by removing control characters
  - **Parameters:** `value` - The string to sanitize
  - **Returns:** A sanitized string with control characters removed (except whitespace)
  - **Example:**
    ```rust
    let clean = utils::string::sanitize("test\x00string\nwith\tspaces");
    assert_eq!(clean, "teststring\nwith\tspaces");
    ```

- `truncate(value: &str, max_len: usize) -> String`
  - Truncates a string to a maximum length with ellipsis
  - **Parameters:**
    - `value` - The string to truncate
    - `max_len` - Maximum length
  - **Returns:** Truncated string with "..." suffix if truncated
  - **Example:**
    ```rust
    let short = utils::string::truncate("short", 10); // "short"
    let long = utils::string::truncate("very long string", 8); // "very ..."
    ```

### Collections Module (`utils::collections`)

Utilities for collection manipulation.

#### Functions

- `merge_hashmaps<K, V>(base: HashMap<K, V>, overlay: HashMap<K, V>) -> HashMap<K, V>`
  - Merges two HashMaps, with values from overlay taking precedence
  - **Parameters:**
    - `base` - The base HashMap
    - `overlay` - The overlay HashMap (values take precedence)
  - **Returns:** A merged HashMap
  - **Example:**
    ```rust
    let mut base = HashMap::new();
    base.insert("a", 1);
    base.insert("b", 2);
    
    let mut overlay = HashMap::new();
    overlay.insert("b", 3);
    overlay.insert("c", 4);
    
    let result = utils::collections::merge_hashmaps(base, overlay);
    // Result: {"a": 1, "b": 3, "c": 4}
    ```

- `filter_map_by_key<K, V, F>(map: HashMap<K, V>, predicate: F) -> HashMap<K, V>`
  - Filters a HashMap by keys matching a predicate
  - **Parameters:**
    - `map` - The HashMap to filter
    - `predicate` - Function to test each key
  - **Returns:** A filtered HashMap containing only entries where predicate returns true
  - **Example:**
    ```rust
    let mut map = HashMap::new();
    map.insert("api_key", "secret");
    map.insert("debug", "true");
    map.insert("api_version", "v1");
    
    let filtered = utils::collections::filter_map_by_key(map, |k| k.starts_with("api_"));
    // Result: {"api_key": "secret", "api_version": "v1"}
    ```

### Retry Module (`utils::retry`)

Utilities for handling transient failures with retry logic.

#### `RetryConfig`

Configuration for retry behavior.

```rust
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub backoff_multiplier: f64,
    pub max_delay: Duration,
}
```

**Fields:**
- `max_attempts` - Maximum number of retry attempts (default: 3)
- `initial_delay` - Initial delay between retries (default: 100ms)
- `backoff_multiplier` - Multiplier for exponential backoff (default: 2.0)
- `max_delay` - Maximum delay between retries (default: 10s)

**Methods:**

- `new() -> Self`
  - Creates a new retry configuration with default values
  - **Returns:** `RetryConfig` with default retry settings

- `with_max_attempts(self, attempts: usize) -> Self`
  - Sets the maximum number of attempts (builder pattern)
  - **Parameters:** `attempts` - Maximum retry attempts
  - **Returns:** Self for method chaining

- `with_initial_delay(self, delay: Duration) -> Self`
  - Sets the initial delay (builder pattern)
  - **Parameters:** `delay` - Initial delay duration
  - **Returns:** Self for method chaining

#### Functions

- `with_retry<F, Fut, T, E>(config: RetryConfig, operation: F) -> Result<T>`
  - Executes a function with retry logic and exponential backoff
  - **Parameters:**
    - `config` - Retry configuration
    - `operation` - Async function to retry
  - **Returns:** `Result<T, SerperError>` - Success result or final error
  - **Behavior:**
    - Retries on any error that converts to SerperError
    - Uses exponential backoff with jitter
    - Stops after max_attempts or on success
  - **Example:**
    ```rust
    let config = RetryConfig::new()
        .with_max_attempts(5)
        .with_initial_delay(Duration::from_millis(200));
    
    let result = utils::retry::with_retry(config, || async {
        // Some operation that might fail transiently
        make_api_request().await
    }).await?;
    ```

## Usage Examples

### URL Validation

```rust
use serper_sdk::utils;

fn validate_api_endpoint(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure URL is valid
    utils::url::validate_url(url)?;
    
    // Ensure it uses HTTPS for security
    utils::url::validate_https(url)?;
    
    // Extract domain for logging
    let domain = utils::url::extract_domain(url)?;
    println!("Validated API endpoint: {}", domain);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    validate_api_endpoint("https://api.example.com/v1")?;
    // validate_api_endpoint("http://insecure.com")?; // Would fail HTTPS check
    
    Ok(())
}
```

### String Processing

```rust
use serper_sdk::utils;

fn process_user_input(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Validate input is not empty
    utils::string::validate_non_empty(input, "user input")?;
    
    // Validate length constraints
    utils::string::validate_length(input, Some(1), Some(100), "user input")?;
    
    // Sanitize the input
    let sanitized = utils::string::sanitize(input);
    
    // Truncate if still too long (defensive)
    let processed = utils::string::truncate(&sanitized, 50);
    
    Ok(processed)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processed = process_user_input("Hello\x00World\nWith some text")?;
    println!("Processed: {}", processed); // "HelloWorld\nWith some text"
    
    let long_input = "This is a very long string that will be truncated";
    let processed_long = process_user_input(long_input)?;
    println!("Processed long: {}", processed_long); // "This is a very long string that will be trunc..."
    
    Ok(())
}
```

### Configuration Merging

```rust
use serper_sdk::utils;
use std::collections::HashMap;

fn merge_configurations() {
    // Base configuration
    let mut base_config = HashMap::new();
    base_config.insert("timeout", "30");
    base_config.insert("retries", "3");
    base_config.insert("endpoint", "https://api.example.com");
    
    // Environment-specific overrides
    let mut env_config = HashMap::new();
    env_config.insert("timeout", "60"); // Override
    env_config.insert("debug", "true"); // Add new
    
    // Merge configurations
    let final_config = utils::collections::merge_hashmaps(base_config, env_config);
    
    println!("Final configuration:");
    for (key, value) in &final_config {
        println!("  {}: {}", key, value);
    }
    // Output:
    // timeout: 60 (overridden)
    // retries: 3
    // endpoint: https://api.example.com
    // debug: true (added)
}

fn filter_sensitive_headers() {
    let mut headers = HashMap::new();
    headers.insert("content-type", "application/json");
    headers.insert("authorization", "Bearer secret-token");
    headers.insert("x-api-key", "secret-key");
    headers.insert("user-agent", "myapp/1.0");
    
    // Filter out sensitive headers for logging
    let safe_headers = utils::collections::filter_map_by_key(headers, |key| {
        !key.to_lowercase().contains("auth") && !key.contains("key")
    });
    
    println!("Safe headers for logging:");
    for (key, value) in &safe_headers {
        println!("  {}: {}", key, value);
    }
    // Output:
    // content-type: application/json
    // user-agent: myapp/1.0
}

fn main() {
    merge_configurations();
    filter_sensitive_headers();
}
```

### Retry Logic

```rust
use serper_sdk::utils;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure retry behavior
    let retry_config = utils::retry::RetryConfig::new()
        .with_max_attempts(5)
        .with_initial_delay(Duration::from_millis(100));
    
    // Simulate an operation that might fail transiently
    let result = utils::retry::with_retry(retry_config, || async {
        // Simulate random failures
        if rand::random::<f32>() < 0.7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Connection temporarily unavailable"
            ));
        }
        Ok("Success!")
    }).await;
    
    match result {
        Ok(value) => println!("Operation succeeded: {}", value),
        Err(e) => println!("Operation failed after retries: {}", e),
    }
    
    Ok(())
}
```

### Complete Validation Pipeline

```rust
use serper_sdk::utils;
use std::collections::HashMap;

struct ApiConfig {
    endpoint: String,
    api_key: String,
    headers: HashMap<String, String>,
}

impl ApiConfig {
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Validate endpoint URL
        utils::url::validate_url(&self.endpoint)?;
        utils::url::validate_https(&self.endpoint)?;
        
        // Validate API key
        utils::string::validate_non_empty(&self.api_key, "API key")?;
        utils::string::validate_length(&self.api_key, Some(10), Some(100), "API key")?;
        
        // Validate headers
        for (name, value) in &self.headers {
            utils::string::validate_non_empty(name, "header name")?;
            utils::string::validate_non_empty(value, "header value")?;
        }
        
        Ok(())
    }
    
    fn sanitize(&mut self) {
        // Sanitize string fields
        self.api_key = utils::string::sanitize(&self.api_key);
        
        // Sanitize headers
        let sanitized_headers: HashMap<String, String> = self.headers
            .drain()
            .map(|(k, v)| (utils::string::sanitize(&k), utils::string::sanitize(&v)))
            .collect();
        
        self.headers = sanitized_headers;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ApiConfig {
        endpoint: "https://api.example.com/v1".to_string(),
        api_key: "sk-1234567890abcdef\x00".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type\n".to_string(), "application/json".to_string());
            h
        },
    };
    
    println!("Before sanitization:");
    println!("API key: {:?}", config.api_key);
    
    // Sanitize first
    config.sanitize();
    
    println!("After sanitization:");
    println!("API key: {:?}", config.api_key);
    
    // Then validate
    config.validate()?;
    
    println!("Configuration is valid!");
    
    Ok(())
}
```

## Dependencies

### Internal Dependencies
- `core::error` - For SerperError and Result types

### External Dependencies
- `url` - For URL parsing and validation
- `tokio::time` - For async sleep in retry logic
- `std::collections::HashMap` - For collection utilities
- `std::time::Duration` - For timing configuration

## Design Principles

1. **Reusability**: Common functionality extracted to prevent duplication
2. **Composability**: Small, focused functions that can be combined
3. **Error Context**: Clear error messages with field names and context
4. **Performance**: Efficient implementations with minimal allocations
5. **Safety**: Input validation and sanitization by default

## Thread Safety

All utility functions are thread-safe:

- **Pure Functions**: Most utilities are pure functions without shared state
- **RetryConfig**: `Send + Sync` - Can be shared across threads
- **No Global State**: No global variables or mutable statics

## Performance Considerations

### String Operations
- `sanitize()` uses iterator-based filtering for efficiency
- `truncate()` handles UTF-8 boundaries correctly
- Minimal string allocations where possible

### Collection Operations
- `merge_hashmaps()` uses HashMap::extend for efficiency
- `filter_map_by_key()` uses iterator chaining to avoid intermediate collections

### Retry Logic
- Uses exponential backoff to reduce load on failing services
- Configurable delays to balance responsiveness and resource usage
- Async-friendly with proper cancellation support

## Testing

The utils module includes comprehensive tests covering:

- URL validation edge cases
- String sanitization and validation
- Collection manipulation
- Retry logic with different failure patterns
- Error message formatting

Run tests with:
```bash
cargo test utils::
```

## Best Practices

1. **Validation**: Always validate inputs at module boundaries
2. **Sanitization**: Sanitize user inputs to remove control characters
3. **Error Messages**: Include field names in validation error messages
4. **Retry Logic**: Use retry logic for transient failures only
5. **Configuration**: Use RetryConfig for consistent retry behavior across the SDK

## Error Handling

Utility functions use appropriate SerperError variants:

- **Validation Errors**: `SerperError::validation_error()` for input validation
- **Configuration Errors**: `SerperError::config_error()` for configuration issues
- **Error Context**: All errors include descriptive messages with context

## Extension Points

The utils module is designed for easy extension:

1. **New Validators**: Add functions to appropriate submodules
2. **Custom Retry Policies**: Extend RetryConfig with new backoff strategies
3. **Additional Collections**: Add new collection utilities as needed
4. **Format Helpers**: Add formatting functions for common data types