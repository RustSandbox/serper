# Search Module Documentation

## Module Purpose

The `search` module provides comprehensive functionality for building search queries, handling responses, and orchestrating search operations. It serves as the high-level interface for search functionality, combining query construction, response parsing, and service orchestration into a cohesive API.

## API Reference

### Query Module (`search::query`)

#### `SearchQuery`

Represents a search query with all possible parameters for the Serper API.

```rust
pub struct SearchQuery {
    pub q: String,
    pub location: Option<String>,
    pub gl: Option<String>,
    pub hl: Option<String>,
    pub page: Option<u32>,
    pub num: Option<u32>,
}
```

**Methods:**

- `new(query: String) -> Result<Self>`
  - Creates a new search query with validation
  - **Parameters:** `query` - The search query string (cannot be empty)
  - **Returns:** `Result<SearchQuery, SerperError>`
  - **Errors:** Returns validation error if query is empty

- `with_location(self, location: String) -> Self`
  - Sets the location for the search query (builder pattern)
  - **Parameters:** `location` - Location string (e.g., "Paris, France")
  - **Returns:** Self for method chaining

- `with_country(self, country: String) -> Self`
  - Sets the country code for the search query (builder pattern)
  - **Parameters:** `country` - Country code (e.g., "fr", "us")
  - **Returns:** Self for method chaining

- `with_language(self, language: String) -> Self`
  - Sets the language code for the search query (builder pattern)
  - **Parameters:** `language` - Language code (e.g., "en", "fr")
  - **Returns:** Self for method chaining

- `with_page(self, page: u32) -> Self`
  - Sets the page number for pagination (builder pattern)
  - **Parameters:** `page` - Page number (1-based)
  - **Returns:** Self for method chaining

- `with_num_results(self, num: u32) -> Self`
  - Sets the number of results per page (builder pattern)
  - **Parameters:** `num` - Number of results (1-100)
  - **Returns:** Self for method chaining

- `validate(&self) -> Result<()>`
  - Validates the search query parameters
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if parameters are invalid

- `query(&self) -> &str`
  - Gets the query string
  - **Returns:** String slice containing the query

- `has_location_params(&self) -> bool`
  - Checks if the query has location parameters
  - **Returns:** `true` if location, country, or language is set

- `has_pagination_params(&self) -> bool`
  - Checks if the query has pagination parameters
  - **Returns:** `true` if page or num_results is set

#### `SearchQueryBuilder`

Builder for creating search queries with validation.

```rust
pub struct SearchQueryBuilder { /* private fields */ }
```

**Methods:**

- `new() -> Self`
  - Creates a new search query builder
  - **Returns:** `SearchQueryBuilder` instance

- `query(self, query: impl Into<String>) -> Self`
  - Sets the search query string (builder pattern)
  - **Parameters:** `query` - The search query
  - **Returns:** Self for method chaining

- `location(self, location: impl Into<String>) -> Self`
  - Sets the location (builder pattern)
  - **Parameters:** `location` - The location string
  - **Returns:** Self for method chaining

- `country(self, country: impl Into<String>) -> Self`
  - Sets the country code (builder pattern)
  - **Parameters:** `country` - The country code
  - **Returns:** Self for method chaining

- `language(self, language: impl Into<String>) -> Self`
  - Sets the language code (builder pattern)
  - **Parameters:** `language` - The language code
  - **Returns:** Self for method chaining

- `page(self, page: u32) -> Self`
  - Sets the page number (builder pattern)
  - **Parameters:** `page` - The page number
  - **Returns:** Self for method chaining

- `num_results(self, num: u32) -> Self`
  - Sets the number of results (builder pattern)
  - **Parameters:** `num` - The number of results
  - **Returns:** Self for method chaining

- `build(self) -> Result<SearchQuery>`
  - Builds the search query with validation
  - **Returns:** `Result<SearchQuery, SerperError>`
  - **Errors:** Returns validation error if required fields are missing or invalid

### Response Module (`search::response`)

#### `SearchResponse`

Complete search response from the Serper API.

```rust
pub struct SearchResponse {
    pub search_metadata: Option<SearchMetadata>,
    pub organic: Option<Vec<OrganicResult>>,
    pub answer_box: Option<AnswerBox>,
    pub knowledge_graph: Option<KnowledgeGraph>,
    pub related_questions: Option<Vec<RelatedQuestion>>,
    pub shopping: Option<Vec<ShoppingResult>>,
    pub news: Option<Vec<NewsResult>>,
}
```

**Methods:**

- `new() -> Self`
  - Creates a new empty search response
  - **Returns:** `SearchResponse` with all fields as None

- `has_results(&self) -> bool`
  - Checks if the response has any results
  - **Returns:** `true` if any result type is present and non-empty

