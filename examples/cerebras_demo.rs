use kalosm_sample::Schema;
use serde::{Deserialize, Serialize};
use unwrap_or_ai::unwrap_or_ai;
use unwrap_or_ai_proc_macro::unwrap_or_ai_func;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
struct UserProfile {
    id: u32,
    name: String,
    email: String,
    department: String,
}

/// Simulates a database lookup that might fail.
/// This function represents a typical scenario where you're fetching user data
/// from a database or external service that might be temporarily unavailable.
#[unwrap_or_ai_func]
fn fetch_user_profile(user_id: u32) -> Result<UserProfile, String> {
    // Simulate a database failure
    Err(format!(
        "Database connection failed while looking up user {}",
        user_id
    ))
}

/// Attempts to get user preferences, which might return None if not configured.
/// This simulates optional data that might not exist for all users.
#[unwrap_or_ai_func]
fn get_user_preferences(_user_id: u32) -> Option<UserProfile> {
    // Simulate no preferences found
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing unwrap_or_ai with Cerebras API\n");

    // Check if API key is set
    if std::env::var("CEREBRAS_API").is_err() {
        println!("❌ CEREBRAS_API environment variable not set!");
        println!("   Please set your API key: export CEREBRAS_API=your_api_key_here");
        return Ok(());
    }

    println!("✅ API key found, testing AI recovery...\n");

    // Test 1: Result<T, E> failure recovery
    println!("🔄 Test 1: Recovering from database failure...");
    let user_result = unwrap_or_ai!(fetch_user_profile(123)).await;

    match user_result {
        Ok(user) => {
            println!("✅ AI recovered with user: {:?}", user);
            assert_eq!(user.id, 123);
            assert!(!user.name.is_empty());
            assert!(!user.email.is_empty());
        }
        Err(e) => {
            println!("❌ Recovery failed: {}", e);
        }
    }

    println!();

    // Test 2: Option<T> None recovery
    println!("🔄 Test 2: Recovering from missing preferences...");
    let prefs_result = unwrap_or_ai!(get_user_preferences(456)).await;

    match prefs_result {
        Some(user) => {
            println!("✅ AI generated preferences: {:?}", user);
            assert_eq!(user.id, 456);
            assert!(!user.department.is_empty());
        }
        None => {
            println!("❌ No preferences generated");
        }
    }

    println!("\n🎉 All tests completed!");

    Ok(())
}
