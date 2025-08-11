use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchQuery {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<u32>,
}

impl SearchQuery {
    pub fn new(query: String) -> Self {
        Self {
            q: query,
            location: None,
            gl: None,
            hl: None,
            page: None,
            num: None,
        }
    }
    
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn with_country(mut self, country: String) -> Self {
        self.gl = Some(country);
        self
    }
    
    pub fn with_language(mut self, language: String) -> Self {
        self.hl = Some(language);
        self
    }
    
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
    
    pub fn with_num_results(mut self, num: u32) -> Self {
        self.num = Some(num);
        self
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchResponse {
    pub search_metadata: Option<SearchMetadata>,
    pub organic: Option<Vec<OrganicResult>>,
    pub answer_box: Option<AnswerBox>,
    pub knowledge_graph: Option<KnowledgeGraph>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchMetadata {
    pub id: String,
    pub status: String,
    pub created_at: String,
    pub request_time_taken: f64,
    pub total_time_taken: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrganicResult {
    pub title: String,
    pub link: String,
    pub snippet: Option<String>,
    pub position: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnswerBox {
    pub answer: Option<String>,
    pub snippet: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KnowledgeGraph {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_search_query_new() {
        let query = SearchQuery::new("test query".to_string());
        assert_eq!(query.q, "test query");
        assert_eq!(query.location, None);
        assert_eq!(query.gl, None);
        assert_eq!(query.hl, None);
        assert_eq!(query.page, None);
        assert_eq!(query.num, None);
    }

    #[test]
    fn test_search_query_with_location() {
        let query = SearchQuery::new("test".to_string())
            .with_location("Paris".to_string());
        assert_eq!(query.location, Some("Paris".to_string()));
    }

    #[test]
    fn test_search_query_with_country() {
        let query = SearchQuery::new("test".to_string())
            .with_country("fr".to_string());
        assert_eq!(query.gl, Some("fr".to_string()));
    }

    #[test]
    fn test_search_query_with_language() {
        let query = SearchQuery::new("test".to_string())
            .with_language("en".to_string());
        assert_eq!(query.hl, Some("en".to_string()));
    }

    #[test]
    fn test_search_query_with_page() {
        let query = SearchQuery::new("test".to_string())
            .with_page(2);
        assert_eq!(query.page, Some(2));
    }

    #[test]
    fn test_search_query_with_num_results() {
        let query = SearchQuery::new("test".to_string())
            .with_num_results(20);
        assert_eq!(query.num, Some(20));
    }

    #[test]
    fn test_search_query_chaining() {
        let query = SearchQuery::new("test".to_string())
            .with_location("Paris".to_string())
            .with_country("fr".to_string())
            .with_language("en".to_string())
            .with_page(1)
            .with_num_results(10);
        
        assert_eq!(query.q, "test");
        assert_eq!(query.location, Some("Paris".to_string()));
        assert_eq!(query.gl, Some("fr".to_string()));
        assert_eq!(query.hl, Some("en".to_string()));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.num, Some(10));
    }

    #[test]
    fn test_search_query_serialize() {
        let query = SearchQuery::new("test query".to_string())
            .with_location("Paris".to_string());
        
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"q\":\"test query\""));
        assert!(json.contains("\"location\":\"Paris\""));
    }

    #[test]
    fn test_search_query_serialize_minimal() {
        let query = SearchQuery::new("test".to_string());
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"q\":\"test\""));
        assert!(!json.contains("\"location\""));
        assert!(!json.contains("\"gl\""));
        assert!(!json.contains("\"hl\""));
        assert!(!json.contains("\"page\""));
        assert!(!json.contains("\"num\""));
    }

    #[test]
    fn test_organic_result_deserialize() {
        let json = r#"
        {
            "title": "Test Title",
            "link": "https://example.com",
            "snippet": "Test snippet",
            "position": 1
        }
        "#;
        
        let result: OrganicResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.title, "Test Title");
        assert_eq!(result.link, "https://example.com");
        assert_eq!(result.snippet, Some("Test snippet".to_string()));
        assert_eq!(result.position, 1);
    }

    #[test]
    fn test_organic_result_deserialize_no_snippet() {
        let json = r#"
        {
            "title": "Test Title",
            "link": "https://example.com",
            "position": 1
        }
        "#;
        
        let result: OrganicResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.snippet, None);
    }

    #[test]
    fn test_search_response_deserialize_empty() {
        let json = "{}";
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.search_metadata, None);
        assert_eq!(response.organic, None);
        assert_eq!(response.answer_box, None);
        assert_eq!(response.knowledge_graph, None);
    }

    #[test]
    fn test_search_metadata_deserialize() {
        let json = r#"
        {
            "id": "test-id",
            "status": "success",
            "created_at": "2023-01-01T00:00:00Z",
            "request_time_taken": 0.5,
            "total_time_taken": 1.0
        }
        "#;
        
        let metadata: SearchMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.id, "test-id");
        assert_eq!(metadata.status, "success");
        assert_eq!(metadata.request_time_taken, 0.5);
        assert_eq!(metadata.total_time_taken, 1.0);
    }

    #[test]
    fn test_answer_box_deserialize() {
        let json = r#"
        {
            "answer": "Test answer",
            "snippet": "Test snippet"
        }
        "#;
        
        let answer_box: AnswerBox = serde_json::from_str(json).unwrap();
        assert_eq!(answer_box.answer, Some("Test answer".to_string()));
        assert_eq!(answer_box.snippet, Some("Test snippet".to_string()));
    }

    #[test]
    fn test_knowledge_graph_deserialize() {
        let json = r#"
        {
            "title": "Test Title",
            "description": "Test Description"
        }
        "#;
        
        let kg: KnowledgeGraph = serde_json::from_str(json).unwrap();
        assert_eq!(kg.title, Some("Test Title".to_string()));
        assert_eq!(kg.description, Some("Test Description".to_string()));
    }
}