- `organic_count(&self) -> usize`
  - Gets the number of organic results
  - **Returns:** Count of organic results (0 if none)

- `organic_results(&self) -> &[OrganicResult]`
  - Gets organic results as a slice
  - **Returns:** Slice of organic results (empty if none)

- `first_result(&self) -> Option<&OrganicResult>`
  - Gets the first organic result if available
  - **Returns:** Reference to first result or None

- `extract_urls(&self) -> Vec<&str>`
  - Extracts all URLs from organic results
  - **Returns:** Vector of URL string slices

#### `OrganicResult`

Individual organic search result.

```rust
pub struct OrganicResult {
    pub title: String,
    pub link: String,
    pub snippet: Option<String>,
    pub position: u32,
    pub extra: HashMap<String, serde_json::Value>,
}
```

**Methods:**

- `new(title: String, link: String, position: u32) -> Self`
  - Creates a new organic result
  - **Parameters:** 
    - `title` - Result title
    - `link` - Result URL
    - `position` - Position in results (1-based)
  - **Returns:** `OrganicResult` instance

- `has_snippet(&self) -> bool`
  - Checks if the result has a snippet
  - **Returns:** `true` if snippet is present

- `snippet_or_default(&self) -> &str`
  - Gets the snippet text or a default message
  - **Returns:** Snippet text or "No description available"

- `domain(&self) -> Option<&str>`
  - Gets the domain from the URL
  - **Returns:** Domain string slice or None if URL is invalid

#### Other Response Types

- `SearchMetadata` - Contains search request metadata (ID, status, timing)
- `AnswerBox` - Direct answers to queries
- `KnowledgeGraph` - Entity information from knowledge graph
- `RelatedQuestion` - "People also ask" questions
- `ShoppingResult` - Shopping/product results
- `NewsResult` - News article results

#### `ResponseParser`

Utility for parsing and validating responses.

```rust
pub struct ResponseParser;
```

**Methods:**

- `parse_response(json_str: &str) -> Result<SearchResponse>`
  - Parses a JSON response into a SearchResponse
  - **Parameters:** `json_str` - The JSON response string
  - **Returns:** `Result<SearchResponse, SerperError>`
  - **Errors:** Returns JSON error if parsing fails

- `validate_response(response: &SearchResponse) -> Result<()>`
  - Validates that a response has the expected structure
  - **Parameters:** `response` - The response to validate
  - **Returns:** `Result<(), SerperError>`
  - **Errors:** Returns validation error if structure is invalid

### Service Module (`search::service`)

#### `SearchService`

Main search service orchestrating all search operations.

```rust
pub struct SearchService { /* private fields */ }
```

**Methods:**

- `new(api_key: String) -> Result<Self>`
  - Creates a new search service with the specified API key
  - **Parameters:** `api_key` - The Serper API key
  - **Returns:** `Result<SearchService, SerperError>`
  - **Errors:** Returns error if API key is invalid

- `search(&self, query: &SearchQuery) -> Result<SearchResponse>`
  - Performs a search with the given query
  - **Parameters:** `query` - The search query to execute
  - **Returns:** `Result<SearchResponse, SerperError>`
  - **Errors:** Returns error if request fails or response is invalid

- `search_simple(&self, query_string: &str) -> Result<SearchResponse>`
  - Performs a search with a simple query string
  - **Parameters:** `query_string` - The search query string
  - **Returns:** `Result<SearchResponse, SerperError>`
  - **Errors:** Returns error if query is invalid or request fails

- `search_multiple(&self, queries: &[SearchQuery]) -> Result<Vec<SearchResponse>>`
  - Performs multiple searches in sequence
  - **Parameters:** `queries` - Array of search queries to execute
  - **Returns:** `Result<Vec<SearchResponse>, SerperError>`
  - **Errors:** Returns error on first failed request

- `search_concurrent(&self, queries: &[SearchQuery], max_concurrent: Option<usize>) -> Result<Vec<SearchResponse>>`
  - Performs multiple searches concurrently
  - **Parameters:** 
    - `queries` - Array of search queries to execute
    - `max_concurrent` - Maximum concurrent requests (default: 5)
  - **Returns:** `Result<Vec<SearchResponse>, SerperError>`
  - **Errors:** Returns error if any request fails

- `query_builder(&self) -> SearchQueryBuilder`
  - Creates a new query builder
  - **Returns:** `SearchQueryBuilder` instance

- `search_with<F>(&self, builder_fn: F) -> Result<SearchResponse>`
  - Searches with query builder pattern
  - **Parameters:** `builder_fn` - Function to configure the query builder
  - **Returns:** `Result<SearchResponse, SerperError>`
  - **Errors:** Returns error if query building or request fails

