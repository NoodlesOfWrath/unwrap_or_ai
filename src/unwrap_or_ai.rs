use kalosm_language::prelude::{ChatModelExt, OpenAICompatibleChatModel, OpenAICompatibleClient};

// Helper trait to extract the inner type and handle AI recovery
#[allow(async_fn_in_trait)]
pub trait UnwrapOrAi<T> {
    async fn unwrap_or_ai_impl(self, prompt: String) -> Self;
}

impl<T, E> UnwrapOrAi<T> for Result<T, E>
where
    T: serde::de::DeserializeOwned
        + kalosm_language::prelude::Schema
        + Unpin
        + Clone
        + Send
        + Sync
        + 'static,
{
    async fn unwrap_or_ai_impl(self, prompt: String) -> Self {
        match self {
            Ok(val) => Ok(val),
            Err(e) => {
                // Call AI for recovery
                match call_ai_for_type::<T>(prompt).await {
                    Ok(ai_result) => Ok(ai_result),
                    Err(_) => Err(e), // Return original error if AI fails
                }
            }
        }
    }
}

impl<T> UnwrapOrAi<T> for Option<T>
where
    T: serde::de::DeserializeOwned
        + kalosm_language::prelude::Schema
        + Unpin
        + Clone
        + Send
        + Sync
        + 'static,
{
    async fn unwrap_or_ai_impl(self, prompt: String) -> Self {
        match self {
            Some(val) => Some(val),
            None => {
                // Call AI for recovery
                match call_ai_for_type::<T>(prompt).await {
                    Ok(ai_result) => Some(ai_result),
                    Err(_) => None, // Return None if AI fails
                }
            }
        }
    }
}

// Helper function to call AI and deserialize to specific type T
pub async fn call_ai_for_type<T>(prompt: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned
        + kalosm_language::prelude::Schema
        + Unpin
        + Clone
        + Send
        + Sync
        + 'static,
{
    let api_key =
        std::env::var("CEREBRAS_API").map_err(|_| "CEREBRAS_API environment variable not set")?;

    // Create Cerebras client using OpenAI-compatible interface
    let client = OpenAICompatibleClient::new()
        .with_base_url("https://api.cerebras.ai/v1")
        .with_api_key(api_key);

    let llm = OpenAICompatibleChatModel::builder()
        .with_client(client)
        .with_model("llama3.1-8b")
        .build();

    let task = llm.task("You are an AI error recovery assistant. When given an error message and program context, your task is to infer the most likely intended response or output. 
                Do not explain the error—directly provide the corrected or plausible output as if the error had not occurred.")
                .typed();

    let ai_response = task(&prompt).await?;

    Ok(ai_response)
}

#[macro_export]
macro_rules! unwrap_or_ai {
    ($fn_name:ident($($args:expr),*)) => {{
        use $crate::unwrap_or_ai::UnwrapOrAi;

        async {
            // Call the original function
            let result = $fn_name($($args),*);

            // Prepare the prompt for the AI with function context
            let prompt = format!(
                "The following function call failed: {}({})
                Function name: {}
                Parameters: {:?}
                
                This function should return the appropriate type. Generate a reasonable response as valid JSON.",
                stringify!($fn_name),
                stringify!($($args),*),
                stringify!($fn_name),
                ($($args),*)
            );

            // Use the trait method to handle AI recovery with proper type inference
            result.unwrap_or_ai_impl(prompt).await
        }
    }};

    // Fallback for other expressions
    ($fn_call:expr) => {{
        use $crate::unwrap_or_ai::UnwrapOrAi;

        async {
            // Call the original function
            let result = $fn_call;

            // Prepare the prompt for the AI
            let prompt = format!(
                "The following function call failed: {}
                
                Generate a reasonable response as valid JSON that matches the expected return type.",
                stringify!($fn_call)
            );

            // Use the trait method to handle AI recovery with proper type inference
            result.unwrap_or_ai_impl(prompt).await
        }
    }};
}
