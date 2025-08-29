use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use unwrap_or_ai::unwrap_or_ai;
use unwrap_or_ai_proc_macro::unwrap_or_ai_func;

/// Represents configuration data for a microservice
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ServiceConfig {
    service_name: String,
    port: u16,
    database_url: String,
    api_key: String,
    timeout_seconds: u32,
    max_connections: u32,
}

/// Represents analysis results from log processing
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct LogAnalysis {
    total_requests: u32,
    error_count: u32,
    average_response_time_ms: f64,
    top_error_message: String,
    peak_hour: String,
    status: String,
}

/// Represents health check data for a service
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct HealthCheck {
    service_name: String,
    status: String,
    uptime_hours: f64,
    memory_usage_mb: f64,
    cpu_usage_percent: f64,
    last_restart: String,
    dependencies_healthy: bool,
}

/// Represents deployment information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct DeploymentInfo {
    environment: String,
    version: String,
    deployment_id: String,
    status: String,
    deployed_at: String,
    rollback_available: bool,
}

/// Simulates loading configuration from a remote config service
#[unwrap_or_ai_func]
fn load_service_config(service_name: &str) -> Result<ServiceConfig, String> {
    // Simulate config service being down
    Err(format!(
        "Config service unreachable for service: {}",
        service_name
    ))
}

/// Simulates analyzing log files that might be corrupted or missing
#[unwrap_or_ai_func]
fn analyze_service_logs(service_name: &str, hours: u32) -> Result<LogAnalysis, String> {
    // Simulate log analysis failures
    match service_name {
        "auth-service" => Err("Log files corrupted during disk failure".to_string()),
        "payment-service" => Err("Insufficient permissions to access log directory".to_string()),
        _ => Err(format!(
            "Log analysis timeout for {} ({}h)",
            service_name, hours
        )),
    }
}

/// Simulates health check that might timeout
#[unwrap_or_ai_func]
fn check_service_health(service_name: &str) -> Option<HealthCheck> {
    // Simulate health check endpoints being unreachable
    println!("Health check endpoint for {} timed out", service_name);
    None
}

/// Simulates querying deployment status that might fail
#[unwrap_or_ai_func]
fn get_deployment_status(environment: &str) -> Option<DeploymentInfo> {
    // Simulate CI/CD system being temporarily unavailable
    println!("CI/CD system unavailable for environment: {}", environment);
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("=== DevOps Monitoring Dashboard with AI Error Recovery ===\n");

    // Example 1: Loading critical service configuration
    println!("⚙️  Example 1: Loading service configuration...");
    let config = unwrap_or_ai!(load_service_config("user-management-service")).await;

    println!("✅ Service configuration loaded: {:?}", config);
    println!("   Service: {}", config.service_name);
    println!("   Port: {}", config.port);
    println!("   Database: {}", config.database_url);
    println!("   Timeout: {}s", config.timeout_seconds);
    println!("   Max Connections: {}", config.max_connections);

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Analyzing service logs for troubleshooting
    println!("📊 Example 2: Analyzing service logs...");
    let analysis = unwrap_or_ai!(analyze_service_logs("auth-service", 24)).await;

    println!("✅ Log analysis completed: {:?}", analysis);
    println!("   Total Requests: {}", analysis.total_requests);
    println!("   Error Count: {}", analysis.error_count);
    println!(
        "   Avg Response Time: {:.2}ms",
        analysis.average_response_time_ms
    );
    println!("   Top Error: {}", analysis.top_error_message);
    println!("   Peak Hour: {}", analysis.peak_hour);
    println!("   Status: {}", analysis.status);

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Checking service health status
    println!("🏥 Example 3: Checking service health...");
    let health = unwrap_or_ai!(check_service_health("payment-gateway")).await;

    println!("✅ Health check successful: {:?}", health);
    println!("   Service: {}", health.service_name);
    println!("   Status: {}", health.status);
    println!("   Uptime: {:.1} hours", health.uptime_hours);
    println!("   Memory Usage: {:.1} MB", health.memory_usage_mb);
    println!("   CPU Usage: {:.1}%", health.cpu_usage_percent);
    println!("   Dependencies Healthy: {}", health.dependencies_healthy);

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Getting deployment status
    println!("🚀 Example 4: Checking deployment status...");
    let deployment = unwrap_or_ai!(get_deployment_status("production")).await;

    println!("✅ Deployment status retrieved: {:?}", deployment);
    println!("   Environment: {}", deployment.environment);
    println!("   Version: {}", deployment.version);
    println!("   Deployment ID: {}", deployment.deployment_id);
    println!("   Status: {}", deployment.status);
    println!("   Deployed At: {}", deployment.deployed_at);
    println!("   Rollback Available: {}", deployment.rollback_available);

    println!("\n{}", "=".repeat(70));
    println!("🎉 DevOps monitoring completed with AI-powered error recovery!");
    println!("💡 All critical system information was provided despite service failures.");
    println!("🔧 This enables continuous monitoring even when individual services are down.");

    Ok(())
}
