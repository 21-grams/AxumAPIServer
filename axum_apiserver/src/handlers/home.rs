use axum::{
    extract::{Request, State},
    response::Html,
};
use std::sync::Arc;

use crate::config::AppState;
use crate::error::Result;
use crate::templates::context::TemplateContext;
use super::utils::extract_language;

/// Handler for the homepage
pub async fn index(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<Html<String>> {
    // Get the current language
    let lang = extract_language(&request);
    
    // Get language data
    let i18n = state.language_manager.get_language(&lang)
        .unwrap_or_else(|| state.language_manager.get_default_language());
    
    // Build the template context
    let context = TemplateContext::new()
        .insert("lang", lang)
        .insert("title", i18n.get("home.title").unwrap_or("Welcome"))
        .insert("nav", i18n.get_section("nav"))
        .insert("footer", i18n.get_section("footer"))
        .insert("content", i18n.get_section("home"))
        .insert("available_languages", state.language_manager.get_available_languages());
    
    // Render the template
    let html = state.template_engine.render("home.html", &context)?;
    
    Ok(Html(html))
}