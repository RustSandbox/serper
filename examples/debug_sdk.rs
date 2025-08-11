/// Debug version of SDK to see exactly what's being sent to the API
use serper_sdk::{SearchQuery, SearchService};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Debug SDK - Testing HTTP Request\n");

    let api_key =
        env::var("SERPER_API_KEY").expect("Please set SERPER_API_KEY environment variable");

    println!("✅ API Key loaded (length: {})", api_key.len());

    // Create search service
    let service = SearchService::new(api_key.clone())?;

    // Create a simple query
    let query = SearchQuery::new("Rust programming language".to_string())?
        .with_country("us".to_string())
        .with_language("en".to_string());

    println!("🔍 Query created:");
    println!("   - q: {}", query.q);
    println!("   - gl: {:?}", query.gl);
    println!("   - hl: {:?}", query.hl);
    println!("   - location: {:?}", query.location);
    println!("   - page: {:?}", query.page);

    // Serialize the query to see what JSON is being sent
    let json_query = serde_json::to_string_pretty(&query)?;
    println!("\n📦 JSON payload that will be sent:");
    println!("{}", json_query);

    println!("\n⏳ Making search request...");

    // Try the search
    match service.search(&query).await {
        Ok(response) => {
            println!("✅ Search successful!");
            println!("📊 Organic results: {}", response.organic_count());

            if let Some(metadata) = &response.search_metadata {
                println!("🔍 Search ID: {}", metadata.id);
                println!("⏱️ Request time: {:.2}s", metadata.request_time_taken);
            }
        }
        Err(e) => {
            println!("❌ Search failed: {}", e);

            // The error should contain the HTTP status and details
            let error_str = format!("{}", e);
            println!("📄 Error details: {}", error_str);
        }
    }

    Ok(())
}
