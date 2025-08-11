use serper_sdk::{SerperClient, SearchQuery};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get API key from environment variable
    let api_key = env::var("SERPER_API_KEY")
        .expect("Please set SERPER_API_KEY environment variable");
    
    let client = SerperClient::new(api_key)?;
    
    let query = SearchQuery::new("Rust programming".to_string())
        .with_location("France".to_string())
        .with_country("fr".to_string())
        .with_language("fr".to_string())
        .with_page(1);
    
    let result = client.search(query).await?;
    
    if let Some(organic) = result.organic {
        for (i, item) in organic.iter().take(5).enumerate() {
            println!("{}. {} - {}", i + 1, item.title, item.link);
        }
    }
    
    let batch_queries = vec![
        SearchQuery::new("Hamze Ghalebi".to_string()).with_location("France".to_string()),
        SearchQuery::new("google inc".to_string()).with_location("France".to_string()),
        SearchQuery::new("tesla inc".to_string()).with_location("France".to_string()),
    ];
    
    let batch_results = client.search_multiple(batch_queries).await?;
    println!("Processed {} batch results", batch_results.len());
    
    Ok(())
}