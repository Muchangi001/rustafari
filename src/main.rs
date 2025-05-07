mod graph;
mod routes;
mod errors;

use routes::routes;
use tracing_subscriber;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Initialize better logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let app = routes();
    
    // Bind to the specified address
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("🦀 Rustafari community server running at http://127.0.0.1:3000");
    
    // Start the server
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// Graceful shutdown handler
async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    
    tracing::info!("Shutdown signal received, stopping server gracefully...");
}