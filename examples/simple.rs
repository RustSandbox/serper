/// Simple usage example
use serper_sdk::{SearchQuery, SdkConfig};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Simple Serper SDK Example\n");

    // Get API key from environment variable
    let api_key = env::var("SERPER_API_KEY")
        .map_err(|_| "Please set SERPER_API_KEY environment variable")?;

    // Create and validate configuration
    let config = SdkConfig::new(api_key);
    config.validate()?;
    println!("✅ Configuration created and validated");

    // Create a search query
    let query = SearchQuery::new("Hamze Ghalebi CTO Remolab".to_string())?
        .with_country("us".to_string())
        .with_page(1);
    
    println!("✅ Search query created:");
    println!("   Query: '{}'", query.q);
    println!("   Location: {:?}", query.location);
    println!("   Page: {:?}", query.page);
    
    println!("\n🎯 Ready to search! (Would call SearchService::search() with real API key)");
    
    Ok(())
}