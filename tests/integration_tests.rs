mod common;

use serper_sdk::{SearchService, SearchQuery, SerperError};
use mockito::{Server, Matcher};
use serde_json::json;
use common::{create_mock_search_response, assert_search_response_valid, create_test_query_with_all_params, create_test_service_with_base_url};

#[tokio::test]
async fn test_end_to_end_search_flow() {
    let mut server = Server::new_async().await;
    
    let mock_response = create_mock_search_response();

    let mock = server.mock("POST", "/search")
        .match_header("X-API-KEY", "test-api-key")
        .match_header("Content-Type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let client = create_test_service_with_base_url(
        "test-api-key".to_string(), 
        server.url()
    );
    
    let query = create_test_query_with_all_params();
    
    let result = client.search(&query).await.unwrap();
    
    // Use our test helper to validate the response structure
    assert_search_response_valid(&result);
    
    // Verify specific fields from our mock response
    assert!(result.search_metadata.is_some());
    let metadata = result.search_metadata.unwrap();
    assert_eq!(metadata.id, "test-123");
    assert_eq!(metadata.status, "Success");
    
    // Verify organic results
    assert!(result.organic.is_some());
    let organic = result.organic.unwrap();
    assert_eq!(organic.len(), 2);
    assert_eq!(organic[0].title, "Test Result 1");
    assert_eq!(organic[0].position, 1);
    
    // Verify answer box and knowledge graph exist
    assert!(result.answer_box.is_some());
    assert!(result.knowledge_graph.is_some());
    
    mock.assert_async().await;
}

#[tokio::test]
async fn test_multiple_search_integration() {
    let mut server = Server::new_async().await;
    
    let response1 = json!({
        "organic": [{"title": "Rust Result 1", "link": "https://rust1.com", "position": 1}]
    });
    
    let response2 = json!({
        "organic": [{"title": "Go Result 1", "link": "https://golang1.com", "position": 1}]
    });
    
    let response3 = json!({
        "organic": [{"title": "Python Result 1", "link": "https://python1.com", "position": 1}]
    });

    let mock1 = server.mock("POST", "/search")
        .match_header("X-API-KEY", "batch-key")
        .match_body(Matcher::JsonString(json!({"q": "Rust programming", "location": "France"}).to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response1.to_string())
        .create_async()
        .await;

    let mock2 = server.mock("POST", "/search")
        .match_header("X-API-KEY", "batch-key")
        .match_body(Matcher::JsonString(json!({"q": "Go programming", "location": "France"}).to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response2.to_string())
        .create_async()
        .await;

    let mock3 = server.mock("POST", "/search")
        .match_header("X-API-KEY", "batch-key")
        .match_body(Matcher::JsonString(json!({"q": "Python programming", "location": "France"}).to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response3.to_string())
        .create_async()
        .await;

    let client = create_test_service_with_base_url(
        "batch-key".to_string(), 
        server.url()
    );
    
    let queries = vec![
        SearchQuery::new("Rust programming".to_string()).unwrap().with_location("France".to_string()),
        SearchQuery::new("Go programming".to_string()).unwrap().with_location("France".to_string()),
        SearchQuery::new("Python programming".to_string()).unwrap().with_location("France".to_string()),
    ];
    
    let results = client.search_multiple(&queries).await.unwrap();
    
    assert_eq!(results.len(), 3);
    
    // Verify first result
    assert!(results[0].organic.is_some());
    let organic1 = &results[0].organic.as_ref().unwrap()[0];
    assert_eq!(organic1.title, "Rust Result 1");
    
    // Verify second result
    assert!(results[1].organic.is_some());
    let organic2 = &results[1].organic.as_ref().unwrap()[0];
    assert_eq!(organic2.title, "Go Result 1");
    
    // Verify third result
    assert!(results[2].organic.is_some());
    let organic3 = &results[2].organic.as_ref().unwrap()[0];
    assert_eq!(organic3.title, "Python Result 1");
    
    mock1.assert_async().await;
    mock2.assert_async().await;
    mock3.assert_async().await;
}

#[tokio::test]
async fn test_error_propagation_through_modules() {
    // Test invalid API key error propagation
    let client_result = SearchService::new("".to_string());
    assert!(client_result.is_err());
    match client_result.unwrap_err() {
        SerperError::InvalidApiKey => {},
        _ => panic!("Expected InvalidApiKey error"),
    }
    
    // Test API error propagation
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/search")
        .with_status(403)
        .with_body("Forbidden")
        .create_async()
        .await;

    let client = create_test_service_with_base_url(
        "invalid-key".to_string(), 
        server.url()
    );
    
    let query = SearchQuery::new("test".to_string()).unwrap();
    let result = client.search(&query).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        SerperError::Api { message } => {
            assert!(message.contains("403"));
        },
        _ => panic!("Expected Api error"),
    }
    
    mock.assert_async().await;
}

#[tokio::test] 
async fn test_serialization_deserialization_integration() {
    // Test that our models serialize and deserialize correctly with real API-like data
    let query = SearchQuery::new("integration test".to_string()).unwrap()
        .with_location("New York".to_string())
        .with_country("us".to_string())
        .with_language("en".to_string())
        .with_page(1)
        .with_num_results(10);
    
    // Serialize the query
    let serialized = serde_json::to_string(&query).unwrap();
    
    // Verify serialization contains expected fields
    assert!(serialized.contains("\"q\":\"integration test\""));
    assert!(serialized.contains("\"location\":\"New York\""));
    assert!(serialized.contains("\"gl\":\"us\""));
    assert!(serialized.contains("\"hl\":\"en\""));
    assert!(serialized.contains("\"page\":1"));
    assert!(serialized.contains("\"num\":10"));
    
    // Test deserialization of a realistic API response
    let api_response = json!({
        "search_metadata": {
            "id": "search-123",
            "status": "Success",
            "created_at": "2023-12-01T10:30:00Z",
            "request_time_taken": 1.2,
            "total_time_taken": 1.8
        },
        "organic": [
            {
                "title": "Integration Testing Best Practices",
                "link": "https://example.com/integration-testing",
                "snippet": "Learn how to write effective integration tests",
                "position": 1
            },
            {
                "title": "Integration Testing Guide",
                "link": "https://guide.example.com/integration",
                "position": 2
            }
        ],
        "answer_box": {
            "answer": "Integration testing verifies that different modules work together correctly."
        },
        "knowledge_graph": {
            "title": "Integration Testing",
            "description": "Software testing technique"
        }
    });
    
    let deserialized: serper_sdk::SearchResponse = 
        serde_json::from_value(api_response).unwrap();
    
    // Verify all components deserialized correctly
    assert!(deserialized.search_metadata.is_some());
    assert!(deserialized.organic.is_some());
    assert!(deserialized.answer_box.is_some());
    assert!(deserialized.knowledge_graph.is_some());
    
    let organic = deserialized.organic.unwrap();
    assert_eq!(organic.len(), 2);
    assert_eq!(organic[1].snippet, None); // Test optional field handling
}

#[test]
fn test_module_public_api_completeness() {
    // Verify that all necessary types are publicly accessible
    let _query = SearchQuery::new("test".to_string()).unwrap();
    let _client_result = SearchService::new("key".to_string());
    let _error = SerperError::InvalidApiKey;
    
    // This test ensures our public API exports are complete and accessible
    // If any required type is not public, this won't compile
}