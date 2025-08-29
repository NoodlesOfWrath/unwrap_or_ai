use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use unwrap_or_ai::unwrap_or_ai;
use unwrap_or_ai_proc_macro::unwrap_or_ai_func;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    department: String,
}

// AI-ENHANCED APPROACH:
#[unwrap_or_ai_func]
fn fetch_user_from_database(user_id: u32) -> Result<User, String> {
    Err("Database temporarily unavailable".to_string())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // INTELLIGENT RECOVERY IN ACTION:
    let user = unwrap_or_ai!(fetch_user_from_database(12345)).await;

    println!("AI-generated user: {}", user.name);
}
