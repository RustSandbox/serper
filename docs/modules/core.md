# Core Module Documentation

## Module Purpose

The `core` module provides the fundamental building blocks for the Serper SDK. It contains essential data types, error handling mechanisms, and core abstractions that are used throughout the entire SDK. This module ensures type safety, consistent error handling, and provides the foundation for all other modules.

## API Reference

### Types Module (`core::types`)

#### `ApiKey`

A secure wrapper around API key strings with validation.

```rust
pub struct ApiKey(String);
```

**Methods:**

- `new(key: String) -> Result<Self, SerperError>`
  - Creates a new API key with validation
  - **Parameters:** `key` - The API key string
  - **Returns:** `Result<ApiKey, SerperError>`
  - **Errors:** Returns `InvalidApiKey` if the key is empty or whitespace-only

- `as_str(&self) -> &str`
  - Returns the API key as a string reference
  - **Returns:** String slice containing the API key

- `into_string(self) -> String`
  - Consumes the ApiKey and returns the inner string
  - **Returns:** The API key as an owned String

#### `BaseUrl`

Represents a base URL for API requests.

```rust
pub struct BaseUrl(String);
```

**Methods:**

- `new(url: String) -> Self`
  - Creates a new base URL
  - **Parameters:** `url` - The URL string
  - **Returns:** `BaseUrl` instance

- `as_str(&self) -> &str`
  - Returns the URL as a string reference
  - **Returns:** String slice containing the URL

- `default() -> Self`
  - Returns the default Serper API base URL
  - **Returns:** BaseUrl with "https://google.serper.dev"

#### `Pagination`

Represents pagination parameters for search queries.

```rust
pub struct Pagination {
    pub page: Option<u32>,
    pub num_results: Option<u32>,
}
```

**Methods:**

- `new() -> Self`
  - Creates new pagination with default values (None for both fields)
  - **Returns:** `Pagination` instance

- `with_page(self, page: u32) -> Self`
  - Sets the page number (builder pattern)
  - **Parameters:** `page` - Page number (1-based)
  - **Returns:** Self for method chaining

- `with_num_results(self, num: u32) -> Self`
  - Sets the number of results per page (builder pattern)
  - **Parameters:** `num` - Number of results per page
  - **Returns:** Self for method chaining

#### `Location`

Represents geographical location parameters for search queries.

```rust
pub struct Location {
    pub location: Option<String>,
    pub country_code: Option<String>,
    pub language_code: Option<String>,
}
```

**Methods:**

- `new() -> Self`
  - Creates a new location with no parameters set
  - **Returns:** `Location` instance with all fields as None

- `with_location(self, location: String) -> Self`
  - Sets the location string (builder pattern)
  - **Parameters:** `location` - Human-readable location (e.g., "Paris, France")
  - **Returns:** Self for method chaining

- `with_country(self, country: String) -> Self`
  - Sets the country code (builder pattern)
  - **Parameters:** `country` - Country code (e.g., "fr", "us")
  - **Returns:** Self for method chaining

- `with_language(self, language: String) -> Self`
  - Sets the language code (builder pattern)
  - **Parameters:** `language` - Language code (e.g., "en", "fr")
  - **Returns:** Self for method chaining

### Error Module (`core::error`)

#### `SerperError`

Main error enum for the Serper SDK covering all possible error conditions.

```rust
pub enum SerperError {
    Request(reqwest::Error),
    Json(serde_json::Error),
    Api { message: String },
    InvalidApiKey,
    Config { message: String },
    Validation { message: String },
}
```

**Variants:**

- `Request(reqwest::Error)` - HTTP request failed
- `Json(serde_json::Error)` - JSON parsing failed
- `Api { message: String }` - API returned an error response
- `InvalidApiKey` - Invalid API key provided
- `Config { message: String }` - Configuration error
- `Validation { message: String }` - Input validation error

**Methods:**

- `api_error(message: impl Into<String>) -> Self`
  - Creates a new API error with custom message
  - **Parameters:** `message` - Error message
  - **Returns:** `SerperError::Api`

- `config_error(message: impl Into<String>) -> Self`
  - Creates a new configuration error
  - **Parameters:** `message` - Error message
  - **Returns:** `SerperError::Config`

- `validation_error(message: impl Into<String>) -> Self`
  - Creates a new validation error
  - **Parameters:** `message` - Error message
  - **Returns:** `SerperError::Validation`

- `is_auth_error(&self) -> bool`
  - Checks if the error is related to authentication
  - **Returns:** `true` if authentication-related

- `is_network_error(&self) -> bool`
  - Checks if the error is related to network/transport
  - **Returns:** `true` if network-related

- `is_parse_error(&self) -> bool`
  - Checks if the error is related to data parsing
  - **Returns:** `true` if parsing-related

- `is_api_error(&self) -> bool`
  - Checks if the error is an API error
  - **Returns:** `true` if API-related

#### `Result<T>`

Type alias for Results using SerperError.

```rust
pub type Result<T> = std::result::Result<T, SerperError>;
```

## Usage Examples

### Creating and Validating an API Key

```rust
use serper_sdk::core::types::ApiKey;

// Create a valid API key
let api_key = ApiKey::new("your-api-key-here".to_string())?;
println!("API key: {}", api_key.as_str());

// This will fail with InvalidApiKey error
let invalid_key = ApiKey::new("".to_string());
assert!(invalid_key.is_err());
```

### Building Location and Pagination Parameters

```rust
use serper_sdk::core::types::{Location, Pagination};

// Build location parameters
let location = Location::new()
    .with_location("San Francisco".to_string())
    .with_country("us".to_string())
    .with_language("en".to_string());

// Build pagination parameters
let pagination = Pagination::new()
    .with_page(2)
    .with_num_results(20);
```

### Error Handling

```rust
use serper_sdk::core::{SerperError, Result};

fn validate_input(input: &str) -> Result<()> {
    if input.is_empty() {
        return Err(SerperError::validation_error("Input cannot be empty"));
    }
    Ok(())
}

// Using the error
match validate_input("") {
    Ok(_) => println!("Valid input"),
    Err(e) if e.is_auth_error() => println!("Authentication error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

### Working with Base URLs

```rust
use serper_sdk::core::types::BaseUrl;

// Use default URL
let default_url = BaseUrl::default();
assert_eq!(default_url.as_str(), "https://google.serper.dev");

// Use custom URL
let custom_url = BaseUrl::new("https://my-proxy.com".to_string());
assert_eq!(custom_url.as_str(), "https://my-proxy.com");
```

## Dependencies

### Internal Dependencies
- None (this is the foundational module)

### External Dependencies
- `thiserror` - For derive macro on error types
- `serde` - For serialization traits on data types

## Design Principles

1. **Type Safety**: All core types enforce invariants at construction time
2. **Builder Pattern**: Fluent interfaces for complex type construction
3. **Error Context**: Rich error types with classification methods
4. **Zero-Cost Abstractions**: Minimal runtime overhead
5. **Validation**: Early validation prevents invalid states

## Thread Safety

All types in the core module are `Send` and `Sync` where appropriate:
- `ApiKey`: `Send + Sync` (contains only String)
- `BaseUrl`: `Send + Sync` (contains only String)
- `Location`: `Send + Sync` (contains only optional Strings)
- `Pagination`: `Send + Sync` (contains only primitives)
- `SerperError`: `Send + Sync` (error types are thread-safe)

## Testing

The core module includes comprehensive unit tests covering:
- Valid and invalid API key creation
- Builder pattern functionality
- Error classification and construction
- Type validation and edge cases

Run tests with:
```bash
cargo test core::
```