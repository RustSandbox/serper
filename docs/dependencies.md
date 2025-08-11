# Module Dependencies and Relationships

## Dependency Graph

This document outlines the dependencies between modules in the Serper SDK, showing how they interact and the data flow between them.

```
┌─────────┐    ┌─────────┐    ┌─────────┐
│  core   │    │ utils   │    │ config  │
│         │    │         │    │         │
└─────┬───┘    └─────────┘    └───┬─────┘
      │                            │
      │        ┌─────────┐         │
      └────────┤  http   │─────────┘
               │         │
               └────┬────┘
                    │
               ┌────▼────┐
               │ search  │
               │         │
               └─────────┘
```

## Module Hierarchy

### Foundation Layer
- **`core`** - Foundational types and error handling
- **`utils`** - Common utilities and helpers  
- **`config`** - Configuration management

### Transport Layer  
- **`http`** - HTTP transport and client functionality

### Application Layer
- **`search`** - High-level search operations

## Detailed Dependencies

### Core Module (`core`)
**Internal Dependencies:** None (foundational module)

**External Dependencies:**
- `thiserror` - Error derive macros
- `serde` - Serialization traits

**Exports to other modules:**
- `SerperError` - Used by all modules for error handling
- `Result<T>` - Used by all modules for return types
- `ApiKey` - Used by http and config modules
- `BaseUrl` - Used by http and config modules
- `Location`, `Pagination` - Used by search module

**Usage by other modules:**
- `http` - Uses ApiKey, BaseUrl, and error types
- `search` - Uses all core types and error handling
- `config` - Uses error types for validation
- `utils` - Uses error types for validation functions

### Utils Module (`utils`)
**Internal Dependencies:**
- `core::error` - For SerperError and Result types

**External Dependencies:**
- `url` - URL parsing and validation
- `tokio::time` - Async sleep functionality
- `std::collections::HashMap` - Collection utilities

**Exports to other modules:**
- URL validation functions - Used by config and http modules
- String validation functions - Used by all modules
- Collection utilities - Used by config and http modules
- Retry logic - Used by http and search modules

