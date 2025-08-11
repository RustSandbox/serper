use crate::{error::Result, models::*};
use reqwest::Client as HttpClient;

const BASE_URL: &str = "https://google.serper.dev";

#[derive(Debug)]
pub struct SerperClient {
    client: HttpClient,
    api_key: String,
    base_url: String,
}

impl SerperClient {
    pub fn new(api_key: String) -> Result<Self> {
        if api_key.is_empty() {
            return Err(crate::error::SerperError::InvalidApiKey);
        }
        
        Ok(Self {
            client: HttpClient::new(),
            api_key,
            base_url: BASE_URL.to_string(),
        })
    }
    
    pub fn new_with_base_url(api_key: String, base_url: String) -> Result<Self> {
        if api_key.is_empty() {
            return Err(crate::error::SerperError::InvalidApiKey);
        }
        
        Ok(Self {
            client: HttpClient::new(),
            api_key,
            base_url,
        })
    }
    
    pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse> {
        let url = format!("{}/search", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("X-API-KEY", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&query)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(crate::error::SerperError::Api {
                message: format!("API returned status: {}", response.status()),
            });
        }
        
        let search_response: SearchResponse = response.json().await?;
        Ok(search_response)
    }
    
    pub async fn search_multiple(&self, queries: Vec<SearchQuery>) -> Result<Vec<SearchResponse>> {
        let mut results = Vec::new();
        
        for query in queries {
            let result = self.search(query).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SerperError;
    use mockito::{Matcher, Server};
    use serde_json::json;

    #[test]
    fn test_client_new_with_valid_api_key() {
        let client = SerperClient::new("valid-api-key".to_string());
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.api_key, "valid-api-key");
        assert_eq!(client.base_url, BASE_URL);
    }

    #[test]
    fn test_client_new_with_empty_api_key() {
        let client = SerperClient::new("".to_string());
        assert!(client.is_err());
        match client.unwrap_err() {
            SerperError::InvalidApiKey => {},
            _ => panic!("Expected InvalidApiKey error"),
        }
    }

    #[test]
    fn test_client_new_with_base_url() {
        let client = SerperClient::new_with_base_url(
            "api-key".to_string(), 
            "https://custom.api.com".to_string()
        );
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_client_new_with_base_url_empty_api_key() {
        let client = SerperClient::new_with_base_url(
            "".to_string(), 
            "https://custom.api.com".to_string()
        );
        assert!(client.is_err());
    }

    #[tokio::test]
    async fn test_search_successful_response() {
        let mut server = Server::new_async().await;
        
        let mock_response = json!({
            "organic": [
                {
                    "title": "Test Result",
                    "link": "https://example.com",
                    "snippet": "Test snippet",
                    "position": 1
                }
            ]
        });

        let mock = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .match_header("Content-Type", "application/json")
            .match_body(Matcher::JsonString(json!({
                "q": "test query"
            }).to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let query = SearchQuery::new("test query".to_string());
        let result = client.search(query).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.organic.is_some());
        let organic = response.organic.unwrap();
        assert_eq!(organic.len(), 1);
        assert_eq!(organic[0].title, "Test Result");
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_api_error_response() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .with_status(401)
            .with_body("Unauthorized")
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let query = SearchQuery::new("test query".to_string());
        let result = client.search(query).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            SerperError::Api { message } => {
                assert!(message.contains("401"));
            },
            _ => panic!("Expected Api error"),
        }
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_invalid_json_response() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{invalid json")
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let query = SearchQuery::new("test query".to_string());
        let result = client.search(query).await;
        
        assert!(result.is_err());
        // The error could be either Json or Request depending on how reqwest handles it
        match result.unwrap_err() {
            SerperError::Json(_) => {},
            SerperError::Request(_) => {},  // Some HTTP clients may surface this as a request error
            other => panic!("Expected Json or Request error, got: {:?}", other),
        }
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_multiple_success() {
        let mut server = Server::new_async().await;
        
        let mock_response1 = json!({
            "organic": [
                {"title": "Result 1", "link": "https://example1.com", "position": 1}
            ]
        });
        
        let mock_response2 = json!({
            "organic": [
                {"title": "Result 2", "link": "https://example2.com", "position": 1}
            ]
        });

        let mock1 = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .match_body(Matcher::JsonString(json!({"q": "query1"}).to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response1.to_string())
            .create_async()
            .await;

        let mock2 = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .match_body(Matcher::JsonString(json!({"q": "query2"}).to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response2.to_string())
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let queries = vec![
            SearchQuery::new("query1".to_string()),
            SearchQuery::new("query2".to_string()),
        ];
        
        let result = client.search_multiple(queries).await;
        
        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 2);
        
        mock1.assert_async().await;
        mock2.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_multiple_first_fails() {
        let mut server = Server::new_async().await;

        let mock1 = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
            .match_body(Matcher::JsonString(json!({"q": "query1"}).to_string()))
            .with_status(500)
            .create_async()
            .await;

        let client = SerperClient::new_with_base_url(
            "test-key".to_string(), 
            server.url()
        ).unwrap();
        
        let queries = vec![
            SearchQuery::new("query1".to_string()),
            SearchQuery::new("query2".to_string()),
        ];
        
        let result = client.search_multiple(queries).await;
        
        assert!(result.is_err());
        mock1.assert_async().await;
    }

    #[tokio::test]
    async fn test_search_multiple_empty_queries() {
        let client = SerperClient::new("test-key".to_string()).unwrap();
        let queries = vec![];
        let result = client.search_multiple(queries).await;
        
        assert!(result.is_ok());
        let responses = result.unwrap();
        assert_eq!(responses.len(), 0);
    }

    #[tokio::test]
    async fn test_search_with_full_query_parameters() {
        let mut server = Server::new_async().await;
        
        let expected_body = json!({
            "q": "test query",
            "location": "Paris",
            "gl": "fr",
            "hl": "en",
            "page": 2,
            "num": 20
        });

        let mock = server.mock("POST", "/search")
            .match_header("X-API-KEY", "test-key")
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
        
        let query = SearchQuery::new("test query".to_string())
            .with_location("Paris".to_string())
            .with_country("fr".to_string())
            .with_language("en".to_string())
            .with_page(2)
            .with_num_results(20);
            
        let result = client.search(query).await;
        assert!(result.is_ok());
        
        mock.assert_async().await;
    }
}