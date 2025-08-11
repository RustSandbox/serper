// Common test utilities and helpers

use serper_sdk::{SearchQuery, SearchResponse, OrganicResult};
use serde_json::json;

pub fn create_test_query() -> SearchQuery {
    SearchQuery::new("test query".to_string())
}

pub fn create_test_query_with_all_params() -> SearchQuery {
    SearchQuery::new("comprehensive test".to_string())
        .with_location("Paris".to_string())
        .with_country("fr".to_string())
        .with_language("en".to_string())
        .with_page(1)
        .with_num_results(10)
}

pub fn create_mock_search_response() -> serde_json::Value {
    json!({
        "search_metadata": {
            "id": "test-123",
            "status": "Success",
            "created_at": "2023-01-01T00:00:00Z",
            "request_time_taken": 0.5,
            "total_time_taken": 1.0
        },
        "organic": [
            {
                "title": "Test Result 1",
                "link": "https://example1.com",
                "snippet": "First test result",
                "position": 1
            },
            {
                "title": "Test Result 2",
                "link": "https://example2.com",
                "snippet": "Second test result",
                "position": 2
            }
        ],
        "answer_box": {
            "answer": "Test answer",
            "snippet": "Test answer snippet"
        },
        "knowledge_graph": {
            "title": "Test Knowledge",
            "description": "Test knowledge description"
        }
    })
}

pub fn create_minimal_search_response() -> serde_json::Value {
    json!({
        "organic": [
            {
                "title": "Minimal Result",
                "link": "https://minimal.com",
                "position": 1
            }
        ]
    })
}

pub fn create_empty_search_response() -> serde_json::Value {
    json!({})
}

pub fn assert_search_response_valid(response: &SearchResponse) {
    // Basic validation that a search response has reasonable structure
    if let Some(ref organic) = response.organic {
        for result in organic {
            assert!(!result.title.is_empty());
            assert!(!result.link.is_empty());
            assert!(result.position > 0);
        }
    }
    
    if let Some(ref metadata) = response.search_metadata {
        assert!(!metadata.id.is_empty());
        assert!(!metadata.status.is_empty());
        assert!(metadata.request_time_taken >= 0.0);
        assert!(metadata.total_time_taken >= 0.0);
    }
}

pub fn create_test_organic_result(position: u32) -> OrganicResult {
    OrganicResult {
        title: format!("Test Title {}", position),
        link: format!("https://example{}.com", position),
        snippet: Some(format!("Test snippet {}", position)),
        position,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_query() {
        let query = create_test_query();
        assert_eq!(query.q, "test query");
        assert_eq!(query.location, None);
    }

    #[test]
    fn test_create_test_query_with_all_params() {
        let query = create_test_query_with_all_params();
        assert_eq!(query.q, "comprehensive test");
        assert_eq!(query.location, Some("Paris".to_string()));
        assert_eq!(query.gl, Some("fr".to_string()));
        assert_eq!(query.hl, Some("en".to_string()));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.num, Some(10));
    }

    #[test]
    fn test_mock_response_deserializable() {
        let mock_json = create_mock_search_response();
        let response: SearchResponse = serde_json::from_value(mock_json).unwrap();
        
        assert!(response.search_metadata.is_some());
        assert!(response.organic.is_some());
        assert!(response.answer_box.is_some());
        assert!(response.knowledge_graph.is_some());
        
        assert_search_response_valid(&response);
    }

    #[test]
    fn test_minimal_response_deserializable() {
        let minimal_json = create_minimal_search_response();
        let response: SearchResponse = serde_json::from_value(minimal_json).unwrap();
        
        assert!(response.search_metadata.is_none());
        assert!(response.organic.is_some());
        assert!(response.answer_box.is_none());
        assert!(response.knowledge_graph.is_none());
        
        assert_search_response_valid(&response);
    }

    #[test]
    fn test_empty_response_deserializable() {
        let empty_json = create_empty_search_response();
        let response: SearchResponse = serde_json::from_value(empty_json).unwrap();
        
        assert!(response.search_metadata.is_none());
        assert!(response.organic.is_none());
        assert!(response.answer_box.is_none());
        assert!(response.knowledge_graph.is_none());
    }

    #[test]
    fn test_create_test_organic_result() {
        let result = create_test_organic_result(5);
        assert_eq!(result.title, "Test Title 5");
        assert_eq!(result.link, "https://example5.com");
        assert_eq!(result.snippet, Some("Test snippet 5".to_string()));
        assert_eq!(result.position, 5);
    }
}