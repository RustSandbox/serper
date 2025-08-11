/// Direct API test to debug the HTTP request
use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Direct API Test\n");

    let api_key = env::var("SERPER_API_KEY")
        .expect("Please set SERPER_API_KEY environment variable");

    println!("✅ API Key loaded (length: {})", api_key.len());

    let client = reqwest::Client::new();
    
    let payload = json!({
        "q": "JavaScript frameworks comparison",
        "gl": "us",
        "hl": "en"
    });

    println!("📡 Making request to: https://google.serper.dev/search");
    println!("📦 Payload: {}", payload);
    println!("🔑 API Key: {}...{}", &api_key[..8], &api_key[api_key.len()-8..]);

    let response = client
        .post("https://google.serper.dev/search")
        .header("X-API-KEY", &api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    println!("📊 Response Status: {}", response.status());
    println!("📋 Response Headers:");
    for (name, value) in response.headers() {
        println!("   {}: {:?}", name, value);
    }

    let response_text = response.text().await?;
    println!("📄 Response Body: {}", response_text);

    Ok(())
}