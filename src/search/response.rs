/// Search response parsing and handling module
/// 
/// This module provides data structures and utilities for handling search responses
/// from the Serper API, including organic results, answer boxes, and knowledge graphs.
use serde::Deserialize;
use std::collections::HashMap;

/// Complete search response from the Serper API
/// 
/// This struct represents the full response structure that can be returned
/// by the Serper search API, with all possible fields as optional.
#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// Metadata about the search request and response
    pub search_metadata: Option<SearchMetadata>,
    
    /// Organic search results
    pub organic: Option<Vec<OrganicResult>>,
    
    /// Answer box information (direct answers)
    pub answer_box: Option<AnswerBox>,
    
    /// Knowledge graph information
    pub knowledge_graph: Option<KnowledgeGraph>,
    
    /// Related questions/searches
    pub related_questions: Option<Vec<RelatedQuestion>>,
    
    /// Shopping results (if applicable)
    pub shopping: Option<Vec<ShoppingResult>>,
    
    /// News results (if applicable)
    pub news: Option<Vec<NewsResult>>,
}

impl SearchResponse {
    /// Creates a new empty search response
    pub fn new() -> Self {
        Self {
            search_metadata: None,
            organic: None,
            answer_box: None,
            knowledge_graph: None,
            related_questions: None,
            shopping: None,
            news: None,
        }
    }

    /// Checks if the response has any results
    pub fn has_results(&self) -> bool {
        self.organic.as_ref().is_some_and(|o| !o.is_empty()) ||
        self.answer_box.is_some() ||
        self.knowledge_graph.is_some() ||
        self.shopping.as_ref().is_some_and(|s| !s.is_empty()) ||
        self.news.as_ref().is_some_and(|n| !n.is_empty())
    }

    /// Gets the number of organic results
    pub fn organic_count(&self) -> usize {
        self.organic.as_ref().map_or(0, |o| o.len())
    }

    /// Gets organic results as a slice
    pub fn organic_results(&self) -> &[OrganicResult] {
        self.organic.as_deref().unwrap_or(&[])
    }

    /// Gets the first organic result if available
    pub fn first_result(&self) -> Option<&OrganicResult> {
        self.organic.as_ref()?.first()
    }

    /// Extracts all URLs from organic results
    pub fn extract_urls(&self) -> Vec<&str> {
        self.organic_results()
            .iter()
            .map(|result| result.link.as_str())
            .collect()
    }
}

impl Default for SearchResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about the search request and response
#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchMetadata {
    /// Unique identifier for this search
    pub id: String,
    
    /// Status of the search request
    pub status: String,
    
    /// Timestamp when the search was created
    pub created_at: String,
    
    /// Time taken to process the request (seconds)
    pub request_time_taken: f64,
    
    /// Total time taken including network overhead (seconds)
    pub total_time_taken: f64,
}

