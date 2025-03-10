use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use std::sync::Arc;

use crate::config::SUPPORTED_LANGUAGES;
use crate::error::{AppError, Result};

/// Represents translations for a specific language
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    strings: HashMap<String, String>,
}

impl Language {
    /// Get a translated string by key
    pub fn get(&self, key: &str) -> Option<&str> {
        self.strings.get(key).map(|s| s.as_str())
    }
    
    /// Get all translations with a common prefix as a new HashMap
    /// This is useful for getting all translations for a section of the site
    pub fn get_section(&self, prefix: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let prefix_with_dot = format!("{}.", prefix);
        
        for (key, value) in self.strings.iter() {
            if key.starts_with(&prefix_with_dot) {
                let short_key = key[prefix_with_dot.len()..].to_string();
                result.insert(short_key, value.clone());
            }
        }
        
        result
    }
}

/// Manages all languages for the application
#[derive(Clone)]
pub struct LanguageManager {
    languages: HashMap<String, Arc<Language>>,
    default_lang: String,
}

impl LanguageManager {
    /// Create a new language manager by loading languages from the lang.json file
    pub fn new() -> Result<Self> {
        let content = fs::read_to_string("i18n/lang.json")
            .map_err(|e| AppError::Language(format!("Failed to read lang.json: {}", e)))?;
            
        let languages: HashMap<String, Language> = serde_json::from_str(&content)
            .map_err(|e| AppError::Language(format!("Failed to parse lang.json: {}", e)))?;
            
        // Validate that all supported languages are present
        for lang in SUPPORTED_LANGUAGES {
            if !languages.contains_key(lang) {
                tracing::warn!("Supported language '{}' not found in lang.json", lang);
            }
        }
            
        let default_lang = languages.keys().next()
            .ok_or_else(|| AppError::Language("No languages found in lang.json".to_string()))?
            .clone();
            
        // Convert to Arc for better performance with cloning
        let languages = languages.into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();
            
        Ok(Self {
            languages,
            default_lang,
        })
    }
    
    /// Get a language by its code, falling back to the default language if not found
    pub fn get_language(&self, lang: &str) -> Option<Arc<Language>> {
        self.languages.get(lang).cloned().or_else(|| self.languages.get(&self.default_lang).cloned())
    }
    
    /// Get the default language
    pub fn get_default_language(&self) -> Arc<Language> {
        self.languages.get(&self.default_lang).cloned().unwrap()
    }
    
    /// Get a list of all available languages
    pub fn get_available_languages(&self) -> Vec<String> {
        self.languages.keys().cloned().collect()
    }
}