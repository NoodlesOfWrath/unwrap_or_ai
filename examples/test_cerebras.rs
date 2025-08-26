use kalosm_language::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Parse, Schema)]
struct SimpleResponse {
    message: String,
    success: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CEREBRAS_API")
        .expect("Please set the CEREBRAS_API environment variable with your Cerebras API key");

    println!("Testing Cerebras connection...");

    let client = OpenAICompatibleClient::new()
        .with_base_url("https://api.cerebras.ai/v1")
        .with_api_key(api_key);

    let llm = OpenAICompatibleChatModel::builder()
        .with_client(client)
        .with_model("llama3.1-8b") 
        .build();

    println!("Creating structured task...");

    let task = llm.task("You are a helpful assistant. Respond with a JSON object containing a message and success status.")
        .typed::<SimpleResponse>();

    println!("Making request to Cerebras...");

    let response: SimpleResponse = task("Hello, can you confirm you're working?").await?;

    println!("Response from Cerebras: {:?}", response);
    println!("✅ Cerebras connection successful!");

    Ok(())
}