/// Individual organic search result
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OrganicResult {
    /// Title of the search result
    pub title: String,
    
    /// URL of the search result
    pub link: String,
    
    /// Text snippet from the page (optional)
    pub snippet: Option<String>,
    
    /// Position in search results (1-based)
    pub position: u32,
    
    /// Additional metadata (optional)
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl OrganicResult {
    /// Creates a new organic result
    pub fn new(title: String, link: String, position: u32) -> Self {
        Self {
            title,
            link,
            snippet: None,
            position,
            extra: HashMap::new(),
        }
    }

    /// Checks if the result has a snippet
    pub fn has_snippet(&self) -> bool {
        self.snippet.is_some()
    }

    /// Gets the snippet text or a default message
    pub fn snippet_or_default(&self) -> &str {
        self.snippet.as_deref().unwrap_or("No description available")
    }

    /// Gets the domain from the URL
    pub fn domain(&self) -> Option<String> {
        url::Url::parse(&self.link)
            .ok()?
            .host_str()
            .map(|host| host.to_string())
    }
}

/// Answer box with direct answers to queries
#[derive(Debug, Deserialize, PartialEq)]
pub struct AnswerBox {
    /// Direct answer text (optional)
    pub answer: Option<String>,
    
    /// Snippet providing context for the answer (optional)
    pub snippet: Option<String>,
    
    /// Source title (optional)
    pub title: Option<String>,
    
    /// Source link (optional)
    pub link: Option<String>,
}

impl AnswerBox {
    /// Checks if the answer box has a direct answer
    pub fn has_answer(&self) -> bool {
        self.answer.is_some()
    }

    /// Gets the best available text (answer or snippet)
    pub fn best_text(&self) -> Option<&str> {
        self.answer.as_deref().or(self.snippet.as_deref())
    }
}

/// Knowledge graph information
#[derive(Debug, Deserialize, PartialEq)]
pub struct KnowledgeGraph {
    /// Title of the entity
    pub title: Option<String>,
    
    /// Description of the entity
    pub description: Option<String>,
    
    /// Entity type (person, organization, etc.)
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    
    /// Website URL (optional)
    pub website: Option<String>,
    
    /// Additional attributes
    #[serde(flatten)]
    pub attributes: HashMap<String, serde_json::Value>,
}

/// Related question from "People also ask"
#[derive(Debug, Deserialize, PartialEq)]
pub struct RelatedQuestion {
    /// The question text
    pub question: String,
    
    /// Snippet answering the question (optional)
    pub snippet: Option<String>,
    
    /// Source title (optional)
    pub title: Option<String>,
    
    /// Source link (optional)
    pub link: Option<String>,
}

/// Shopping result for product searches
#[derive(Debug, Deserialize, PartialEq)]
pub struct ShoppingResult {
    /// Product title
    pub title: String,
    
    /// Product link
    pub link: String,
    
    /// Product price (optional)
    pub price: Option<String>,
    
    /// Product source/merchant (optional)
    pub source: Option<String>,
    
    /// Product image URL (optional)
    pub image: Option<String>,
    
    /// Position in shopping results
    pub position: u32,
}

/// News result for news searches
#[derive(Debug, Deserialize, PartialEq)]
pub struct NewsResult {
    /// News article title
    pub title: String,
    
    /// News article link
    pub link: String,
    
    /// Article snippet (optional)
    pub snippet: Option<String>,
    
    /// News source (optional)
    pub source: Option<String>,
    
    /// Publication date (optional)
    pub date: Option<String>,
    
    /// Position in news results
    pub position: u32,
}

/// Response parser for handling different response formats
pub struct ResponseParser;

impl ResponseParser {
    /// Parses a JSON response into a SearchResponse
    /// 
    /// # Arguments
    /// 
    /// * `json_str` - The JSON response string
    /// 
    /// # Returns
    /// 
    /// Result containing the parsed SearchResponse or an error
    pub fn parse_response(json_str: &str) -> crate::core::Result<SearchResponse> {
        serde_json::from_str(json_str)
            .map_err(crate::core::error::SerperError::Json)
    }

    /// Validates that a response has the expected structure
    pub fn validate_response(response: &SearchResponse) -> crate::core::Result<()> {
        // Basic validation - could be extended with more checks
        if let Some(metadata) = &response.search_metadata
            && metadata.id.is_empty() {
                return Err(crate::core::error::SerperError::validation_error(
                    "Response metadata has empty ID"
                ));
            }

        // Validate organic results
        if let Some(organic) = &response.organic {
            for (idx, result) in organic.iter().enumerate() {
                if result.title.is_empty() {
                    return Err(crate::core::error::SerperError::validation_error(
                        format!("Organic result {} has empty title", idx)
                    ));
                }
                if result.link.is_empty() {
                    return Err(crate::core::error::SerperError::validation_error(
                        format!("Organic result {} has empty link", idx)
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_search_response_creation() {
        let response = SearchResponse::new();
        assert!(!response.has_results());
        assert_eq!(response.organic_count(), 0);
    }

    #[test]
    fn test_organic_result() {
        let result = OrganicResult::new(
            "Test Title".to_string(),
            "https://example.com".to_string(),
            1
        );
        
        assert_eq!(result.title, "Test Title");
        assert_eq!(result.position, 1);
        assert!(!result.has_snippet());
        assert_eq!(result.snippet_or_default(), "No description available");
    }

    #[test]
    fn test_response_parsing() {
        let json_data = json!({
            "search_metadata": {
                "id": "test-123",
                "status": "Success",
                "created_at": "2023-01-01T00:00:00Z",
                "request_time_taken": 0.5,
                "total_time_taken": 1.0
            },
            "organic": [
                {
                    "title": "Test Result",
                    "link": "https://example.com",
                    "snippet": "Test snippet",
                    "position": 1
                }
            ]
        });

        let response: SearchResponse = serde_json::from_value(json_data).unwrap();
        assert!(response.has_results());
        assert_eq!(response.organic_count(), 1);
        
        let first = response.first_result().unwrap();
        assert_eq!(first.title, "Test Result");
    }

    #[test]
    fn test_answer_box() {
        let answer_box = AnswerBox {
            answer: Some("42".to_string()),
            snippet: Some("The answer to everything".to_string()),
            title: None,
            link: None,
        };

        assert!(answer_box.has_answer());
        assert_eq!(answer_box.best_text(), Some("42"));
    }

    #[test]
    fn test_response_validation() {
        let mut response = SearchResponse::new();
        
        // Valid response should pass
        assert!(ResponseParser::validate_response(&response).is_ok());
        
        // Response with empty organic result title should fail
        response.organic = Some(vec![
            OrganicResult {
                title: "".to_string(),
                link: "https://example.com".to_string(),
                snippet: None,
                position: 1,
                extra: HashMap::new(),
            }
        ]);
        
        assert!(ResponseParser::validate_response(&response).is_err());
    }
}