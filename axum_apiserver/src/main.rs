use std::sync::Arc;

use axum::Router;
use axum::handler::HandlerWithoutStateExt;
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;
mod i18n;
mod middleware;
mod routes;
mod templates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize template engine
    let template_engine = templates::engine::TemplateEngine::new()?;
    
    // Initialize language manager
    let language_manager = i18n::language::LanguageManager::new()?;
    
    // Create shared application state
    let app_state = Arc::new(config::AppState {
        template_engine,
        language_manager,
    });

    // Build the application with routes
    let app = routes::create_routes(app_state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    
    tracing::info!("Server started at http://{}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}