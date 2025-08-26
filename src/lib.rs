pub use unwrap_or_ai_proc_macro;

#[macro_use]
pub mod unwrap_or_ai;

#[cfg(test)]
mod tests {
    use kalosm_language::prelude::{
        ChatModelExt, OpenAICompatibleChatModel, OpenAICompatibleClient,
    };
    use kalosm_sample::Schema;
    use serde::{Deserialize, Serialize};
    use unwrap_or_ai_proc_macro::unwrap_or_ai_func;

    // Import the helper functions and traits
    use crate::unwrap_or_ai::{UnwrapOrAi, call_ai_for_type};

    use super::*;

    // Test data structures with comprehensive documentation for AI context
    /// Represents a user in our system with basic profile information.
    /// This structure contains the essential fields needed to identify and contact a user.
    /// Fields:
    /// - id: Unique identifier for the user (positive integer)
    /// - name: Full name of the user (first and last name)
    /// - email: Valid email address for contacting the user
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Schema)]
    struct TestUser {
        id: u32,
        name: String,
        email: String,
    }

    /// Represents a product in our catalog with pricing information.
    /// This structure contains the core product details needed for commerce operations.
    /// Fields:
    /// - id: Unique product identifier (positive integer)
    /// - name: Product name or title (descriptive string)
    /// - price: Product price in USD (positive decimal number)
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Schema)]
    struct TestProduct {
        id: u32,
        name: String,
        price: f64,
    }

    // Well-documented mock functions that return Results and Options for testing

    /// Retrieves a user by ID from the database.
    /// This function simulates a successful database lookup operation.
    ///
    /// Parameters:
    /// - id: The unique user identifier to look up
    ///
    /// Returns:
    /// - Ok(TestUser): A user object with the specified ID, default name "John Doe",
    ///   and email "john@example.com"
    ///
    /// Example usage:
    /// ```
    /// let user = get_user_success(1).unwrap();
    /// assert_eq!(user.name, "John Doe");
    /// ```
    #[unwrap_or_ai_func]
    fn get_user_success(id: u32) -> Result<TestUser, String> {
        Ok(TestUser {
            id,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        })
    }

    /// Attempts to retrieve a user by ID but always fails.
    /// This function simulates a database lookup that fails because the user doesn't exist.
    ///
    /// Parameters:
    /// - id: The user ID that will not be found
    ///
    /// Returns:
    /// - Err(String): An error message indicating the user was not found
    ///
    /// This function is designed to test error recovery scenarios where the AI
    /// should generate a plausible user object when the lookup fails.
    #[unwrap_or_ai_func]
    fn get_user_failure(id: u32) -> Result<TestUser, String> {
        Err(format!("User with id {} not found in database", id))
    }

    /// Retrieves product information by ID.
    /// This function simulates a successful product catalog lookup.
    ///
    /// Parameters:
    /// - id: The unique product identifier
    ///
    /// Returns:
    /// - Some(TestProduct): A product with the given ID, name "Test Product", and price $99.99
    ///
    /// The returned product represents a standard catalog item with reasonable defaults.
    #[unwrap_or_ai_func]
    fn get_optional_product_some(id: u32) -> Option<TestProduct> {
        Some(TestProduct {
            id,
            name: "Test Product".to_string(),
            price: 99.99,
        })
    }

    /// Attempts to find a product but returns None (product not found).
    /// This function simulates a product lookup that fails to find any matching item.
    ///
    /// Parameters:
    /// - _id: The product ID to search for (ignored since no product is found)
    ///
    /// Returns:
    /// - None: Indicates no product was found
    ///
    /// This is used to test scenarios where the AI should generate a reasonable
    /// product suggestion when the original lookup fails.
    #[unwrap_or_ai_func]
    fn get_optional_product_none(_id: u32) -> Option<TestProduct> {
        None
    }

    /// Simulates fetching user preferences that might fail.
    /// This function represents getting user settings or preferences from a service
    /// that might be temporarily unavailable.
    ///
    /// Parameters:
    /// - user_id: The ID of the user whose preferences to fetch
    /// - preference_type: The type of preference (e.g., "theme", "language", "notifications")
    ///
    /// Returns:
    /// - Err: Always fails to simulate a service outage
    ///
    /// When this fails, the AI should generate reasonable default preferences.
    #[unwrap_or_ai_func]
    fn get_user_preferences(user_id: u32, preference_type: &str) -> Result<TestUser, String> {
        Err(format!(
            "Preferences service unavailable for user {} requesting {}",
            user_id, preference_type
        ))
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_successful_result() {
        // Test that successful Results are returned as-is without calling AI
        let result = unwrap_or_ai!(get_user_success(1)).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_successful_option() {
        // Test that Some Options are returned as-is without calling AI
        let result = unwrap_or_ai!(get_optional_product_some(1)).await;

        assert!(result.is_some());
        let product = result.unwrap();
        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Test Product");
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_failed_result_no_api_key() {
        // Test that failed Results return the original error when no API key is set
        // First, ensure CEREBRAS_API is not set
        unsafe {
            std::env::remove_var("CEREBRAS_API");
        }

        let result = unwrap_or_ai!(get_user_failure(999)).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, "User with id 999 not found in database");
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_real_api_call_failed_result() {
        // Test that when a function fails and API key is set, we get an AI-generated response

        // Skip if no API key is set
        if std::env::var("CEREBRAS_API").is_err() {
            println!("Skipping test - CEREBRAS_API environment variable not set");
            return;
        }

        let result = unwrap_or_ai!(get_user_failure(42)).await;

        // The AI should have generated a user, so this should be Ok
        assert!(
            result.is_ok(),
            "Expected AI to generate a user when original function failed"
        );

        let user = result.unwrap();
        assert_eq!(user.id, 42, "AI should preserve the requested user ID");
        assert!(!user.name.is_empty(), "AI should generate a non-empty name");
        assert!(
            !user.email.is_empty(),
            "AI should generate a non-empty email"
        );
        assert!(
            user.email.contains('@'),
            "AI should generate a valid email format"
        );

        println!("AI generated user: {:?}", user);
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_real_api_call_none_option() {
        // Test that when an Option function returns None and API key is set, we get an AI-generated response
        if std::env::var("CEREBRAS_API").is_err() {
            println!("Skipping test - CEREBRAS_API environment variable not set");
            return;
        }

        let result = unwrap_or_ai!(get_optional_product_none(123)).await;

        // The AI should have generated a product, so this should be Some
        assert!(
            result.is_some(),
            "Expected AI to generate a product when original function returned None"
        );

        let product = result.unwrap();
        print!("product: {:?}", product);
        assert_eq!(
            product.id, 123,
            "AI should preserve the requested product ID"
        );
        assert!(
            !product.name.is_empty(),
            "AI should generate a non-empty product name"
        );
        assert!(product.price > 0.0, "AI should generate a positive price");

        println!("AI generated product: {:?}", product);
    }

    #[tokio::test]
    async fn test_api_connection_debug() {
        // Debug test to check what's wrong with the API connection
        if std::env::var("CEREBRAS_API").is_err() {
            println!("Skipping test - CEREBRAS_API environment variable not set");
            return;
        }

        let api_key = std::env::var("CEREBRAS_API").unwrap();

        // Create Cerebras client using OpenAI-compatible interface
        let client = OpenAICompatibleClient::new()
            .with_base_url("https://api.cerebras.ai/v1")
            .with_api_key(api_key);

        let llm = OpenAICompatibleChatModel::builder()
            .with_client(client)
            .with_model("qwen-3-coder-480b")
            .build();

        println!("LLM created successfully");

        // Try a simple chat call first (non-structured)
        let mut chat = llm.chat();
        match chat("Hello, please respond with just 'Hi'").await {
            Ok(response) => {
                println!("Simple chat worked: {}", response);
            }
            Err(e) => {
                println!("Simple chat failed: {}", e);
                panic!("Error details: {:?}", e);
            }
        }

        println!("Chat test passed, trying structured generation...");

        // Now try structured generation
        let task = llm
            .task("Generate a simple test response")
            .typed::<TestUser>();
        let result =
            task("Generate a user with ID 123, name 'Test', email 'test@example.com'").await;

        match result {
            Ok(user) => {
                println!("Structured generation worked: {:?}", user);
            }
            Err(e) => {
                println!("Structured generation failed: {}", e);
                println!("Error details: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_ai_with_complex_context() {
        // Test AI with a more complex scenario that includes context from function docs

        if std::env::var("CEREBRAS_API").is_err() {
            println!("Skipping test - CEREBRAS_API environment variable not set");
            return;
        }

        let result = unwrap_or_ai!(get_user_preferences(789, "theme")).await;

        match &result {
            Ok(user) => {
                println!(
                    "Success! AI generated user from preferences failure: {:?}",
                    user
                );
            }
            Err(e) => {
                println!("Unwrap or AI failed with error: {}", e);
                println!("Error details: {:?}", e);
            }
        }

        assert!(
            result.is_ok(),
            "AI should generate a user when preferences service fails. Error: {:?}",
            result.err()
        );

        let user = result.unwrap();
        assert_eq!(
            user.id, 789,
            "AI should preserve the user ID from the failed call"
        );
        assert!(!user.name.is_empty(), "AI should generate a user name");
        assert!(!user.email.is_empty(), "AI should generate an email");

        println!("AI generated user from preferences failure: {:?}", user);
    }

    #[tokio::test]
    async fn test_unwrap_or_ai_with_none_option_no_api_key() {
        // Test that None Options remain None when no API key is set
        unsafe {
            std::env::remove_var("CEREBRAS_API");
        }

        let result = unwrap_or_ai!(get_optional_product_none(999)).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_call_ai_for_type_no_api_key() {
        // Test the helper function behavior when no API key is set
        unsafe {
            std::env::remove_var("CEREBRAS_API");
        }

        let prompt = "Test prompt".to_string();
        let result = call_ai_for_type::<TestUser>(prompt).await;

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("CEREBRAS_API environment variable not set"));
    }

    // Mock test for when API key is set (but we won't actually call the API)
    #[tokio::test]
    async fn test_trait_implementation_structure() {
        // This test verifies the trait implementations compile correctly

        // Test Result implementation
        let success_result: Result<TestUser, String> = Ok(TestUser {
            id: 1,
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
        });

        // We can't actually test the AI call without a real API key,
        // but we can test that the trait method exists and can be called
        let prompt = "test prompt".to_string();
        let _result = success_result.unwrap_or_ai_impl(prompt.clone()).await;

        // Test Option implementation
        let some_option: Option<TestProduct> = Some(TestProduct {
            id: 1,
            name: "Test Product".to_string(),
            price: 10.0,
        });

        let _option_result = some_option.unwrap_or_ai_impl(prompt).await;

        // If we get here, the trait implementations compiled and executed
        assert!(true);
    }

    #[test]
    fn test_source_code_generation_for_test_functions() {
        // Test that our test functions have their source code properly generated
        let user_source = print_source_of_get_user_success();
        assert!(user_source.contains("get_user_success"));
        assert!(user_source.contains("TestUser"));

        let product_source = print_source_of_get_optional_product_some();
        assert!(product_source.contains("get_optional_product_some"));
        assert!(product_source.contains("TestProduct"));
    }

    #[test]
    fn test_type_constraints() {
        // This test ensures our types implement the required traits
        fn assert_deserialize<T: serde::de::DeserializeOwned>() {}
        fn assert_serialize<T: serde::Serialize>() {}

        assert_deserialize::<TestUser>();
        assert_deserialize::<TestProduct>();
        assert_serialize::<TestUser>();
        assert_serialize::<TestProduct>();
    }
}
