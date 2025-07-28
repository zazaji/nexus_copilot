// src-tauri/src/services/window_manager.rs
use tauri::{AppHandle, Manager};

pub fn toggle_copilot_window(app: &AppHandle) {
    if let Some(window) = app.get_window("copilot") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            if let Err(e) = window.hide() {
                log::error!("Failed to hide copilot window: {}", e);
            }
        } else {
            let app_handle_clone = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::commands::system::show_window("copilot", app_handle_clone, true).await {
                    log::error!("Failed to show copilot window from toggle: {}", e);
                }
            });
        }
    } else {
        log::error!("Toggle failed: Window with label 'copilot' not found. This indicates a startup issue.");
    }
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        if let Err(e) = window.show() {
            log::error!("Failed to show main window: {}", e);
        }
        if let Err(e) = window.set_focus() {
            log::error!("Failed to focus main window: {}", e);
        }
    }
}