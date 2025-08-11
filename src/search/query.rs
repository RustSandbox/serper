use crate::core::{
    error::{Result, SerperError},
    types::{Location, Pagination},
};
/// Search query construction and validation module
///
/// This module provides functionality for building and validating search queries
/// with type-safe parameter handling and fluent builder patterns.
use serde::{Deserialize, Serialize};

/// Represents a search query with all possible parameters
///
/// This struct encapsulates all the parameters that can be sent to the Serper API
/// for search requests, with optional fields for flexible query construction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchQuery {
    /// The search query string (required)
    pub q: String,

    /// Optional location specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Optional country code (gl parameter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gl: Option<String>,

    /// Optional language code (hl parameter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hl: Option<String>,

    /// Optional page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optional number of results per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<u32>,
}

impl SearchQuery {
    /// Creates a new search query with the specified query string
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string
    ///
    /// # Returns
    ///
    /// A Result containing the SearchQuery or an error if validation fails
    pub fn new(query: String) -> Result<Self> {
        if query.trim().is_empty() {
            return Err(SerperError::validation_error(
                "Query string cannot be empty",
            ));
        }

        Ok(Self {
            q: query,
            location: None,
            gl: None,
            hl: None,
            page: None,
            num: None,
        })
    }

    /// Sets the location for the search query
    ///
    /// # Arguments
    ///
    /// * `location` - The location string (e.g., "Paris, France")
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the country code for the search query
    ///
    /// # Arguments
    ///
    /// * `country` - The country code (e.g., "fr", "us")
    pub fn with_country(mut self, country: String) -> Self {
        self.gl = Some(country);
        self
    }

    /// Sets the language code for the search query
    ///
    /// # Arguments
    ///
    /// * `language` - The language code (e.g., "en", "fr")
    pub fn with_language(mut self, language: String) -> Self {
        self.hl = Some(language);
        self
    }

    /// Sets the page number for pagination
    ///
    /// # Arguments
    ///
    /// * `page` - The page number (1-based)
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page
    ///
    /// # Arguments
    ///
    /// * `num` - The number of results (typically 1-100)
    pub fn with_num_results(mut self, num: u32) -> Self {
        self.num = Some(num);
        self
    }

    /// Applies location settings from a Location struct
    ///
    /// # Arguments
    ///
    /// * `location` - The location configuration
    pub fn with_location_config(mut self, location: Location) -> Self {
        if let Some(loc) = location.location {
            self.location = Some(loc);
        }
        if let Some(country) = location.country_code {
            self.gl = Some(country);
        }
        if let Some(language) = location.language_code {
            self.hl = Some(language);
        }
        self
    }

    /// Applies pagination settings from a Pagination struct
    ///
    /// # Arguments
    ///
    /// * `pagination` - The pagination configuration
    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        if let Some(page) = pagination.page {
            self.page = Some(page);
        }
        if let Some(num) = pagination.num_results {
            self.num = Some(num);
        }
        self
    }

    /// Validates the search query parameters
    ///
    /// # Returns
    ///
    /// Result indicating whether the query is valid
    pub fn validate(&self) -> Result<()> {
        if self.q.trim().is_empty() {
            return Err(SerperError::validation_error(
                "Query string cannot be empty",
            ));
        }

        if let Some(page) = self.page
            && page == 0
        {
            return Err(SerperError::validation_error(
                "Page number must be greater than 0",
            ));
        }

        if let Some(num) = self.num
            && (num == 0 || num > 100)
        {
            return Err(SerperError::validation_error(
                "Number of results must be between 1 and 100",
            ));
        }

        Ok(())
    }

    /// Gets the query string
    pub fn query(&self) -> &str {
        &self.q
    }

    /// Checks if the query has location parameters
    pub fn has_location_params(&self) -> bool {
        self.location.is_some() || self.gl.is_some() || self.hl.is_some()
    }

    /// Checks if the query has pagination parameters
    pub fn has_pagination_params(&self) -> bool {
        self.page.is_some() || self.num.is_some()
    }
}

/// Builder for creating search queries with validation
pub struct SearchQueryBuilder {
    query: Option<String>,
    location: Option<String>,
    country: Option<String>,
    language: Option<String>,
    page: Option<u32>,
    num_results: Option<u32>,
}

impl SearchQueryBuilder {
    /// Creates a new search query builder
    pub fn new() -> Self {
        Self {
            query: None,
            location: None,
            country: None,
            language: None,
            page: None,
            num_results: None,
        }
    }

    /// Sets the search query string
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Sets the location
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Sets the country code
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the language code
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sets the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results
    pub fn num_results(mut self, num: u32) -> Self {
        self.num_results = Some(num);
        self
    }

    /// Builds the search query with validation
    pub fn build(self) -> Result<SearchQuery> {
        let query = self
            .query
            .ok_or_else(|| SerperError::validation_error("Query string is required"))?;

        let mut search_query = SearchQuery::new(query)?;

        if let Some(location) = self.location {
            search_query = search_query.with_location(location);
        }
        if let Some(country) = self.country {
            search_query = search_query.with_country(country);
        }
        if let Some(language) = self.language {
            search_query = search_query.with_language(language);
        }
        if let Some(page) = self.page {
            search_query = search_query.with_page(page);
        }
        if let Some(num) = self.num_results {
            search_query = search_query.with_num_results(num);
        }

        search_query.validate()?;
        Ok(search_query)
    }
}

impl Default for SearchQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_new() {
        let query = SearchQuery::new("test query".to_string()).unwrap();
        assert_eq!(query.q, "test query");
        assert_eq!(query.location, None);
        assert_eq!(query.gl, None);
        assert_eq!(query.hl, None);
        assert_eq!(query.page, None);
        assert_eq!(query.num, None);
    }

    #[test]
    fn test_search_query_empty_fails() {
        let result = SearchQuery::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_search_query_with_location() {
        let query = SearchQuery::new("test".to_string())
            .unwrap()
            .with_location("Paris".to_string());
        assert_eq!(query.location, Some("Paris".to_string()));
    }

    #[test]
    fn test_search_query_builder() {
        let query = SearchQueryBuilder::new()
            .query("test query")
            .location("Paris")
            .country("fr")
            .language("en")
            .page(1)
            .num_results(10)
            .build()
            .unwrap();

        assert_eq!(query.q, "test query");
        assert_eq!(query.location, Some("Paris".to_string()));
        assert_eq!(query.gl, Some("fr".to_string()));
        assert_eq!(query.hl, Some("en".to_string()));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.num, Some(10));
    }

    #[test]
    fn test_search_query_validation() {
        let query = SearchQuery::new("test".to_string()).unwrap().with_page(0);

        assert!(query.validate().is_err());

        let query = SearchQuery::new("test".to_string())
            .unwrap()
            .with_num_results(101);

        assert!(query.validate().is_err());
    }

    #[test]
    fn test_search_query_helper_methods() {
        let query = SearchQuery::new("test".to_string())
            .unwrap()
            .with_location("Paris".to_string())
            .with_page(1);

        assert!(query.has_location_params());
        assert!(query.has_pagination_params());
        assert_eq!(query.query(), "test");
    }
}
