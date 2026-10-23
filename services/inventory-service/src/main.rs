// services/inventory-service/src/main.rs

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/health", get(health_check));

    // Define the address our microservice will listen on
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .unwrap();
    
    println!("Inventory Service running on http://127.0.0.1:8081");

    // Run the server using Axum's tcp listener integration
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Inventory Service is healthy and operational!"
}