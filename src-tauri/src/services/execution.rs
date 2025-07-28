use crate::{
    database::queries,
    error::{AppError, Result},
    state::AppState,
};
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use reqwest_eventsource::{Event, EventSource};
use serde::Serialize;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    sync::Arc,
};
use tauri::{api::path::home_dir, AppHandle, Manager};
use tokio::sync::Mutex;

const BACKEND_REPO_URL: &str = "https://github.com/zazaji/nexus_copilot_fastapi.git";
static BACKEND_PROCESS: Lazy<Arc<Mutex<Option<tokio::process::Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Serialize, Clone)]
struct ProgressPayload {
    progress: f32,
    message: String,
}

fn get_backend_dir() -> Result<PathBuf> {
    home_dir()
        .ok_or_else(|| AppError::Internal("Could not determine home directory.".to_string()))
        .map(|p| p.join("nexus_copilot/backend"))
}

fn emit_progress(app_handle: &AppHandle, event: &str, progress: f32, message: &str) {
    if let Err(e) = app_handle.emit_all(
        event,
        ProgressPayload {
            progress,
            message: message.to_string(),
        },
    ) {
        log::error!("Failed to emit progress: {}", e);
    }
}

pub async fn check_status(
    _app_handle: &AppHandle,
    state: &AppState,
    _python_path: &str,
) -> Result<String> {
    let backend_dir = get_backend_dir()?;
    if !backend_dir.exists() || !backend_dir.join("venv").exists() {
        return Ok("not_installed".to_string());
    }

    let mut process_guard = BACKEND_PROCESS.lock().await;
    if let Some(child) = process_guard.as_mut() {
        match child.try_wait() {
            Ok(Some(_)) => { // Process has exited
                *process_guard = None;
            },
            Ok(None) => { // Process is still running
                 let settings = queries::get_settings(&state.db.lock().unwrap())?;
                 let health_url = format!("{}/", settings.execution.backend_url);
                 if reqwest::get(&health_url).await.is_ok() {
                     return Ok("running".to_string());
                 }
            },
            Err(_) => { // Error checking status, assume it's stopped
                *process_guard = None;
            }
        }
    }

    Ok("stopped".to_string())
}

pub async fn install_service(app_handle: &AppHandle, state: &AppState) -> Result<()> {
    let backend_dir = get_backend_dir()?;
    fs::create_dir_all(backend_dir.parent().unwrap())?;

    emit_progress(app_handle, "backend-install-progress", 10.0, "Cloning backend repository...");
    Command::new("git")
        .args(["clone", BACKEND_REPO_URL, &backend_dir.to_string_lossy()])
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| AppError::Internal("Failed to clone git repository.".to_string()))?;

    let python_path = queries::get_settings(&state.db.lock().unwrap())?
        .execution
        .python_path;
    let python_cmd = if python_path.is_empty() { "python" } else { &python_path };

    emit_progress(app_handle, "backend-install-progress", 40.0, "Creating Python virtual environment...");
    Command::new(python_cmd)
        .args(["-m", "venv", "venv"])
        .current_dir(&backend_dir)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| AppError::Internal("Failed to create virtual environment.".to_string()))?;

    let pip_path = if cfg!(target_os = "windows") {
        backend_dir.join("venv/Scripts/pip")
    } else {
        backend_dir.join("venv/bin/pip")
    };

    emit_progress(app_handle, "backend-install-progress", 70.0, "Installing dependencies...");
    Command::new(pip_path)
        .args(["install", "-r", "requirements.txt"])
        .current_dir(&backend_dir)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| AppError::Internal("Failed to install dependencies.".to_string()))?;

    emit_progress(app_handle, "backend-install-progress", 100.0, "Installation complete!");
    Ok(())
}

pub async fn start_service(
    _app_handle: &AppHandle,
    state: &AppState,
    _python_path: &str,
) -> Result<()> {
    let backend_dir = get_backend_dir()?;
    let uvicorn_path = if cfg!(target_os = "windows") {
        backend_dir.join("venv/Scripts/uvicorn.exe")
    } else {
        backend_dir.join("venv/bin/uvicorn")
    };

    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let url = url::Url::parse(&settings.execution.backend_url)?;
    let port = url.port().unwrap_or(8008).to_string();

    let child = tokio::process::Command::new(uvicorn_path)
        .args(["app.main:app", "--host", "0.0.0.0", "--port", &port])
        .current_dir(&backend_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let mut process_guard = BACKEND_PROCESS.lock().await;
    *process_guard = Some(child);
    log::info!("Backend service started.");
    Ok(())
}

pub async fn stop_service(_state: &AppState) -> Result<()> {
    let mut process_guard = BACKEND_PROCESS.lock().await;
    if let Some(child) = process_guard.as_mut() {
        child.kill().await?;
        log::info!("Backend service stopped.");
    }
    *process_guard = None;
    Ok(())
}

pub async fn batch_convert_to_markdown(app_handle: AppHandle, state: AppState, input_dir: String, output_dir: String) -> Result<()> {
    let backend_url = queries::get_settings(&state.db.lock().unwrap())?.execution.backend_url;
    let url = format!("{}/api/v1/convert/to-markdown", backend_url);

    let request_body = serde_json::json!({
        "input_dir": input_dir,
        "output_dir": output_dir,
    });

    let request_builder = state.http_client.post(url).json(&request_body);
    let mut es = EventSource::new(request_builder)?;

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Message(message)) => {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&message.data) {
                    app_handle.emit_all("conversion-progress", payload)?;
                }
            }
            Err(e) => {
                log::error!("Error during conversion stream: {}", e);
                es.close();
                break;
            }
            _ => {}
        }
    }
    Ok(())
}