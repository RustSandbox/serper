# Contributing to Serper SDK

We welcome contributions to the Serper SDK! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)

## Code of Conduct

This project adheres to a code of conduct adapted from the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you are expected to uphold this code.

### Our Standards

- **Be respectful** and inclusive of differing viewpoints and experiences
- **Be collaborative** and constructive in discussions
- **Focus on what is best** for the community and the project
- **Show empathy** towards other community members

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a topic branch** from `main`
4. **Make your changes**
5. **Test your changes**
6. **Submit a pull request**

## Development Setup

### Prerequisites

- **Rust 1.83+** (we use Rust Edition 2024)
- **Git**
- A **Serper API key** for testing (optional, required only for integration tests)

### Setup Instructions

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/serper.git
cd serper

# Install dependencies
cargo build

# Run tests
cargo test

# Run clippy for code quality
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check documentation
cargo doc --open
```

### Environment Variables

For testing with the real API (optional):

```bash
export SERPER_API_KEY="your-api-key-here"
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feature/add-new-endpoint` - for new features
- `fix/http-timeout-issue` - for bug fixes
- `docs/improve-readme` - for documentation
- `refactor/simplify-error-handling` - for refactoring

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:
- `feat(search): add concurrent search support`
- `fix(http): resolve timeout configuration bug`
- `docs(readme): update installation instructions`
- `refactor(core): simplify error type hierarchy`

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test core::
cargo test search::
cargo test http::

# Run with output
cargo test -- --nocapture

# Run integration tests (requires API key)
SERPER_API_KEY="your-key" cargo test --test integration
```

### Test Requirements

- **Unit tests** are required for all new functionality
- **Integration tests** should be added for API interactions
- **Documentation tests** must pass (`cargo test --doc`)
- All tests must pass on CI

### Writing Tests

- Place unit tests in the same file as the code being tested (in `#[cfg(test)]` modules)
- Use descriptive test names that explain what is being tested
- Test both success and failure cases
- Mock external dependencies where appropriate

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_validation_empty_string() {
        let result = SearchQuery::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SerperError::Validation { .. }));
    }

    #[test]
    fn test_search_query_valid_construction() {
        let query = SearchQuery::new("rust programming".to_string()).unwrap();
        assert_eq!(query.q, "rust programming");
        assert_eq!(query.location, None);
    }
}
```

## Documentation

### Documentation Requirements

- **Public APIs** must be documented with `///` comments
- **Examples** should be provided for complex functionality
- **Module-level documentation** should explain the module's purpose
- **README updates** are required for significant changes

### Documentation Style

```rust
/// Searches for the given query using the Serper API
///
/// # Arguments
///
/// * `query` - The search query to execute
///
/// # Returns
///
/// Returns a `Result` containing the search response or an error
///
/// # Examples
///
/// ```rust
/// use serper_sdk::{SearchService, SearchQuery};
///
/// # tokio_test::block_on(async {
/// let service = SearchService::new("api-key".to_string())?;
/// let query = SearchQuery::new("rust programming".to_string())?;
/// let response = service.search(&query).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
///
/// # Errors
///
/// Returns `SerperError::InvalidApiKey` if the API key is invalid
/// Returns `SerperError::Request` for network-related errors
pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
    // implementation
}
```

## Submitting Changes

### Pull Request Process

1. **Ensure tests pass**: Run `cargo test` and `cargo clippy`
2. **Update documentation**: Update relevant docs and examples
3. **Update CHANGELOG**: Add your changes to the unreleased section
4. **Create pull request**: Use the provided template
5. **Address feedback**: Respond to review comments promptly

### Pull Request Template

```markdown
## Description

Brief description of what this PR does.

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to change)
- [ ] Documentation update

## Testing

- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] I have tested this manually

## Checklist

- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have updated the documentation accordingly
- [ ] My changes generate no new warnings
- [ ] I have updated CHANGELOG.md
```

## Code Style

### Rust Code Style

We follow standard Rust conventions:

- **Use `cargo fmt`** for formatting
- **Follow Clippy suggestions** (`cargo clippy -- -D warnings`)
- **Use meaningful variable names**
- **Prefer explicit types** when it improves clarity
- **Write self-documenting code**

### Module Organization

```rust
/// Module documentation
/// 
/// Explanation of what this module does and how it fits into the overall architecture.

// Imports
use std::collections::HashMap;
use crate::core::Result;

// Constants
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

// Types
#[derive(Debug, Clone)]
pub struct MyStruct {
    // fields
}

// Implementations
impl MyStruct {
    /// Constructor
    pub fn new() -> Self { }
    
    /// Public methods
    pub fn do_something(&self) -> Result<()> { }
    
    /// Private methods
    fn internal_helper(&self) { }
}

// Trait implementations
impl Default for MyStruct { }

// Tests
#[cfg(test)]
mod tests { }
```

### Error Handling

- **Use `Result<T, SerperError>`** for fallible operations
- **Provide context** in error messages
- **Convert errors** at module boundaries
- **Document error conditions** in function docs

### Async Code

- **Use `async`/`await`** consistently
- **Avoid blocking** in async code
- **Use `tokio` primitives** for concurrency
- **Handle cancellation** appropriately

## Architecture Guidelines

### Modular Design

- **Single responsibility**: Each module should have a clear, focused purpose
- **Clean interfaces**: Modules should interact through well-defined APIs
- **Dependency direction**: Higher-level modules depend on lower-level ones
- **Loose coupling**: Minimize dependencies between modules

### Error Handling

- **Centralized errors**: Use the `core::SerperError` enum for all errors
- **Error propagation**: Use `?` operator for clean error propagation
- **Context preservation**: Include relevant context in error messages
- **Recovery strategies**: Implement retry logic where appropriate

### Performance

- **Async first**: Use async/await for I/O operations
- **Connection reuse**: Reuse HTTP connections when possible
- **Memory efficiency**: Avoid unnecessary allocations
- **Benchmark changes**: Profile performance-critical code

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Run full test suite** (`cargo test --all-features`)
4. **Check documentation** (`cargo doc --no-deps`)
5. **Create release tag** (`git tag v0.x.y`)
6. **Publish to crates.io** (`cargo publish`)
7. **Create GitHub release** with changelog

## Getting Help

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and ideas
- **Security**: Email security@remolab.fr for security-related issues

Thank you for contributing to the Serper SDK! 🚀