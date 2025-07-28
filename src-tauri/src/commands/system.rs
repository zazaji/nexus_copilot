// src-tauri/src/commands/system.rs
use crate::error::{AppError, Result};
use enigo::{Enigo, Key, Keyboard, Settings as EnigoSettings, Direction};
use serde::Serialize;
use std::time::Duration;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_autostart::ManagerExt;

#[derive(Serialize)]
pub struct DirectoryPickerResponse {
    success: bool,
    path: Option<String>,
    error: Option<String>,
}

#[tauri::command]
pub async fn show_window(label: &str, app: AppHandle, focus: bool) -> Result<()> {
    let window = app.get_window(label).ok_or_else(|| {
        AppError::Internal(format!(
            "Window with label '{}' not found. Check tauri.conf.json.",
            label
        ))
    })?;

    log::info!("Showing window: {} with focus: {}", label, focus);

    if label == "copilot" {
        if !window.is_visible()? {
            if let Some(monitor) = window.current_monitor()? {
                let monitor_size = monitor.size();
                let window_width = monitor_size.width / 4;
                let window_height = monitor_size.height / 2;

                window.set_size(PhysicalSize::new(window_width, window_height))?;

                let new_pos_x = (monitor_size.width - window_width) as i32;
                let new_pos_y = ((monitor_size.height - window_height) / 2) as i32;

                window.set_position(PhysicalPosition::new(new_pos_x, new_pos_y))?;
            } else {
                log::error!("Could not get current monitor to position the copilot window.");
            }
        }
        
        if let Ok(Some(clipboard_content)) = crate::system::clipboard::read_clipboard(&app).await {
            app.emit_all("copilot-context-update", clipboard_content)?;
        }
    }

    if !window.is_visible()? {
        window.show()?;
    }
    
    if focus {
        window.set_focus()?;
    }
    
    if label == "copilot" {
        app.emit_all("copilot-shown", ())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn open_directory_picker() -> Result<DirectoryPickerResponse> {
    match crate::system::fs::pick_directory().await {
        Ok(path) => Ok(DirectoryPickerResponse {
            success: true,
            path,
            error: None,
        }),
        Err(e) => Ok(DirectoryPickerResponse {
            success: false,
            path: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn show_in_folder(path: String) -> Result<()> {
    crate::system::fs::show_in_folder(path)
}

#[derive(Serialize)]
pub struct WindowInfo {
    app_name: String,
    window_title: String,
}

#[tauri::command]
pub async fn get_active_window_info() -> Result<Option<WindowInfo>> {
    Ok(Some(WindowInfo {
        app_name: "Unknown".to_string(),
        window_title: "Unknown".to_string(),
    }))
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardPayload {
    pub content_type: String,
    pub content: serde_json::Value,
}

#[tauri::command]
pub async fn get_clipboard_content(app: AppHandle) -> Result<Option<ClipboardPayload>> {
    crate::system::clipboard::read_clipboard(&app).await
}

#[tauri::command]
pub async fn enable_autostart(app: AppHandle) -> Result<()> {
    app.autolaunch().enable().map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub async fn disable_autostart(app: AppHandle) -> Result<()> {
    app.autolaunch().disable().map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub async fn is_autostart_enabled(app: AppHandle) -> Result<bool> {
    app.autolaunch().is_enabled().map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub async fn paste_text(text: String) -> Result<()> {
    // Wait a moment for focus to shift back to the previous application.
    tokio::time::sleep(Duration::from_millis(200)).await;

    // We don't need to set the clipboard here, as the user is pasting what's already there.
    // The primary action is to simulate the paste shortcut.
    
    let mut enigo = Enigo::new(&EnigoSettings::default()).map_err(|e| AppError::Internal(e.to_string()))?;
    let modifier = if cfg!(target_os = "macos") { Key::Meta } else { Key::Control };
    
    enigo.key(modifier, Direction::Press)?;
    enigo.key(Key::Unicode('v'), Direction::Click)?;
    enigo.key(modifier, Direction::Release)?;
    
    Ok(())
}