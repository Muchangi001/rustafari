mod routes;
mod errors;
mod graph;

use routes::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Initialize logging only when NOT running in Shuttle
    if std::env::var("SHUTTLE").is_err() {
        // This will only initialize if no subscriber exists
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    }

    let app = routes();
    
    tracing::info!("🦀 Rustafari community server started");
    
    Ok(app.into())
}