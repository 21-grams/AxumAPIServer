use minijinja::Environment;
use serde::Serialize;
use std::sync::Arc;

use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct TemplateEngine {
    env: Arc<Environment<'static>>,
}

impl TemplateEngine {
    /// Create a new template engine with templates loaded from the templates directory
    pub fn new() -> Result<Self> {
        let mut env = Environment::new();
        
        // Set up template loader to load from the templates directory
        env.set_loader(minijinja::path_loader("../axum_apiserver/src/templates"));
        
        // Add any custom filters here if needed
        // env.add_filter("custom_filter", |value: String| { ... });
        
        Ok(Self {
            env: Arc::new(env),
        })
    }
    
    /// Render a template with the given context
    pub fn render<S: Serialize>(&self, template_name: &str, context: &S) -> Result<String> {
        let template = self.env
            .get_template(template_name)
            .map_err(|e| AppError::Template(format!("Failed to load template '{}': {}", template_name, e)))?;
            
        template.render(context)
            .map_err(|e| AppError::Template(format!("Failed to render template '{}': {}", template_name, e)))
    }
}