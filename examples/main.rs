/// Main example - comprehensive search demonstration
use serper_sdk::{SdkConfig, SearchQuery, SearchService};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Serper SDK - Main Example");
    println!("Comprehensive search example with multiple queries\n");

    // Load configuration from environment
    let config = match SdkConfig::from_env() {
        Ok(config) => {
            println!("✅ Configuration loaded from environment");
            config
        }
        Err(_) => {
            // Fallback to manual API key reading
            let api_key = env::var("SERPER_API_KEY")
                .expect("❌ Please set SERPER_API_KEY environment variable");
            println!("✅ API key loaded from environment");
            SdkConfig::new(api_key)
        }
    };

    // Create search service
    let service = SearchService::new(config.api_key)?;

    // Create queries for different searches
    let queries = vec![
        SearchQuery::new("Hamze Ghalebi CTO at Remolab".to_string())?
            .with_country("us".to_string())
            .with_language("en".to_string()),
        SearchQuery::new("Hamze Ghalebi Remolab technology".to_string())?
            .with_country("us".to_string())
            .with_language("en".to_string()),
        SearchQuery::new("Remolab France innovation software".to_string())?
            .with_country("us".to_string())
            .with_language("en".to_string()),
    ];

    // Perform searches
    for (i, query) in queries.iter().enumerate() {
        println!("🔍 Search {} - Query: '{}'", i + 1, query.q);
        println!("⏳ Searching...");

        match service.search(query).await {
            Ok(response) => {
                println!("✅ Search completed!\n");

                // Show search metadata
                if let Some(metadata) = &response.search_metadata {
                    println!(
                        "📊 Metadata: {} results in {:.2}s",
                        response.organic_count(),
                        metadata.request_time_taken
                    );
                }

                // Show knowledge graph if available
                if let Some(kg) = &response.knowledge_graph {
                    println!("🧠 Knowledge Graph Found:");
                    if let Some(title) = &kg.title {
                        println!("   📌 {}", title);
                    }
                    if let Some(description) = &kg.description {
                        let desc = serper_sdk::utils::string::truncate(description, 200);
                        println!("   📝 {}", desc);
                    }
                    if let Some(website) = &kg.website {
                        println!("   🌐 {}", website);
                    }
                    println!();
                }

                // Show answer box if available
                if let Some(answer_box) = &response.answer_box {
                    println!("💡 Answer Box:");
                    if let Some(answer) = &answer_box.answer {
                        println!("   ➤ {}", answer);
                    }
                    if let Some(snippet) = &answer_box.snippet {
                        println!("   📄 {}", snippet);
                    }
                    println!();
                }

                // Show top organic results
                let organic_results = response.organic_results();
                if !organic_results.is_empty() {
                    println!("🔗 Top Results:");
                    for (idx, result) in organic_results.iter().enumerate().take(3) {
                        println!("   {}. 📌 {}", idx + 1, result.title);
                        println!("      🌐 {}", result.link);
                        if let Some(domain) = result.domain() {
                            println!("      🏢 Domain: {}", domain);
                        }
                        if let Some(snippet) = &result.snippet {
                            let snippet_preview = serper_sdk::utils::string::truncate(snippet, 120);
                            println!("      📄 {}", snippet_preview);
                        }
                        println!();
                    }
                } else {
                    println!("❌ No organic results found");
                }

                // Show related questions
                if let Some(related_questions) = &response.related_questions {
                    if !related_questions.is_empty() {
                        println!("❓ Related Questions:");
                        for (idx, question) in related_questions.iter().enumerate().take(2) {
                            println!("   {}. {}", idx + 1, question.question);
                        }
                        println!();
                    }
                }

                println!(
                    "✨ Found {} total results for '{}'\n",
                    response.organic_count(),
                    query.q
                );
            }
            Err(e) => {
                println!("❌ Search failed: {}", e);
                println!("💡 Please check your API key and internet connection\n");
            }
        }

        println!("{}", "=".repeat(80));
        println!();
    }

    println!("🎉 All searches completed!");
    println!("💡 To run this example: export SERPER_API_KEY=your-key && cargo run --example main");

    Ok(())
}
