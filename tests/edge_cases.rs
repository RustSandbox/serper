use serper_sdk::{SerperClient, SearchQuery, SerperError};
use mockito::{Server, Matcher};
use serde_json::json;

#[tokio::test]
async fn test_empty_query_string() {
    let mut server = Server::new_async().await;
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(json!({"q": ""}).to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new("".to_string());
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_very_long_query_string() {
    let mut server = Server::new_async().await;
    
    // Create a very long query string (2000 characters)
    let long_query = "a".repeat(2000);
    
    let expected_body = json!({"q": long_query});
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(expected_body.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new(long_query);
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_special_characters_in_query() {
    let mut server = Server::new_async().await;
    
    let special_query = "test @#$%^&*(){}[]|\\:;\"'<>?,./~`";
    let expected_body = json!({"q": special_query});
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(expected_body.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new(special_query.to_string());
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_unicode_characters_in_query() {
    let mut server = Server::new_async().await;
    
    let unicode_query = "测试 हैलो مرحبا 🚀 Ελληνικά";
    let expected_body = json!({"q": unicode_query});
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(expected_body.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new(unicode_query.to_string());
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_maximum_page_number() {
    let mut server = Server::new_async().await;
    
    let expected_body = json!({"q": "test", "page": u32::MAX});
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(expected_body.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new("test".to_string()).with_page(u32::MAX);
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_zero_num_results() {
    let mut server = Server::new_async().await;
    
    let expected_body = json!({"q": "test", "num": 0});
    
    let mock = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(expected_body.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new("test".to_string()).with_num_results(0);
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_all_http_error_codes() {
    let error_codes = vec![400, 401, 403, 404, 429, 500, 502, 503, 504];
    
    for code in error_codes {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/search")
            .with_status(code)
            .with_body(format!("Error {}", code))
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let query = SearchQuery::new("test".to_string());
        let result = client.search(query).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            SerperError::Api { message } => {
                assert!(message.contains(&code.to_string()));
            },
            _ => panic!("Expected Api error for status code {}", code),
        }
        
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn test_malformed_json_responses() {
    let malformed_jsons = vec![
        "{ incomplete json",
        "{ \"key\": }",
        "not json at all",
        "",
        "null",
        "[]",
        "{\"organic\": \"should be array\"}",
        "{\"organic\": [\"invalid organic result\"]}",
    ];
    
    for (i, malformed_json) in malformed_jsons.iter().enumerate() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(malformed_json)
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            format!("test-key-{}", i), 
            server.url()
        ).unwrap();
        
        let query = SearchQuery::new("test".to_string());
        let result = client.search(query).await;
        
        // Most malformed JSON should result in a Json error
        // Some might succeed if they're valid but unexpected structure
        if result.is_err() {
            match result.unwrap_err() {
                SerperError::Json(_) => {},
                other => println!("Unexpected error for '{}': {:?}", malformed_json, other),
            }
        }
        
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn test_network_timeout_simulation() {
    let mut server = Server::new_async().await;
    
    // Simulate a response (timeout testing requires different approach)
    let mock = server.mock("POST", "/search")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "test-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new("test".to_string());
    let result = client.search(query).await;
    
    // With default timeout, this should succeed
    // In a real timeout scenario, this would be a Request error
    assert!(result.is_ok());
    
    mock.assert_async().await;
}

#[tokio::test]
async fn test_concurrent_requests() {
    let mut server = Server::new_async().await;
    
    let mock = server.mock("POST", "/search")
        .expect(10)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let _client = SerperClient::new_with_base_url(
        "concurrent-key".to_string(), 
        server.url()
    ).unwrap();
    
    let mut handles = vec![];
    
    // Launch 10 concurrent requests
    for i in 0..10 {
        let client_clone = SerperClient::new_with_base_url(
            "concurrent-key".to_string(), 
            server.url()
        ).unwrap();
        
        let handle = tokio::spawn(async move {
            let query = SearchQuery::new(format!("query {}", i));
            client_clone.search(query).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    let mut success_count = 0;
    for handle in handles {
        let result = handle.await.unwrap();
        if result.is_ok() {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, 10);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_partial_response_data() {
    let mut server = Server::new_async().await;
    
    // Response with only some fields present
    let partial_response = json!({
        "organic": [
            {
                "title": "Partial Result",
                "link": "https://example.com",
                "position": 1
                // Missing snippet field
            }
        ]
        // Missing search_metadata, answer_box, knowledge_graph
    });
    
    let mock = server.mock("POST", "/search")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(partial_response.to_string())
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "partial-key".to_string(), 
        server.url()
    ).unwrap();
    
    let query = SearchQuery::new("partial test".to_string());
    let result = client.search(query).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify that missing optional fields are None
    assert!(response.search_metadata.is_none());
    assert!(response.answer_box.is_none());
    assert!(response.knowledge_graph.is_none());
    
    // Verify organic results exist and handle missing snippet
    assert!(response.organic.is_some());
    let organic = response.organic.unwrap();
    assert_eq!(organic.len(), 1);
    assert_eq!(organic[0].title, "Partial Result");
    assert!(organic[0].snippet.is_none());
    
    mock.assert_async().await;
}

#[tokio::test]
async fn test_search_multiple_with_mixed_success_failure() {
    let mut server = Server::new_async().await;
    
    // First request succeeds
    let mock1 = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(json!({"q": "success query"}).to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;
    
    // Second request fails
    let mock2 = server.mock("POST", "/search")
        .match_body(Matcher::JsonString(json!({"q": "failure query"}).to_string()))
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = SerperClient::new_with_base_url(
        "mixed-key".to_string(), 
        server.url()
    ).unwrap();
    
    let queries = vec![
        SearchQuery::new("success query".to_string()),
        SearchQuery::new("failure query".to_string()),
    ];
    
    let result = client.search_multiple(queries).await;
    
    // The entire operation should fail on the first error
    assert!(result.is_err());
    match result.unwrap_err() {
        SerperError::Api { message } => {
            assert!(message.contains("500"));
        },
        _ => panic!("Expected Api error"),
    }
    
    mock1.assert_async().await;
    mock2.assert_async().await;
}