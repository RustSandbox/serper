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
use serper_sdk::{SearchService, SearchQuery, SearchResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a search service (with example API key for documentation)
    let service = SearchService::new("demo-key-for-docs".to_string())?;
    
    // Build a search query
    let query = SearchQuery::new("Rust programming".to_string())?
        .with_location("San Francisco".to_string())
        .with_page(1);
    
    // Create a mock response for documentation example
    let response = SearchResponse::new();
    
    // Process results (in real usage, you'd use service.search(&query).await?)
    println!("Query: {}", query.q);
    println!("Location: {:?}", query.location);
    println!("Results found: {}", response.organic_count());
    
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
use serper_sdk::{SearchService, http::TransportConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service with custom configuration
    let transport_config = TransportConfig::default()
        .with_timeout(Duration::from_secs(60));
        
    let service = SearchService::with_config(
        "demo-key-for-docs".to_string(),
        "https://google.serper.dev".to_string(),
        transport_config
    )?;
    
    println!("Service created with custom 60s timeout");
    Ok(())
}
```

### Concurrent Searches

```rust
use serper_sdk::{SearchService, SearchQuery};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service and queries
    let _service = SearchService::new("demo-key-for-docs".to_string())?;
    
    let queries = vec![
        SearchQuery::new("Rust".to_string())?,
        SearchQuery::new("Python".to_string())?,
        SearchQuery::new("JavaScript".to_string())?,
    ];
    
    // In real usage: let results = service.search_concurrent(&queries, Some(3)).await?;
    println!("Prepared {} queries for concurrent execution", queries.len());
    for (i, query) in queries.iter().enumerate() {
        println!("Query {}: {}", i + 1, query.q);
    }
    
    Ok(())
}
```

### Query Builder Pattern

```rust
use serper_sdk::{SearchService, SearchQueryBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _service = SearchService::new("demo-key-for-docs".to_string())?;
    
    // Demonstrate the builder pattern
    let query = SearchQueryBuilder::new()
        .query("machine learning")
        .location("New York")
        .country("us")
        .language("en")
        .page(1)
        .build()?;
    
    println!("Built query: {}", query.q);
    println!("Location: {:?}", query.location);
    println!("Country: {:?}", query.gl);
    println!("Language: {:?}", query.hl);
    println!("Page: {:?}", query.page);
    
    // In real usage: let response = service.search_with(|builder| { ... }).await?;
    Ok(())
}
```
*/

// Core modules
pub mod core;
pub mod search;
pub mod http;
pub mod config;
pub mod utils;

// Re-export main types for convenience
pub use core::{SerperError, Result};
pub use search::{
    SearchQuery, SearchQueryBuilder, SearchResponse, SearchService,
    OrganicResult, AnswerBox, KnowledgeGraph, SearchMetadata
};
pub use config::{SdkConfig, SdkConfigBuilder};

// Legacy compatibility - re-export the main client for backward compatibility
pub use search::SearchService as SerperClient;