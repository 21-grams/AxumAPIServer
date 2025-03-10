use axum::{
    extract::{Path, Request, State},
    response::Html,
};
use std::sync::Arc;

use crate::config::AppState;
use crate::error::{AppError, Result};
use super::utils::extract_language;

/// Handler for loading feature details via HTMX
pub async fn get_feature(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Html<String>> {
    // Get the current language
    let lang = extract_language(&request);
    
    // Get language data
    let i18n = state.language_manager.get_language(&lang)
        .unwrap_or_else(|| state.language_manager.get_default_language());
    
    // Get the content section
    let content = i18n.get_section("home");
    
    // Generate the HTML based on the feature ID
    let html = match id.as_str() {
        "1" => format!(
            "<h3>{}</h3><p>{}</p>",
            content.get("feature1_title").unwrap_or(&"Feature 1".to_string()),
            content.get("feature1_desc").unwrap_or(&"Description of feature 1".to_string())
        ),
        "2" => format!(
            "<h3>{}</h3><p>{}</p>",
            content.get("feature2_title").unwrap_or(&"Feature 2".to_string()),
            content.get("feature2_desc").unwrap_or(&"Description of feature 2".to_string())
        ),
        "3" => format!(
            "<h3>{}</h3><p>{}</p>",
            content.get("feature3_title").unwrap_or(&"Feature 3".to_string()),
            content.get("feature3_desc").unwrap_or(&"Description of feature 3".to_string())
        ),
        _ => return Err(AppError::NotFound(format!("Feature '{}' not found", id))),
    };
    
    Ok(Html(html))
}