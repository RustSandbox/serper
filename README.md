# Serper SDK

A minimalistic yet ergonomic Rust SDK for the [Serper Google Search API](https://serper.dev). Built with a focus on type safety, modularity, and developer experience.

[![Crates.io](https://img.shields.io/crates/v/serper-sdk.svg)](https://crates.io/crates/serper-sdk)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://rustsandbox.github.io/serper/serper_sdk/)
[![License](https://img.shields.io/crates/l/serper-sdk.svg)](LICENSE)
[![Build Status](https://github.com/RustSandbox/serper/workflows/CI/badge.svg)](https://github.com/RustSandbox/serper/actions)
[![Downloads](https://img.shields.io/crates/d/serper-sdk.svg)](https://crates.io/crates/serper-sdk)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

## Features

- 🔒 **Type-safe API** with comprehensive error handling
- 🏗️ **Modular architecture** with clear separation of concerns  
- ⚡ **Async-first design** with concurrent request support
- 🔧 **Flexible configuration** with environment variable support
- 🧪 **Comprehensive testing** with extensive test coverage
- 📚 **Rich documentation** with examples and API references

## Quick Start

### 1. Get Your API Key

First, sign up at [serper.dev](https://serper.dev) to get your free API key. The service offers generous free tier limits.

### 2. Add to Your Project

Add this to your `Cargo.toml`:

```toml
[dependencies]
serper-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Basic usage:

```rust
use serper_sdk::{SearchService, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a search service (API key from SERPER_API_KEY env var)
    let api_key = std::env::var("SERPER_API_KEY")
        .expect("Please set SERPER_API_KEY environment variable");
    let service = SearchService::new(api_key)?;
    
    // Build a search query
    let query = SearchQuery::new("Hamze Ghalebi CTO at Remolab".to_string())?
        .with_location("Paris".to_string())
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

The SDK is organized into focused modules with clear responsibilities:

- **`core`** - Fundamental types and error handling
- **`search`** - Query construction and response parsing
- **`http`** - Transport layer and HTTP client functionality
- **`config`** - Configuration management with environment variable support
- **`utils`** - Common utilities and validation helpers

For detailed architecture information, see [ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Examples

### Advanced Query Building

```rust
use serper_sdk::{SearchService, SearchQueryBuilder};

let service = SearchService::new("YOUR_API_KEY".to_string())?;

// Using SearchQueryBuilder
let query = SearchQueryBuilder::new()
    .query("Hamze Ghalebi CTO at Remolab")
    .location("Paris")
    .country("fr")
    .language("en")
    .page(1)
    .num_results(20)
    .build()?;

let response = service.search(&query).await?;
```

### Builder Pattern Search

```rust
let response = service.search_with(|builder| {
    builder
        .query("Hamze Ghalebi CTO at Remolab")
        .location("Paris")
        .country("fr")
        .page(1)
}).await?;
```

### Concurrent Searches

```rust
let queries = vec![
    SearchQuery::new("Hamze Ghalebi CTO at Remolab".to_string())?,
    SearchQuery::new("Hamze Ghalebi Remolab technology".to_string())?,
    SearchQuery::new("Remolab France innovation".to_string())?,
];

// Execute up to 3 searches concurrently
let results = service.search_concurrent(&queries, Some(3)).await?;

for (i, response) in results.iter().enumerate() {
    println!("Query {}: {} results", i + 1, response.organic_count());
}
```

### Custom Configuration

```rust
use serper_sdk::{SearchServiceBuilder, config::SdkConfig};
use std::time::Duration;

// Using service builder
// Option 1: Using environment variables (recommended)
let config = SdkConfig::from_env()?;
let service = SearchService::new(config.api_key)?;

// Option 2: Manual configuration
let api_key = std::env::var("SERPER_API_KEY")?;
let service = SearchServiceBuilder::new()
    .api_key(api_key)
    .timeout(Duration::from_secs(60))
    .user_agent("my-app/1.0")
    .build()?;

// Using configuration object  
let api_key = std::env::var("SERPER_API_KEY")?;
let config = SdkConfig::new(api_key)
    .with_timeout(Duration::from_secs(60))
    .with_max_concurrent(10)
    .with_logging(true);
```

### Environment Variables

Set environment variables:

```bash
export SERPER_API_KEY="your-api-key"
export SERPER_TIMEOUT_SECS="60"
export SERPER_MAX_CONCURRENT="10"
export SERPER_ENABLE_LOGGING="true"
```

Then use in your code:

```rust
use serper_sdk::config::SdkConfig;

let config = SdkConfig::from_env()?;
// Configuration loaded from environment variables
```

### Processing Different Result Types

```rust
let response = service.search(&query).await?;

// Direct answers
if let Some(answer_box) = &response.answer_box {
    if let Some(answer) = &answer_box.answer {
        println!("Direct answer: {}", answer);
    }
}

// Knowledge graph
if let Some(kg) = &response.knowledge_graph {
    if let Some(title) = &kg.title {
        println!("Knowledge graph: {}", title);
    }
}

// Organic results
for result in response.organic_results() {
    println!("Result: {} ({})", 
        result.title, 
        result.domain().unwrap_or("unknown")
    );
    
    if result.has_snippet() {
        println!("  {}", result.snippet_or_default());
    }
}
```

## Error Handling

The SDK provides comprehensive error handling with specific error types:

```rust
use serper_sdk::core::SerperError;

match service.search(&query).await {
    Ok(response) => {
        println!("Success: {} results", response.organic_count());
    },
    Err(SerperError::InvalidApiKey) => {
        println!("Invalid API key provided");
    },
    Err(SerperError::Api { message }) => {
        println!("API error: {}", message);
    },
    Err(SerperError::Request(e)) => {
        println!("Network error: {}", e);
    },
    Err(SerperError::Json(e)) => {
        println!("JSON parsing error: {}", e);
    },
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

## Response Types

The SDK provides rich response types for different result categories:

### Organic Results
```rust
pub struct OrganicResult {
    pub title: String,
    pub link: String,
    pub snippet: Option<String>,
    pub position: u32,
    // ... additional fields
}
```

### Answer Box
```rust
pub struct AnswerBox {
    pub answer: Option<String>,
    pub snippet: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
}
```

### Knowledge Graph
```rust
pub struct KnowledgeGraph {
    pub title: Option<String>,
    pub description: Option<String>,
    pub entity_type: Option<String>,
    pub website: Option<String>,
    // ... additional attributes
}
```

## Configuration Options

### SdkConfig Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `api_key` | `String` | Required | Serper API key |
| `base_url` | `String` | `"https://google.serper.dev"` | API base URL |
| `timeout` | `Duration` | `30s` | Request timeout |
| `max_concurrent_requests` | `usize` | `5` | Max concurrent requests |
| `default_headers` | `HashMap<String, String>` | `{"Content-Type": "application/json"}` | Default headers |
| `user_agent` | `String` | `"serper-sdk/{version}"` | User agent string |
| `enable_logging` | `bool` | `false` | Enable request/response logging |

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERPER_API_KEY` | API key (required) | None |
| `SERPER_BASE_URL` | API base URL | `"https://google.serper.dev"` |
| `SERPER_TIMEOUT_SECS` | Timeout in seconds | `30` |
| `SERPER_MAX_CONCURRENT` | Max concurrent requests | `5` |
| `SERPER_USER_AGENT` | Custom user agent | `"serper-sdk/{version}"` |
| `SERPER_ENABLE_LOGGING` | Enable logging (`"true"`/`"false"`) | `false` |

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test search::
cargo test http::
cargo test core::

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

The SDK includes comprehensive tests:
- **54 total tests** across all modules
- **Unit tests** for individual module functionality  
- **Integration tests** for module interactions
- **Edge case tests** for error conditions and boundary cases

## Documentation

### API Documentation

Generate and view the API documentation:

```bash
cargo doc --open
```

### Module Documentation

Detailed documentation for each module is available in the `docs/modules/` directory:

- [Core Module](docs/modules/core.md) - Fundamental types and error handling
- [Search Module](docs/modules/search.md) - Query construction and response handling
- [HTTP Module](docs/modules/http.md) - Transport layer and HTTP client
- [Config Module](docs/modules/config.md) - Configuration management
- [Utils Module](docs/modules/utils.md) - Common utilities and helpers

### Architecture Documentation

- [Architecture Overview](docs/ARCHITECTURE.md) - Complete architecture documentation
- [Module Dependencies](docs/dependencies.md) - Module relationships and dependencies

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/RustSandbox/serper.git
cd serper
```

2. Install dependencies:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Check formatting:
```bash
cargo fmt --check
```

5. Run clippy:
```bash
cargo clippy -- -D warnings
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## Documentation

- 📚 **[Primary Documentation](https://rustsandbox.github.io/serper/serper_sdk/)** - Complete API reference and examples on GitHub Pages
- 📖 **[docs.rs Mirror](https://docs.rs/serper-sdk)** - Alternative API reference on docs.rs
- 🔧 **[Local Documentation](target/doc/serper_sdk/index.html)** - Generate locally with `cargo doc --open`

## Support

- 🐛 [Issue Tracker](https://github.com/RustSandbox/serper/issues)
- 💬 [Discussions](https://github.com/RustSandbox/serper/discussions)

## Author

This SDK is created and maintained by **Hamze Ghalebi**, CTO at [Remolab](https://remolab.fr). Remolab is a technology company focused on innovative software solutions and API integrations.

## Related Projects

- [Serper API](https://serper.dev) - The official Serper Google Search API
- [reqwest](https://github.com/seanmonstar/reqwest) - The HTTP client used by this SDK
- [serde](https://github.com/serde-rs/serde) - Serialization framework used for JSON handling

