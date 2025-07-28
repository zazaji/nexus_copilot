// src-tauri/src/services/chat/mod.rs
pub mod llm_utils;
pub mod message_handler;

pub use llm_utils::generate_title_for_conversation;
pub use message_handler::handle_message;