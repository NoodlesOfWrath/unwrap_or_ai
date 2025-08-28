use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;

/// Response types for Groq API
#[derive(Debug, Deserialize)]
pub struct GroqResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<GroqChoice>,
    pub usage: GroqUsage,
}

#[derive(Debug, Deserialize)]
pub struct GroqChoice {
    pub index: i32,
    pub message: GroqMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct GroqMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GroqUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// A simple client for Groq API that provides both simple and structured completions
pub struct GroqClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl GroqClient {
    /// Create a new Groq client with the given API key
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: "https://api.groq.com/openai/v1".to_string(),
        }
    }

    /// Simple chat completion - returns just the text content
    pub async fn chat_completion_simple(
        &self,
        model: &str,
        messages: Vec<(&str, &str)>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let messages: Vec<serde_json::Value> = messages
            .into_iter()
            .map(|(role, content)| {
                json!({
                    "role": role,
                    "content": content
                })
            })
            .collect();

        let request_body = json!({
            "model": model,
            "messages": messages
        });

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        let groq_response: GroqResponse = response.json().await?;

        if groq_response.choices.is_empty() {
            return Err("No choices in response".into());
        }

        Ok(groq_response.choices[0].message.content.clone())
    }

    /// Structured chat completion using JsonSchema trait - automatically generates JSON schema
    /// Only works with models that support structured output:
    /// - openai/gpt-oss-20b
    /// - openai/gpt-oss-120b  
    /// - moonshotai/kimi-k2-instruct
    /// - meta-llama/llama-4-maverick-17b-128e-instruct
    /// - meta-llama/llama-4-scout-17b-16e-instruct
    pub async fn chat_completion_typed<T>(
        &self,
        model: &str,
        messages: Vec<(&str, &str)>,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de> + JsonSchema,
    {
        let schema_name = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("response")
            .to_lowercase();

        let schema_def = schemars::schema_for!(T);
        let schema = serde_json::to_value(&schema_def)?;

        self.chat_completion_structured(model, messages, &schema_name, schema)
            .await
    }

    /// Structured chat completion - returns a typed response based on JSON schema
    /// Only works with models that support structured output:
    /// - openai/gpt-oss-20b
    /// - openai/gpt-oss-120b  
    /// - moonshotai/kimi-k2-instruct
    /// - meta-llama/llama-4-maverick-17b-128e-instruct
    /// - meta-llama/llama-4-scout-17b-16e-instruct
    pub async fn chat_completion_structured<T>(
        &self,
        model: &str,
        messages: Vec<(&str, &str)>,
        schema_name: &str,
        schema: serde_json::Value,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let messages: Vec<serde_json::Value> = messages
            .into_iter()
            .map(|(role, content)| {
                json!({
                    "role": role,
                    "content": content
                })
            })
            .collect();

        let request_body = json!({
            "model": model,
            "messages": messages,
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": schema_name,
                    "schema": schema
                }
            }
        });

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        let groq_response: GroqResponse = response.json().await?;

        if groq_response.choices.is_empty() {
            return Err("No choices in response".into());
        }

        let content = &groq_response.choices[0].message.content;
        let parsed: T = serde_json::from_str(content)?;
        Ok(parsed)
    }

    /// Helper function to create a JSON schema for simple types
    pub fn create_simple_schema(properties: Vec<(&str, &str, &str)>) -> serde_json::Value {
        let mut props = serde_json::Map::new();
        let mut required = Vec::new();

        for (name, type_str, description) in properties {
            props.insert(
                name.to_string(),
                json!({
                    "type": type_str,
                    "description": description
                }),
            );
            required.push(name);
        }

        json!({
            "type": "object",
            "properties": props,
            "required": required,
            "additionalProperties": false
        })
    }
}

/// Commonly used models for different purposes
pub mod models {
    /// Fast models - good for simple text generation
    pub const LLAMA3_8B: &str = "llama3-8b-8192";
    pub const LLAMA3_70B: &str = "llama3-70b-8192";

    /// Models that support structured output
    pub const GPT_OSS_20B: &str = "openai/gpt-oss-20b";
    pub const GPT_OSS_120B: &str = "openai/gpt-oss-120b";
    pub const KIMI_K2: &str = "moonshotai/kimi-k2-instruct";
    pub const LLAMA4_MAVERICK: &str = "meta-llama/llama-4-maverick-17b-128e-instruct";
    pub const LLAMA4_SCOUT: &str = "meta-llama/llama-4-scout-17b-16e-instruct";
}
