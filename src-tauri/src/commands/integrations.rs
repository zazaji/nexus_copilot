use crate::{
    database::{models, queries},
    error::Result,
    state::AppState,
};
use tauri::State;

#[tauri::command]
pub fn list_integrations(state: State<'_, AppState>) -> Result<Vec<models::Integration>> {
    let conn = state.db.lock().unwrap();
    queries::list_integrations(&conn)
}

#[tauri::command]
pub fn add_integration(state: State<'_, AppState>, integration: models::Integration) -> Result<models::Integration> {
    let conn = state.db.lock().unwrap();
    queries::add_integration(&conn, &integration)?;
    Ok(integration)
}

#[tauri::command]
pub fn delete_integration(state: State<'_, AppState>, id: String) -> Result<()> {
    let conn = state.db.lock().unwrap();
    queries::delete_integration(&conn, &id)
}

#[tauri::command]
pub async fn get_integration_templates(state: State<'_, AppState>) -> Result<Vec<models::IntegrationTemplate>> {
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let url = format!("{}/api/v1/integrations/templates", backend_url);

    let response = state.http_client.get(url).send().await?.error_for_status()?;
    let templates = response.json::<Vec<models::IntegrationTemplate>>().await?;
    Ok(templates)
}