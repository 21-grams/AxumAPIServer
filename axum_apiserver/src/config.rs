use crate::i18n::language::LanguageManager;
use crate::templates::engine::TemplateEngine;

/// Shared application state that will be available to all handlers
#[derive(Clone)]
pub struct AppState {
    pub template_engine: TemplateEngine,
    pub language_manager: LanguageManager,
}

/// Supported languages for the application
pub const SUPPORTED_LANGUAGES: [&str; 3] = ["en", "es", "fr"];
pub const DEFAULT_LANGUAGE: &str = "en";