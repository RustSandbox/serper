/// Utility functions and helpers
///
/// This module provides common utility functions used throughout the SDK,
/// including validation helpers, formatting utilities, and convenience functions.
use crate::core::{Result, SerperError};
use std::collections::HashMap;

/// URL validation utilities  
pub mod url {
    use super::*;
    use ::url::Url;

    /// Validates that a URL is properly formatted
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to validate
    ///
    /// # Returns
    ///
    /// Result indicating whether the URL is valid
    pub fn validate_url(url: &str) -> Result<()> {
        if url.trim().is_empty() {
            return Err(SerperError::validation_error("URL cannot be empty"));
        }

        Url::parse(url)
            .map_err(|_| SerperError::validation_error(format!("Invalid URL: {}", url)))?;

        Ok(())
    }

    /// Validates that a URL uses HTTPS
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to validate
    ///
    /// # Returns
    ///
    /// Result indicating whether the URL uses HTTPS
    pub fn validate_https(url: &str) -> Result<()> {
        validate_url(url)?;

        if !url.starts_with("https://") {
            return Err(SerperError::validation_error("URL must use HTTPS"));
        }

        Ok(())
    }

    /// Extracts the domain from a URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string
    ///
    /// # Returns
    ///
    /// Result containing the domain or an error
    pub fn extract_domain(url: &str) -> Result<String> {
        let parsed = Url::parse(url)
            .map_err(|_| SerperError::validation_error(format!("Invalid URL: {}", url)))?;

        parsed
            .host_str()
            .map(|host| host.to_string())
            .ok_or_else(|| SerperError::validation_error("URL has no domain"))
    }
}

/// String validation and formatting utilities
pub mod string {
    use super::*;

    /// Validates that a string is not empty after trimming
    ///
    /// # Arguments
    ///
    /// * `value` - The string to validate
    /// * `field_name` - Name of the field for error messages
    ///
    /// # Returns
    ///
    /// Result indicating whether the string is valid
    pub fn validate_non_empty(value: &str, field_name: &str) -> Result<()> {
        if value.trim().is_empty() {
            return Err(SerperError::validation_error(format!(
                "{} cannot be empty",
                field_name
            )));
        }
        Ok(())
    }

    /// Validates string length constraints
    ///
    /// # Arguments
    ///
    /// * `value` - The string to validate
    /// * `min_len` - Minimum length (optional)
    /// * `max_len` - Maximum length (optional)
    /// * `field_name` - Name of the field for error messages
    ///
    /// # Returns
    ///
    /// Result indicating whether the string length is valid
    pub fn validate_length(
        value: &str,
        min_len: Option<usize>,
        max_len: Option<usize>,
        field_name: &str,
    ) -> Result<()> {
        let len = value.len();

        if let Some(min) = min_len
            && len < min
        {
            return Err(SerperError::validation_error(format!(
                "{} must be at least {} characters",
                field_name, min
            )));
        }

        if let Some(max) = max_len
            && len > max
        {
            return Err(SerperError::validation_error(format!(
                "{} must be at most {} characters",
                field_name, max
            )));
        }

        Ok(())
    }

    /// Sanitizes a string by removing control characters
    ///
    /// # Arguments
    ///
    /// * `value` - The string to sanitize
    ///
    /// # Returns
    ///
    /// A sanitized string
    pub fn sanitize(value: &str) -> String {
        value
            .chars()
            .filter(|c| !c.is_control() || c.is_whitespace())
            .collect()
    }

    /// Truncates a string to a maximum length with ellipsis
    ///
    /// # Arguments
    ///
    /// * `value` - The string to truncate
    /// * `max_len` - Maximum length
    ///
    /// # Returns
    ///
    /// A truncated string
    pub fn truncate(value: &str, max_len: usize) -> String {
        if value.len() <= max_len {
            value.to_string()
        } else if max_len <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &value[..max_len - 3])
        }
    }
}

/// Collection utilities
pub mod collections {
    use super::*;

    /// Merges two HashMaps, with values from the second map taking precedence
    ///
    /// # Arguments
    ///
    /// * `base` - The base HashMap
    /// * `overlay` - The overlay HashMap
    ///
    /// # Returns
    ///
    /// A merged HashMap
    pub fn merge_hashmaps<K, V>(base: HashMap<K, V>, overlay: HashMap<K, V>) -> HashMap<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        let mut result = base;
        result.extend(overlay);
        result
    }

    /// Filters a HashMap by keys matching a predicate
    ///
    /// # Arguments
    ///
    /// * `map` - The HashMap to filter
    /// * `predicate` - Function to test each key
    ///
    /// # Returns
    ///
    /// A filtered HashMap
    pub fn filter_map_by_key<K, V, F>(map: HashMap<K, V>, predicate: F) -> HashMap<K, V>
    where
        K: std::hash::Hash + Eq,
        F: Fn(&K) -> bool,
    {
        map.into_iter().filter(|(k, _)| predicate(k)).collect()
    }
}

