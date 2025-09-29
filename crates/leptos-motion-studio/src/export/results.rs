//! Export results and related structures

use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Export result containing generated content
#[derive(Debug, Clone)]
pub struct ExportResult {
    /// Generated content
    pub content: String,
    /// MIME type of content
    pub mime_type: String,
    /// Recommended file extension
    pub file_extension: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ExportResult {
    /// Create new export result
    pub fn new(content: String, mime_type: String, file_extension: String) -> Self {
        Self {
            content,
            mime_type,
            file_extension,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl Default for ExportResult {
    fn default() -> Self {
        Self {
            content: String::new(),
            mime_type: "text/plain".to_string(),
            file_extension: "txt".to_string(),
            metadata: HashMap::new(),
        }
    }
}
