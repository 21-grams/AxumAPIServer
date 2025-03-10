pub mod home;
pub mod features;
pub mod language;

/// Utility functions shared across handlers
pub mod utils {
    use axum::extract::{Request, State};
    use std::sync::Arc;

    use crate::config::AppState;
    use crate::i18n::middleware::LanguageInfo;

    /// Extract the current language from a request
    pub fn extract_language(request: &Request) -> String {
        request
            .extensions()
            .get::<LanguageInfo>()
            .map(|info| info.lang.clone())
            .unwrap_or_else(|| "en".to_string())
    }
}