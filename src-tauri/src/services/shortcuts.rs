// src-tauri/src/services/shortcuts.rs
use crate::database::models::Settings;
use tauri::{AppHandle, GlobalShortcutManager};

pub fn update_global_shortcuts(app_handle: &AppHandle, settings: &Settings) -> tauri::Result<()> {
    let mut manager = app_handle.global_shortcut_manager();
    
    manager.unregister_all()?;
    log::info!("Unregistered all previous global shortcuts.");

    let toggle_copilot_handle = app_handle.clone();
    let toggle_copilot_shortcut = settings.shortcuts.toggle_copilot.clone();
    if !toggle_copilot_shortcut.is_empty() {
        manager.register(&toggle_copilot_shortcut, move || {
            crate::services::window_manager::toggle_copilot_window(&toggle_copilot_handle);
        })?;
        log::info!("Registered 'Toggle Copilot' shortcut: {}", &toggle_copilot_shortcut);
    }

    let show_main_handle = app_handle.clone();
    let show_main_shortcut = settings.shortcuts.show_main_window.clone();
    if !show_main_shortcut.is_empty() {
        manager.register(&show_main_shortcut, move || {
            crate::services::window_manager::show_main_window(&show_main_handle);
        })?;
        log::info!("Registered 'Show Main Window' shortcut: {}", &show_main_shortcut);
    }

    Ok(())
}