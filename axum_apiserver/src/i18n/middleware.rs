use axum::{
    extract::{Request, State},
    http::StatusCode, 
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::config::{AppState, DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES};

/// Information about the language for the current request
#[derive(Clone, Debug)]
pub struct LanguageInfo {
    pub lang: String,
}

/// Middleware to detect the language for the current request
pub async fn language_detector(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract language from cookie if present
    let lang = if let Some(cookie_header) = request.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            extract_lang_from_cookie(cookie_str)
                .unwrap_or_else(|| extract_lang_from_header(&request))
        } else {
            extract_lang_from_header(&request)
        }
    } else {
        extract_lang_from_header(&request)
    };
    
    // Check if the language is supported
    let lang = if SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
        lang
    } else {
        DEFAULT_LANGUAGE.to_string()
    };
    
    // Make sure the language is loaded
    if state.language_manager.get_language(&lang).is_none() {
        tracing::warn!("Language '{}' not found, falling back to default", lang);
    }
    
    // Insert language info into the request extensions
    request.extensions_mut().insert(LanguageInfo { lang });
    
    // Continue processing the request
    Ok(next.run(request).await)
}

/// Extract language from the Cookie header
fn extract_lang_from_cookie(cookie_str: &str) -> Option<String> {
    cookie_str.split(';')
        .find_map(|pair| {
            let mut parts = pair.trim().split('=');
            if parts.next() == Some("lang") {
                parts.next().map(|v| v.to_string())
            } else {
                None
            }
        })
}

/// Extract language from Accept-Language header, falling back to default
fn extract_lang_from_header(request: &Request) -> String {
    if let Some(accept_lang) = request.headers().get("Accept-Language") {
        if let Ok(accept_lang_str) = accept_lang.to_str() {
            return accept_lang_str
                .split(',')
                .next()
                .and_then(|lang| lang.split(';').next())
                .unwrap_or(DEFAULT_LANGUAGE)
                .split('-')
                .next()
                .unwrap_or(DEFAULT_LANGUAGE)
                .to_string();
        }
    }
    
    DEFAULT_LANGUAGE.to_string()
}