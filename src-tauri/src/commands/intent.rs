use crate::{
    database::models,
    error::Result,
    services,
    state::AppState,
};
use tauri::State;

#[tauri::command]
pub async fn get_intent_suggestions(
    state: State<'_, AppState>,
    context: models::IntentContext,
) -> Result<Vec<models::IntentSuggestion>> {
    log::info!("Getting suggestions for context: {:?}", context);
    services::intent::get_suggestions_for_context(state, context).await
}