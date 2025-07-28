// src-tauri/src/knowledge_base/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeSource {
    pub id: String,
    pub file_path: String,
    pub source_name: String,
    pub content_snippet: String,
    pub score: f32,
}