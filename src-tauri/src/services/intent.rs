// src-tauri/src/services/intent.rs
use crate::{
    database::{models, queries},
    error::Result,
    state::AppState,
};
use tauri::State;

pub async fn get_suggestions_for_context(state: State<'_, AppState>, context: models::IntentContext) -> Result<Vec<models::IntentSuggestion>> {
    log::info!("Getting suggestions for context type: {:?}", context.content_type);
    let mut suggestions = Vec::new();

    suggestions.push(models::IntentSuggestion {
        action: "ask_ai".to_string(),
        label: "Ask AI".to_string(),
        icon: "MessageSquare".to_string(),
    });

    if context.content_type == "text" {
        suggestions.push(models::IntentSuggestion {
            action: "built_in::execute".to_string(),
            label: "Execute".to_string(),
            icon: "Play".to_string(),
        });
        suggestions.push(models::IntentSuggestion {
            action: "built_in::translate".to_string(),
            label: "Translate".to_string(),
            icon: "Languages".to_string(),
        });
        suggestions.push(models::IntentSuggestion {
            action: "built_in::find_file".to_string(),
            label: "Find File".to_string(),
            icon: "FileSearch".to_string(),
        });
    }

    let conn = state.db.lock().unwrap();
    if let Ok(copilot_tools) = queries::get_copilot_tools(&conn) {
        for tool in copilot_tools {
            suggestions.push(models::IntentSuggestion {
                action: tool.id,
                label: tool.name,
                icon: "Wrench".to_string(),
            });
        }
    }

    suggestions.push(models::IntentSuggestion {
        action: "built_in::save_to_kb".to_string(),
        label: "Save to KB".to_string(),
        icon: "BookPlus".to_string(),
    });

    suggestions.push(models::IntentSuggestion {
        action: "built_in::view_clipboard_history".to_string(),
        label: "View Clipboard".to_string(),
        icon: "ClipboardList".to_string(),
    });

    Ok(suggestions)
}