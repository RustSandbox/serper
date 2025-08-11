# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-11

### Added
- Initial release of the Serper SDK
- **Core module** with type-safe error handling and fundamental types (`ApiKey`, `BaseUrl`, `Location`, `Pagination`)
- **Search module** with comprehensive query construction and response parsing
  - `SearchQuery` with fluent builder pattern
  - `SearchResponse` with full response type support (organic results, answer boxes, knowledge graphs)
  - `SearchService` for high-level search operations
- **HTTP module** with transport layer abstraction
  - `HttpTransport` for low-level HTTP operations
  - `SerperHttpClient` for API-specific operations
  - Configurable timeouts, user agents, and headers
- **Config module** with environment variable support
  - `SdkConfig` with builder pattern
  - Environment variable integration (`SERPER_API_KEY`, `SERPER_BASE_URL`, etc.)
  - Comprehensive validation
- **Utils module** with common utilities
  - URL validation and manipulation
  - String processing and sanitization  
  - Collection utilities (HashMap merging, filtering)
  - Retry logic with exponential backoff
- **Async-first design** with concurrent request support
- **Type safety** with comprehensive error handling using `thiserror`
- **Builder patterns** for flexible configuration and query construction
- **Environment variable support** for deployment flexibility
- **Comprehensive test coverage** with 54 unit tests
- **Rich documentation** with API references and examples
- **Modular architecture** with clear separation of concerns

### Technical Features
- **Rust Edition 2024** support
- **Zero Clippy warnings** with strict linting
- **HTTP client abstraction** built on `reqwest`
- **JSON serialization** with `serde`
- **URL handling** with proper validation
- **Concurrent operations** with configurable limits
- **Request/response logging** (optional)
- **Connection pooling** and reuse
- **Proper error propagation** throughout the stack

### Examples
- Basic search example
- Environment configuration example
- Debug and testing utilities
- Comprehensive search demonstrations

### Documentation
- Complete API reference documentation
- Module-specific documentation with usage examples
- Architecture overview and design principles
- Dependency relationships and module interactions
- Contributing guidelines
- Security best practices
- **Automatic documentation generation and deployment to GitHub Pages**
- Local documentation generation with `cargo doc`

[Unreleased]: https://github.com/RustSandbox/serper/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/RustSandbox/serper/releases/tag/v0.1.0