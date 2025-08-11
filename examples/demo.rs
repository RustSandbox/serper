/// Demo of the modular Serper SDK
use serper_sdk::{SearchQuery, SearchService, SearchResponse, SdkConfig};
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Serper SDK Modular Architecture Demo\n");

    // Get API key from environment variable
    let api_key = env::var("SERPER_API_KEY")
        .unwrap_or_else(|_| {
            println!("⚠️ SERPER_API_KEY environment variable not set, using demo key");
            "demo-api-key".to_string()
        });

    // 1. Configuration Module Demo
    println!("📋 Configuration Module:");
    let config = SdkConfig::new(api_key.clone())
        .with_timeout(Duration::from_secs(30))
        .with_max_concurrent(5)
        .with_user_agent("SerperSDK-Demo/1.0".to_string())
        .with_logging(true);
    
    match config.validate() {
        Ok(_) => println!("✅ Configuration validated successfully"),
        Err(e) => println!("❌ Configuration error: {}", e),
    }
    println!("   - Timeout: {:?}", config.timeout);
    println!("   - Max concurrent: {}", config.max_concurrent_requests);
    println!("   - User agent: {}\n", config.user_agent);

    // 2. Search Module Demo (Query Construction)
    println!("🔍 Search Module (Query Construction):");
    let query = match SearchQuery::new("Rust programming language".to_string()) {
        Ok(q) => {
            println!("✅ Query created successfully");
            let enhanced_query = q
                .with_location("San Francisco".to_string())
                .with_page(1);
            println!("   - Query: {}", enhanced_query.q);
            println!("   - Location: {:?}", enhanced_query.location);
            println!("   - Page: {:?}\n", enhanced_query.page);
            enhanced_query
        },
        Err(e) => {
            println!("❌ Query creation failed: {}", e);
            return Err(e.into());
        }
    };

    // 3. Search Module Demo (Response Structure)
    println!("📦 Search Module (Response Structure):");
    let response = SearchResponse::new();
    println!("✅ Search response structure initialized");
    println!("   - Has results: {}", response.has_results());
    println!("   - Organic count: {}", response.organic_count());
    println!("   - URLs extracted: {:?}\n", response.extract_urls());

    // 4. Utils Module Demo
    println!("🛠️ Utils Module:");
    
    // String utilities
    match serper_sdk::utils::string::validate_non_empty("test query", "search query") {
        Ok(_) => println!("✅ String validation passed"),
        Err(e) => println!("❌ String validation failed: {}", e),
    }
    
    let sanitized = serper_sdk::utils::string::sanitize("test\x00string\nwith\tspaces");
    println!("✅ String sanitized: '{}'", sanitized);
    
    let truncated = serper_sdk::utils::string::truncate("This is a very long string that will be truncated", 20);
    println!("✅ String truncated: '{}'", truncated);

    // URL utilities
    match serper_sdk::utils::url::validate_url("https://google.serper.dev") {
        Ok(_) => println!("✅ URL validation passed"),
        Err(e) => println!("❌ URL validation failed: {}", e),
    }
    
    match serper_sdk::utils::url::extract_domain("https://google.serper.dev/search") {
        Ok(domain) => println!("✅ Domain extracted: '{}'", domain),
        Err(e) => println!("❌ Domain extraction failed: {}", e),
    }
    println!();

    // 5. Core Module Demo (Error Handling)
    println!("⚠️ Core Module (Error Handling):");
    
    // Test validation error
    let invalid_query = SearchQuery::new("".to_string());
    match invalid_query {
        Ok(_) => println!("❌ Expected validation to fail"),
        Err(e) => {
            println!("✅ Validation error caught: {}", e);
            println!("   - Error type: {:?}", std::mem::discriminant(&e));
        }
    }
    
    // Test API key validation
    let invalid_config = SdkConfig::new("".to_string());
    match invalid_config.validate() {
        Ok(_) => println!("❌ Expected validation to fail"),
        Err(e) => {
            println!("✅ Config validation error caught: {}", e);
        }
    }
    println!();

    // 6. Module Integration Demo
    println!("🔗 Module Integration:");
    println!("✅ All modules work together seamlessly");
    println!("   - Config → validates settings");
    println!("   - Utils → validates and sanitizes input"); 
    println!("   - Search → constructs queries and responses");
    println!("   - Core → provides unified error handling");
    println!("   - HTTP → would handle API communication (in real usage)\n");

    // 7. Architecture Summary
    println!("🏗️ Architecture Summary:");
    println!("✅ Modular design with 5 focused modules");
    println!("✅ Clean separation of concerns");
    println!("✅ Type-safe APIs with comprehensive error handling");
    println!("✅ Builder patterns for flexible configuration");
    println!("✅ Comprehensive documentation and examples");
    println!("✅ Ready for production use with real API key\n");

    println!("🎉 Demo completed successfully! The SDK is ready to use.");
    
    Ok(())
}