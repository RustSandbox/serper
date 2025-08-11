/*!
# Serper SDK

A minimalistic yet ergonomic Rust SDK for the Serper Google Search API.

## Features

- **Type-safe API** with comprehensive error handling
- **Modular architecture** with clear separation of concerns
- **Async-first design** with concurrent request support
- **Flexible configuration** with environment variable support
- **Comprehensive testing** with extensive test coverage

## Quick Start

```rust
use serper_sdk::{SearchService, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a search service
    let service = SearchService::new("YOUR_API_KEY".to_string())?;
    
    // Build a search query
    let query = SearchQuery::new("Rust programming".to_string())?
        .with_location("San Francisco".to_string())
        .with_page(1);
    
    // Execute the search
    let response = service.search(&query).await?;
    
    // Process results
    for result in response.organic_results() {
        println!("{}: {}", result.title, result.link);
    }
    
    Ok(())
}
```

## Architecture

The SDK is organized into several focused modules:

- **`core`**: Fundamental types and error handling
- **`search`**: Query construction and response parsing
- **`http`**: Transport layer and HTTP client functionality  
- **`config`**: Configuration management
- **`utils`**: Common utilities and helpers

## Advanced Usage

### Custom Configuration

```rust
use serper_sdk::{SearchServiceBuilder, config::SdkConfig};
use std::time::Duration;

let service = SearchServiceBuilder::new()
    .api_key("YOUR_API_KEY")
    .timeout(Duration::from_secs(60))
    .user_agent("my-app/1.0")
    .build()?;
```

### Concurrent Searches

```rust
let queries = vec![
    SearchQuery::new("Rust".to_string())?,
    SearchQuery::new("Python".to_string())?,
    SearchQuery::new("JavaScript".to_string())?,
];

let results = service.search_concurrent(&queries, Some(3)).await?;
```

### Query Builder Pattern

```rust
let response = service.search_with(|builder| {
    builder
        .query("machine learning")
        .location("New York")
        .country("us")
        .language("en")
        .page(1)
}).await?;
```
*/

// Core modules
pub mod core;
pub mod search;
pub mod http;
pub mod config;
pub mod utils;
pub mod verify;

// Re-export main types for convenience
pub use core::{SerperError, Result};
pub use search::{
    SearchQuery, SearchQueryBuilder, SearchResponse, SearchService,
    OrganicResult, AnswerBox, KnowledgeGraph, SearchMetadata
};
pub use config::{SdkConfig, SdkConfigBuilder};

// Legacy compatibility - re-export the main client for backward compatibility
pub use search::SearchService as SerperClient;