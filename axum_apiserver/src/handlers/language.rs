use axum::{
    extract::{Query, State},
    http::header,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::{AppState, DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES};
use crate::error::Result;
use crate::templates::context::TemplateContext;

#[derive(Deserialize)]
pub struct LanguageQuery {
    lang: String,
}

/// Handler for changing the current language
pub async fn change_language(
    Query(query): Query<LanguageQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    // Validate the language
    let lang = if SUPPORTED_LANGUAGES.contains(&query.lang.as_str()) {
        query.lang
    } else {
        DEFAULT_LANGUAGE.to_string()
    };
    
    // Get language data
    let i18n = state.language_manager.get_language(&lang)
        .unwrap_or_else(|| state.language_manager.get_default_language());
    
    // Build the template context
    let context = TemplateContext::new()
        .insert("lang", &lang)
        .insert("title", i18n.get("home.title").unwrap_or("Welcome"))
        .insert("nav", i18n.get_section("nav"))
        .insert("footer", i18n.get_section("footer"))
        .insert("content", i18n.get_section("home"))
        .insert("available_languages", state.language_manager.get_available_languages());
    
    // Render the template
    let html = state.template_engine.render("home.html", &context)?;
    
    // Set the language cookie
    let cookie = format!("lang={}; Path=/; Max-Age=31536000; SameSite=Lax", lang);
    
    // Return the response with the cookie
    Ok((
        [(header::SET_COOKIE, cookie)],
        html,
    ))
}