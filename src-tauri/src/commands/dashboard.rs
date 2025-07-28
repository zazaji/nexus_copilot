// src-tauri/src/commands/dashboard.rs
use crate::{
    database::{models, queries},
    error::Result,
    state::AppState,
};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;
use walkdir::WalkDir;

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    notes_count: i64,
    vectors_count: i64,
    conversations_count: i64,
    tools_count: i64,
    db_size: u64,
    vector_db_size: u64,
    backend_db_size: u64,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct BackendDashboardStats {
    vectors_count: i64,
    vector_db_size: u64,
    backend_db_size: u64,
}

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats> {
    // --- Tauri-side stats ---
    let tauri_db_path = state.context.app_data_dir.join("nexus.sqlite");
    let db_size = fs::metadata(tauri_db_path).map(|m| m.len()).unwrap_or(0);

    let (conversations_count, tools_count, backend_url, indexed_dirs) = {
        let conn = state.db.lock().unwrap();
        let settings = queries::get_settings(&conn)?;
        let conversations_count = conn.query_row("SELECT COUNT(*) FROM conversations", [], |row| row.get(0))?;
        let tools_count = conn.query_row("SELECT COUNT(*) FROM configured_tools", [], |row| row.get(0))?;
        (conversations_count, tools_count, settings.execution.backend_url, settings.knowledge_base.indexed_directories)
    };

    let notes_count = indexed_dirs.iter().fold(0, |acc, dir| {
        acc + WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
            .count() as i64
    });

    // --- Backend-side stats ---
    let backend_stats_url = format!("{}/api/v1/dashboard/stats", backend_url);
    let backend_stats = match state.http_client.get(&backend_stats_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                response.json::<BackendDashboardStats>().await.unwrap_or_default()
            } else {
                log::error!("Backend stats endpoint returned error: {}", response.status());
                BackendDashboardStats::default()
            }
        },
        Err(e) => {
            log::error!("Failed to fetch backend stats: {}", e);
            BackendDashboardStats::default()
        }
    };

    // --- Combine stats ---
    Ok(DashboardStats {
        notes_count,
        vectors_count: backend_stats.vectors_count,
        conversations_count,
        tools_count,
        db_size,
        vector_db_size: backend_stats.vector_db_size,
        backend_db_size: backend_stats.backend_db_size,
    })
}

#[tauri::command]
pub async fn get_api_call_stats(state: State<'_, AppState>, time_range: String) -> Result<Vec<models::ApiCallStatPeriod>> {
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let url = format!("{}/api/v1/dashboard/stats/api-calls?time_range={}", backend_url, time_range);

    let response = state.http_client.get(url).send().await?.error_for_status()?;
    let stats = response.json::<Vec<models::ApiCallStatPeriod>>().await?;
    Ok(stats)
}