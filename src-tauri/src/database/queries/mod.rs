// src-tauri/src/database/queries/mod.rs
// Declare all sub-modules within the queries directory
mod settings_queries;
mod chat_queries;
mod tool_queries;
mod note_queries;
mod clipboard_queries;
mod agent_queries;
mod online_kb_queries;

// Re-export all public functions from the sub-modules
pub use settings_queries::*;
pub use chat_queries::*;
pub use tool_queries::*;
pub use clipboard_queries::*;
pub use agent_queries::*;
pub use online_kb_queries::*;