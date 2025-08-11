/// Example using environment variables for configuration with real search
use serper_sdk::{SdkConfig, SearchQuery, SearchService};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Environment-based Configuration with Real Search Example\n");

    // Method 1: Using SdkConfig::from_env() (preferred)
    println!("📋 Loading Configuration from Environment");
    match SdkConfig::from_env() {
        Ok(config) => {
            println!("✅ Configuration loaded from environment variables");
            println!("   - Base URL: {}", config.base_url);
            println!("   - Timeout: {:?}", config.timeout);
            println!("   - Max concurrent: {}", config.max_concurrent_requests);
            println!("   - User agent: {}", config.user_agent);
            println!("   - Logging enabled: {}\n", config.enable_logging);

            // Create search service with environment config
            let service = SearchService::new(config.api_key.clone())?;

            // Create a query for technology search
            let query = SearchQuery::new("modern web development frameworks".to_string())?
                .with_country("us".to_string())
                .with_language("en".to_string())
                .with_page(1);

            println!("🔍 Performing Search:");
            println!("   Query: '{}'", query.q);
            println!("   Location: {:?}", query.location);
            println!("   Country (gl): {:?}", query.gl);
            println!("   Language (hl): {:?}", query.hl);
            println!("   Page: {:?}\n", query.page);

            // Perform the actual search
            println!("⏳ Searching...");
            match service.search(&query).await {
                Ok(response) => {
                    println!("✅ Search completed successfully!\n");

                    // Display search metadata
                    if let Some(metadata) = &response.search_metadata {
                        println!("📊 Search Metadata:");
                        println!("   - Search ID: {}", metadata.id);
                        println!("   - Status: {}", metadata.status);
                        println!("   - Request time: {:.2}s", metadata.request_time_taken);
                        println!("   - Total time: {:.2}s\n", metadata.total_time_taken);
                    }

                    // Display answer box if available
                    if let Some(answer_box) = &response.answer_box {
                        println!("💡 Answer Box:");
                        if let Some(answer) = &answer_box.answer {
                            println!("   Answer: {}", answer);
                        }
                        if let Some(snippet) = &answer_box.snippet {
                            println!("   Snippet: {}", snippet);
                        }
                        if let Some(title) = &answer_box.title {
                            println!("   Title: {}", title);
                        }
                        if let Some(link) = &answer_box.link {
                            println!("   Link: {}", link);
                        }
                        println!();
                    }

                    // Display knowledge graph if available
                    if let Some(kg) = &response.knowledge_graph {
                        println!("🧠 Knowledge Graph:");
                        if let Some(title) = &kg.title {
                            println!("   Title: {}", title);
                        }
                        if let Some(description) = &kg.description {
                            println!("   Description: {}", description);
                        }
                        if let Some(entity_type) = &kg.entity_type {
                            println!("   Type: {}", entity_type);
                        }
                        if let Some(website) = &kg.website {
                            println!("   Website: {}", website);
                        }
                        println!();
                    }

                    // Display organic results
                    let organic_results = response.organic_results();
                    if !organic_results.is_empty() {
                        println!("🔗 Organic Results ({} total):", organic_results.len());
                        for (i, result) in organic_results.iter().enumerate().take(5) {
                            println!("   {}. {}", i + 1, result.title);
                            println!("      URL: {}", result.link);
                            if let Some(domain) = result.domain() {
                                println!("      Domain: {}", domain);
                            }
                            if let Some(snippet) = &result.snippet {
                                let truncated_snippet = if snippet.len() > 150 {
                                    format!("{}...", &snippet[..150])
                                } else {
                                    snippet.clone()
                                };
                                println!("      Snippet: {}", truncated_snippet);
                            }
                            println!();
                        }

                        if organic_results.len() > 5 {
                            println!("   ... and {} more results", organic_results.len() - 5);
                        }
                    } else {
                        println!("❌ No organic results found");
                    }

                    // Display related questions if available
                    if let Some(related_questions) = &response.related_questions {
                        if !related_questions.is_empty() {
                            println!("❓ Related Questions:");
                            for (i, question) in related_questions.iter().enumerate().take(3) {
                                println!("   {}. {}", i + 1, question.question);
                            }
                            println!();
                        }
                    }

                    println!(
                        "✨ Search completed! Found {} organic results",
                        response.organic_count()
                    );
                }
                Err(e) => {
                    println!("❌ Search failed: {}", e);
                    println!("💡 This might be due to:");
                    println!("   - Invalid API key");
                    println!("   - Network connectivity issues");
                    println!("   - API rate limits");
                    println!("   - Invalid query parameters\n");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load config from environment: {}", e);
            println!("💡 Falling back to Method 2...\n");

            // Method 2: Manual environment variable reading
            println!("📋 Method 2: Manual environment variable reading");
            let api_key = env::var("SERPER_API_KEY").unwrap_or_else(|_| {
                println!("⚠️ SERPER_API_KEY not set, using placeholder");
                "your-api-key-here".to_string()
            });

            let config = SdkConfig::new(api_key).with_user_agent("EnvConfigDemo/1.0".to_string());

            println!("✅ Configuration created manually");
            println!(
                "   - API key: {}",
                if config.api_key == "your-api-key-here" {
                    "placeholder (set SERPER_API_KEY)"
                } else {
                    "loaded from environment"
                }
            );
            println!("   - User agent: {}\n", config.user_agent);
        }
    }

    // Show expected environment variables
    println!("📝 Expected Environment Variables:");
    println!("   - SERPER_API_KEY (required) - Your Serper API key");
    println!("   - SERPER_BASE_URL (optional) - API base URL");
    println!("   - SERPER_TIMEOUT_SECS (optional) - Request timeout in seconds");
    println!("   - SERPER_MAX_CONCURRENT (optional) - Max concurrent requests");
    println!("   - SERPER_USER_AGENT (optional) - Custom user agent");
    println!("   - SERPER_ENABLE_LOGGING (optional) - Enable logging (true/false)");

    println!("\n💡 Example usage:");
    println!("   export SERPER_API_KEY=\"your-actual-api-key\"");
    println!("   export SERPER_TIMEOUT_SECS=\"60\"");
    println!("   export SERPER_ENABLE_LOGGING=\"true\"");
    println!("   cargo run --example env_config");

    Ok(())
}
