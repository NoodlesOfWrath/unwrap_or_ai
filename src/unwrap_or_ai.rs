use crate::groq_client::{GroqClient, models};

// Helper trait to extract the inner type and handle AI recovery
#[allow(async_fn_in_trait)]
pub trait UnwrapOrAi<T> {
    async fn unwrap_or_ai_impl(self, prompt: String) -> T;
}

impl<T, E> UnwrapOrAi<T> for Result<T, E>
where
    T: serde::de::DeserializeOwned + schemars::JsonSchema + Unpin + Clone + Send + Sync + 'static,
{
    async fn unwrap_or_ai_impl(self, prompt: String) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                println!("Result error detected, calling AI for recovery...");
                // Call AI for recovery
                match call_ai_for_type::<T>(prompt).await {
                    Ok(ai_result) => ai_result,
                    Err(ai_error) => {
                        panic!("AI recovery failed: {}", ai_error);
                    }
                }
            }
        }
    }
}

impl<T> UnwrapOrAi<T> for Option<T>
where
    T: serde::de::DeserializeOwned + schemars::JsonSchema + Unpin + Clone + Send + Sync + 'static,
{
    async fn unwrap_or_ai_impl(self, prompt: String) -> T {
        match self {
            Some(val) => val,
            None => {
                println!("Option is None, calling AI for recovery...");
                // Call AI for recovery
                match call_ai_for_type::<T>(prompt).await {
                    Ok(ai_result) => {
                        println!("AI recovery successful!");
                        ai_result
                    }
                    Err(ai_error) => {
                        panic!("AI recovery failed: {}", ai_error);
                    }
                }
            }
        }
    }
}

// Helper function to call AI and deserialize to specific type T
pub async fn call_ai_for_type<T>(prompt: String) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + schemars::JsonSchema + Unpin + Clone + Send + Sync + 'static,
{
    let api_key = std::env::var("GROQ_API").map_err(|_| "GROQ_API environment variable not set")?;

    // Create Groq client using our direct HTTP client
    let groq = GroqClient::new(api_key);

    let ai_response: T = groq.chat_completion_typed(
        models::KIMI_K2, // Use a model that supports structured output
        vec![
            ("system", "You are an AI error recovery assistant. When given an error message and program context, your task is to infer the most likely intended response or output. Do not explain the error—directly provide the corrected or plausible output as if the error had not occurred."),
            ("user", &prompt)
        ]
    ).await?;

    Ok(ai_response)
}

#[macro_export]
macro_rules! unwrap_or_ai {
    ($fn_name:ident($($args:expr),*)) => {{
        use $crate::unwrap_or_ai::UnwrapOrAi;

        async {
            // Call the original function
            let result = $fn_name($($args),*);

            let source_code = paste::paste! { [<print_source_of_ $fn_name>]() };
            // Prepare the prompt for the AI with function context
            let prompt = format!(
                "The following function call failed: {}({})
                Function name: {}
                Parameters: {:?}
                Source code: {}
                
                This function should return the appropriate type. Generate a reasonable response as valid JSON.",
                stringify!($fn_name),
                stringify!($($args),*),
                stringify!($fn_name),
                stringify!($($args),*),
                source_code
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

            println!("Prompt for AI: {}", prompt);

            // Use the trait method to handle AI recovery with proper type inference
            result.unwrap_or_ai_impl(prompt).await
        }
    }};
}