**Usage by other modules:**
- `core` - Could use string validation (currently doesn't)
- `http` - Uses retry logic and URL validation
- `search` - Uses validation functions and retry logic
- `config` - Uses URL and string validation extensively

### Config Module (`config`)
**Internal Dependencies:**
- `core::error` - For error types and Result

**External Dependencies:**
- `std::time::Duration` - Timeout configuration
- `std::collections::HashMap` - Header storage
- `std::env` - Environment variable access

**Exports to other modules:**
- `SdkConfig` - Used by search and http modules
- `SdkConfigBuilder` - Used by applications

**Usage by other modules:**
- `http` - Uses timeout and header configuration
- `search` - Could use SdkConfig for service configuration
- Applications use this for centralized configuration

### HTTP Module (`http`)
**Internal Dependencies:**
- `core` - All types (ApiKey, BaseUrl, errors, Result)
- `search::query` - SearchQuery type
- `search::response` - SearchResponse type and parsing

**External Dependencies:**
- `reqwest` - HTTP client implementation
- `serde` - Serialization for request bodies
- `tokio` - Async runtime and synchronization

**Exports to other modules:**
- `HttpTransport` - Low-level HTTP operations
- `SerperHttpClient` - High-level HTTP client
- Transport builders and configuration

**Usage by other modules:**
- `search` - Uses SerperHttpClient for API operations
- Could be used directly by applications for custom HTTP operations

### Search Module (`search`)
**Internal Dependencies:**
- `core` - All types and error handling
- `http` - SerperHttpClient and transport configuration
- Could use `utils` - For validation and retry logic
- Could use `config` - For service configuration

**External Dependencies:**
- `serde` - Serialization/deserialization
- `tokio` - Async operations
- `url` - URL parsing in response processing
- `std::collections::HashMap` - Response metadata

**Exports to other modules:**
- `SearchService` - Main service interface
- `SearchQuery`, `SearchQueryBuilder` - Query construction
- `SearchResponse` and related types - Response handling
- All response parsing utilities

**Usage by other modules:**
- This is the top-level module used by applications
- No other internal modules depend on search

## Data Flow

### Search Operation Flow
1. **Application** creates `SearchQuery` using `search` module
2. **Search** module validates query using `core` types
3. **Search** module uses `http` module to make API request
4. **HTTP** module uses `core::ApiKey` and `core::BaseUrl` for authentication
5. **HTTP** module serializes request using `SearchQuery`
6. **HTTP** module makes HTTP request using `reqwest`
7. **HTTP** module parses response into `SearchResponse`
8. **Search** module returns `SearchResponse` to application

### Configuration Flow
1. **Application** creates `SdkConfig` using `config` module
2. **Config** module validates configuration using `utils` validation
3. **Config** module uses `core` error types for validation errors
4. **Application** uses config to create `SearchService`
5. **Search** service uses config to configure `http` client

### Error Propagation
1. **Core** module defines `SerperError` enum
2. **All modules** use `core::Result<T>` for error handling
3. **Utils** module creates validation errors using `core::SerperError`
4. **HTTP** module converts `reqwest::Error` to `SerperError::Request`
5. **Search** module propagates all errors to application

## Coupling Analysis

### Low Coupling (Good)
- `core` has no internal dependencies
- `utils` only depends on `core` for error types
- `config` only depends on `core` for error types

### Medium Coupling (Acceptable)
- `http` depends on `core` and `search` types (circular dependency handled via careful API design)
- `search` depends on `core` and `http`

### High Coupling (Areas for improvement)
- `search::query` and `search::response` could be more independent
- `http::client` tightly coupled to search types

## Module Interface Contracts

### Core Module Interface
```rust
// Error handling
pub enum SerperError { /* ... */ }
pub type Result<T> = std::result::Result<T, SerperError>;

// Type safety
pub struct ApiKey(String);
pub struct BaseUrl(String);
pub struct Location { /* ... */ }
pub struct Pagination { /* ... */ }
```

### HTTP Module Interface
```rust
// Transport abstraction  
pub struct HttpTransport;
impl HttpTransport {
    pub async fn post_json<T>(&self, url: &str, api_key: &ApiKey, body: &T) -> Result<Response>;
}

// High-level client
pub struct SerperHttpClient;
impl SerperHttpClient {
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse>;
}
```

### Search Module Interface
```rust
// Service interface
pub struct SearchService;
impl SearchService {
    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse>;
    pub async fn search_multiple(&self, queries: &[SearchQuery]) -> Result<Vec<SearchResponse>>;
}

// Query construction
pub struct SearchQuery { /* ... */ }
pub struct SearchQueryBuilder { /* ... */ }

// Response handling
pub struct SearchResponse { /* ... */ }
```

## Dependency Inversion

The SDK uses dependency inversion in several places:

### HTTP Transport Abstraction
- `SearchService` depends on `SerperHttpClient` abstraction
- `SerperHttpClient` depends on `HttpTransport` abstraction
- This allows for easy mocking and testing

### Configuration Abstraction  
- Services accept configuration objects rather than individual parameters
- This allows for flexible configuration without changing service interfaces

### Error Abstraction
- All modules use `core::SerperError` rather than exposing underlying errors
- This provides a consistent error handling interface

## Testing Dependencies

### Unit Testing
- Each module can be tested independently
- `core` module has no dependencies (easy to test)
- `utils` module only needs `core` for error types
- `config` module only needs `core` and standard library

### Integration Testing
- `http` module needs mocking for network operations
- `search` module needs `http` mocking
- Full integration tests exercise all modules together

### Test Utilities
- Common test utilities could be extracted to a `test_utils` module
- Mock implementations could be provided for key interfaces

## Future Improvements

### Reducing Coupling
1. **Extract Response Types**: Move response types to a separate module
2. **Plugin Architecture**: Allow custom transport implementations
3. **Async Traits**: Use async traits for better abstraction

### Improving Testability
1. **Dependency Injection**: Make dependencies more explicit
2. **Mock Traits**: Provide mock implementations for testing
3. **Test Builders**: Create test data builders for complex types

### Performance Optimizations
1. **Lazy Loading**: Load heavy dependencies only when needed
2. **Connection Pooling**: Better connection reuse across modules
3. **Caching**: Add caching layer between modules

## Module Stability

### Stable Interfaces (Unlikely to change)
- `core::SerperError` - Error types are foundational
- `core::Result<T>` - Result type alias is standard
- `SearchService::search()` - Main API method is stable

### Evolving Interfaces (May change)
- HTTP transport configuration - May add new options
- Query builder methods - May add new parameters
- Response parsing - May add new response types

### Experimental Interfaces (Likely to change)
- Retry configuration - May be redesigned
- Concurrent search limits - May become more sophisticated
- Custom transport plugins - Not yet implemented

## Backwards Compatibility

### API Stability Promise
- `SearchService` public API will remain stable
- `SearchQuery` and `SearchResponse` will maintain backwards compatibility
- Core error types will not change

### Extension Strategy
- New functionality added via new methods rather than changing existing ones
- Optional parameters added via builder patterns
- New response fields added as optional

### Migration Path
- Deprecated methods will be marked clearly
- Migration guides provided for breaking changes
- Old APIs supported for at least 2 major versions