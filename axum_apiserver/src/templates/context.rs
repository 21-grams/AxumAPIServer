use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;

/// Template context builder for rendering templates
#[derive(Serialize, Default)]
pub struct TemplateContext {
    #[serde(flatten)]
    values: HashMap<String, Value>,
}

impl TemplateContext {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Insert a value into the template context
    pub fn insert<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.values.insert(key.into(), value);
        self
    }
    
    /// Merge another context into this one
    pub fn merge(mut self, other: TemplateContext) -> Self {
        for (key, value) in other.values {
            self.values.insert(key, value);
        }
        self
    }
}