#### `SearchServiceBuilder`

Builder for creating search services with custom configuration.

```rust
pub struct SearchServiceBuilder { /* private fields */ }
```

**Methods:**

- `new() -> Self`
  - Creates a new search service builder
  - **Returns:** `SearchServiceBuilder` instance

- `api_key(self, api_key: impl Into<String>) -> Self`
  - Sets the API key (builder pattern)
  - **Parameters:** `api_key` - The API key
  - **Returns:** Self for method chaining

- `timeout(self, timeout: Duration) -> Self`
  - Sets the request timeout (builder pattern)
  - **Parameters:** `timeout` - Timeout duration
  - **Returns:** Self for method chaining

- `build(self) -> Result<SearchService>`
  - Builds the search service
  - **Returns:** `Result<SearchService, SerperError>`
  - **Errors:** Returns error if required configuration is missing

## Usage Examples

### Basic Search

```rust
use serper_sdk::search::{SearchService, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SearchService::new("YOUR_API_KEY".to_string())?;
    
    let query = SearchQuery::new("Rust programming".to_string())?;
    let response = service.search(&query).await?;
    
    println!("Found {} results", response.organic_count());
    for result in response.organic_results().iter().take(5) {
        println!("{}: {}", result.title, result.link);
    }
    
    Ok(())
}
```

### Advanced Query Building

```rust
use serper_sdk::search::{SearchService, SearchQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SearchService::new("YOUR_API_KEY".to_string())?;
    
    // Using SearchQueryBuilder
    let query = SearchQueryBuilder::new()
        .query("machine learning")
        .location("San Francisco")
        .country("us")
        .language("en")
        .page(1)
        .num_results(20)
        .build()?;
    
    let response = service.search(&query).await?;
    
    // Using service builder pattern
    let response2 = service.search_with(|builder| {
        builder
            .query("artificial intelligence")
            .location("New York")
            .page(2)
    }).await?;
    
    Ok(())
}
```

### Concurrent Searches

```rust
use serper_sdk::search::{SearchService, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SearchService::new("YOUR_API_KEY".to_string())?;
    
    let queries = vec![
        SearchQuery::new("Rust".to_string())?,
        SearchQuery::new("Python".to_string())?,
        SearchQuery::new("JavaScript".to_string())?,
    ];
    
    // Execute up to 3 searches concurrently
    let results = service.search_concurrent(&queries, Some(3)).await?;
    
    for (i, response) in results.iter().enumerate() {
        println!("Query {}: {} results", i + 1, response.organic_count());
    }
    
    Ok(())
}
```

### Processing Different Result Types

```rust
use serper_sdk::search::{SearchService, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SearchService::new("YOUR_API_KEY".to_string())?;
    let query = SearchQuery::new("weather today".to_string())?;
    let response = service.search(&query).await?;
    
    // Check for answer box (direct answer)
    if let Some(answer_box) = &response.answer_box {
        if let Some(answer) = &answer_box.answer {
            println!("Direct answer: {}", answer);
        }
    }
    
    // Check for knowledge graph
    if let Some(kg) = &response.knowledge_graph {
        if let Some(title) = &kg.title {
            println!("Knowledge graph: {}", title);
        }
    }
    
    // Process organic results
    for result in response.organic_results() {
        println!("Result: {} ({})", result.title, result.domain().unwrap_or("unknown"));
        if result.has_snippet() {
            println!("  {}", result.snippet_or_default());
        }
    }
    
    Ok(())
}
```

## Dependencies

### Internal Dependencies
- `core::types` - For Location, Pagination, ApiKey, BaseUrl
- `core::error` - For SerperError and Result types
- `http::client` - For HTTP client functionality
- `http::transport` - For transport configuration

### External Dependencies
- `serde` - For serialization/deserialization
- `serde_json` - For JSON handling
- `tokio` - For async runtime
- `reqwest` - For HTTP requests (via http module)
- `url` - For URL parsing in response processing

## Design Principles

1. **Builder Pattern**: Fluent interfaces for complex query construction
2. **Type Safety**: Strong typing prevents invalid queries and responses
3. **Async First**: All operations are async with concurrent support
4. **Validation**: Input validation at multiple levels
5. **Composability**: Service can be configured and extended

## Thread Safety

All search module types are designed for concurrent use:
- `SearchService` can be shared across tasks (implements Clone internally)
- `SearchQuery` is `Send + Sync` 
- `SearchResponse` types are `Send + Sync`
- Concurrent searches use internal synchronization

## Testing

The search module includes comprehensive tests covering:
- Query building and validation
- Response parsing and validation
- Service operations (with mocked HTTP)
- Error handling and edge cases

Run tests with:
```bash
cargo test search::
```