use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use unwrap_or_ai::unwrap_or_ai;
use unwrap_or_ai_proc_macro::unwrap_or_ai_func;

/// Represents a user profile in our application
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    department: String,
}

/// Represents a payment transaction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct PaymentResult {
    transaction_id: String,
    amount: f64,
    currency: String,
    status: String,
    timestamp: String,
}

/// Represents an API response from a weather service
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct WeatherData {
    city: String,
    temperature: f32,
    humidity: f32,
    condition: String,
    wind_speed: f32,
}

/// Simulates a database lookup that often fails due to network issues
#[unwrap_or_ai_func]
fn fetch_user_from_database(user_id: u32) -> Result<User, String> {
    // Simulate intermittent database failures
    if user_id == 404 {
        Err("Database connection timeout".to_string())
    } else if user_id == 500 {
        Err("User not found in database".to_string())
    } else {
        // This would normally work, but let's simulate it failing
        Err(format!("Network error while fetching user {}", user_id))
    }
}

/// Simulates a payment processing system that might fail
#[unwrap_or_ai_func]
fn process_payment(amount: f64, user_id: u32) -> Result<PaymentResult, String> {
    // Simulate payment processing failures
    if amount > 10000.0 {
        Err("Payment amount exceeds daily limit".to_string())
    } else if user_id == 999 {
        Err("Invalid user account for payment processing".to_string())
    } else {
        Err("Payment gateway temporarily unavailable".to_string())
    }
}

/// Simulates fetching weather data that might return None
#[unwrap_or_ai_func]
fn get_weather_data(city: &str) -> Option<WeatherData> {
    // Simulate API failures by always returning None
    // In real scenarios, this might fail due to API limits, network issues, etc.
    println!("Weather API for {} is currently unavailable", city);
    None
}

/// Simulates getting user preferences that might not exist
#[unwrap_or_ai_func]
fn get_user_preferences(user_id: u32) -> Option<User> {
    // Simulate missing user preferences
    println!("No cached preferences found for user {}", user_id);
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("=== E-Commerce Platform Demo with AI Error Recovery ===\n");

    // Example 1: User lookup with AI recovery
    println!("🔍 Example 1: Fetching user profile...");
    let user_result = unwrap_or_ai!(fetch_user_from_database(12345)).await;

    match user_result {
        Ok(user) => {
            println!("✅ Successfully retrieved user: {:?}", user);
            println!("   User ID: {}", user.id);
            println!("   Name: {}", user.name);
            println!("   Email: {}", user.email);
            println!("   Department: {}", user.department);
        }
        Err(e) => {
            println!("❌ Failed to retrieve user: {}", e);
        }
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 2: Payment processing with AI recovery
    println!("💳 Example 2: Processing payment...");
    let payment_result = unwrap_or_ai!(process_payment(599.99, 12345)).await;

    match payment_result {
        Ok(payment) => {
            println!("✅ Payment processed successfully: {:?}", payment);
            println!("   Transaction ID: {}", payment.transaction_id);
            println!("   Amount: ${} {}", payment.amount, payment.currency);
            println!("   Status: {}", payment.status);
        }
        Err(e) => {
            println!("❌ Payment processing failed: {}", e);
        }
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 3: Weather data with AI recovery for None values
    println!("🌤️  Example 3: Fetching weather data...");
    let weather_result = unwrap_or_ai!(get_weather_data("San Francisco")).await;

    match weather_result {
        Some(weather) => {
            println!("✅ Weather data retrieved: {:?}", weather);
            println!("   City: {}", weather.city);
            println!("   Temperature: {}°F", weather.temperature);
            println!("   Condition: {}", weather.condition);
            println!("   Humidity: {}%", weather.humidity);
            println!("   Wind Speed: {} mph", weather.wind_speed);
        }
        None => {
            println!("❌ No weather data available");
        }
    }

    println!("\n{}\n", "=".repeat(60));

    // Example 4: User preferences with AI recovery
    println!("⚙️  Example 4: Loading user preferences...");
    let prefs_result = unwrap_or_ai!(get_user_preferences(67890)).await;

    match prefs_result {
        Some(user) => {
            println!("✅ User preferences loaded: {:?}", user);
            println!("   User ID: {}", user.id);
            println!("   Name: {}", user.name);
            println!("   Preferred Department: {}", user.department);
        }
        None => {
            println!("❌ No user preferences found");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!(
        "🎉 Demo completed! AI error recovery provided fallback data for all failed operations."
    );
    println!(
        "💡 In a real application, this would prevent crashes and provide graceful degradation."
    );

    Ok(())
}
