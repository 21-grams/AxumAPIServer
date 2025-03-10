use axum::{
    Router,
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
};
use tower_http::services::ServeDir;
use std::sync::Arc;

use crate::config::AppState;
use crate::handlers;
use crate::middleware;

/// Create the application router with all routes
pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Core routes
        .route("/", get(handlers::home::index))
        .route("/change-language", get(handlers::language::change_language))
        
        // API routes for HTMX
        .route("/api/feature/:id", get(handlers::features::get_feature))
        
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        
        // Apply middleware
        .layer(axum::middleware::from_fn_with_state(state.clone(), middleware::language_detector))
        
        // Add state
        .with_state(state)
        
        // Fallback for 404s
        .fallback(handle_404.into_service())
}

/// Handler for 404 Not Found responses
async fn handle_404(uri: Uri) -> impl IntoResponse {
    tracing::warn!("Not found: {}", uri);
    (StatusCode::NOT_FOUND, format!("Not found: {}", uri))
}