/// Retry utilities for handling transient failures
pub mod retry {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Retry configuration
    #[derive(Debug, Clone)]
    pub struct RetryConfig {
        /// Maximum number of retry attempts
        pub max_attempts: usize,
        /// Initial delay between retries
        pub initial_delay: Duration,
        /// Multiplier for exponential backoff
        pub backoff_multiplier: f64,
        /// Maximum delay between retries
        pub max_delay: Duration,
    }

    impl RetryConfig {
        /// Creates a new retry configuration with default values
        pub fn new() -> Self {
            Self {
                max_attempts: 3,
                initial_delay: Duration::from_millis(100),
                backoff_multiplier: 2.0,
                max_delay: Duration::from_secs(10),
            }
        }

        /// Sets the maximum number of attempts
        pub fn with_max_attempts(mut self, attempts: usize) -> Self {
            self.max_attempts = attempts;
            self
        }

        /// Sets the initial delay
        pub fn with_initial_delay(mut self, delay: Duration) -> Self {
            self.initial_delay = delay;
            self
        }
    }

    impl Default for RetryConfig {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Executes a function with retry logic
    ///
    /// # Arguments
    ///
    /// * `config` - Retry configuration
    /// * `operation` - Async function to retry
    ///
    /// # Returns
    ///
    /// Result containing the operation result or final error
    pub async fn with_retry<F, Fut, T, E>(config: RetryConfig, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = std::result::Result<T, E>>,
        E: Into<SerperError>,
    {
        let mut last_error = None;
        let mut delay = config.initial_delay;

        for attempt in 0..config.max_attempts {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.into());

                    if attempt + 1 < config.max_attempts {
                        sleep(delay).await;
                        delay = std::cmp::min(
                            Duration::from_millis(
                                (delay.as_millis() as f64 * config.backoff_multiplier) as u64,
                            ),
                            config.max_delay,
                        );
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| SerperError::config_error("Unknown retry error")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod url_tests {
        use super::*;

        #[test]
        fn test_validate_url() {
            assert!(url::validate_url("https://example.com").is_ok());
            assert!(url::validate_url("http://example.com").is_ok());
            assert!(url::validate_url("").is_err());
            assert!(url::validate_url("not-a-url").is_err());
        }

        #[test]
        fn test_validate_https() {
            assert!(url::validate_https("https://example.com").is_ok());
            assert!(url::validate_https("http://example.com").is_err());
            assert!(url::validate_https("").is_err());
        }

        #[test]
        fn test_extract_domain() {
            assert_eq!(
                url::extract_domain("https://example.com/path").unwrap(),
                "example.com"
            );
            assert_eq!(
                url::extract_domain("http://sub.example.com").unwrap(),
                "sub.example.com"
            );
            assert!(url::extract_domain("not-a-url").is_err());
        }
    }

    mod string_tests {
        use super::*;

        #[test]
        fn test_validate_non_empty() {
            assert!(string::validate_non_empty("test", "field").is_ok());
            assert!(string::validate_non_empty("", "field").is_err());
            assert!(string::validate_non_empty("   ", "field").is_err());
        }

        #[test]
        fn test_validate_length() {
            assert!(string::validate_length("test", Some(3), Some(5), "field").is_ok());
            assert!(string::validate_length("te", Some(3), Some(5), "field").is_err());
            assert!(string::validate_length("toolong", Some(3), Some(5), "field").is_err());
        }

        #[test]
        fn test_sanitize() {
            assert_eq!(string::sanitize("test\x00string"), "teststring");
            assert_eq!(string::sanitize("test\nstring"), "test\nstring");
        }

        #[test]
        fn test_truncate() {
            assert_eq!(string::truncate("short", 10), "short");
            assert_eq!(string::truncate("toolongstring", 8), "toolo...");
            assert_eq!(string::truncate("test", 3), "...");
        }
    }

    mod collections_tests {
        use super::*;

        #[test]
        fn test_merge_hashmaps() {
            let mut base = HashMap::new();
            base.insert("a", 1);
            base.insert("b", 2);

            let mut overlay = HashMap::new();
            overlay.insert("b", 3);
            overlay.insert("c", 4);

            let result = collections::merge_hashmaps(base, overlay);
            assert_eq!(result.get("a"), Some(&1));
            assert_eq!(result.get("b"), Some(&3)); // Overlay wins
            assert_eq!(result.get("c"), Some(&4));
        }
    }

    mod retry_tests {
        use crate::utils::retry::RetryConfig;
        use std::time::Duration;

        #[test]
        fn test_retry_config() {
            let config = RetryConfig::new()
                .with_max_attempts(5)
                .with_initial_delay(Duration::from_millis(50));

            assert_eq!(config.max_attempts, 5);
            assert_eq!(config.initial_delay, Duration::from_millis(50));
        }
    }
}
