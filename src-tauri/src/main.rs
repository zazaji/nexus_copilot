// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod database;
mod error;
mod knowledge_base;
mod services;
mod state;
mod system;

use state::AppStateInner;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use system::context::AppContext;
use tauri::{
    App, CustomMenuItem, Manager, Menu, MenuItem, Submenu,
    SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent,
};
use crate::database::queries;
use tauri_plugin_autostart::MacosLauncher;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("VUE_DEVTOOLS_PLUGINS_DIR", ".vue-devtools/plugins");

    env_logger::init();

    let show_main = CustomMenuItem::new("show_main".to_string(), "Open Main Window");
    let toggle_copilot = CustomMenuItem::new("toggle_copilot".to_string(), "Toggle Copilot");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Nexus");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show_main)
        .add_item(toggle_copilot)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let app_menu = Submenu::new(
        "App",
        Menu::new()
            .add_native_item(MenuItem::About(
                "Nexus".to_string(),
                tauri::AboutMetadata::default(),
            ))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    let file_menu = Submenu::new("File", Menu::new());
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(edit_menu);

    let app_result = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => std::process::exit(0),
                    "toggle_copilot" => services::window_manager::toggle_copilot_window(app),
                    "show_main" => services::window_manager::show_main_window(app),
                    _ => {}
                },
                SystemTrayEvent::LeftClick { .. } => services::window_manager::toggle_copilot_window(app),
                _ => {}
            }
        })
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "main" {
                    api.prevent_close();
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .setup(|app: &mut App| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let handle = app.handle();

            let handle_clone = handle.clone();
            log::info!("Starting global clipboard monitor.");
            tauri::async_runtime::spawn(async move {
                system::clipboard::start_clipboard_monitor(handle_clone).await;
            });

            if let Some(main_window) = app.get_window("main") {
                main_window.set_focus()?;
            }

            let app_data_dir: PathBuf = handle
                .path_resolver()
                .app_data_dir()
                .expect("Failed to get app data dir.");
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir.");
            }

            env::set_var("NEXUS_DATA_PATH", &app_data_dir);
            log::info!("Set NEXUS_DATA_PATH to: {}", app_data_dir.display());

            let app_context = AppContext::new(app_data_dir.clone());
            fs::create_dir_all(&app_context.scripts_dir)?;
            fs::create_dir_all(&app_context.creations_dir)?;
            let db_path = app_data_dir.join("nexus.sqlite");
            log::info!("Database path set to: {}", db_path.display());

            let app_state = Arc::new(AppStateInner::new(&db_path, app_context)?);

            if let Ok(conn) = app_state.db.lock() {
                if let Err(e) = queries::delete_empty_conversations(&conn) {
                    log::error!("Failed to clean up empty conversations: {}", e);
                }
            }

            let settings = queries::get_settings(&app_state.db.lock().unwrap())?;
            services::shortcuts::update_global_shortcuts(&handle, &settings)
                .map_err(|e| anyhow::anyhow!("Failed to initialize shortcuts: {}", e))?;

            if settings.execution.auto_start_backend {
                let handle_clone = handle.clone();
                let state_clone = app_state.clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = services::execution::start_service(&handle_clone, &state_clone, &settings.execution.python_path).await {
                        log::error!("Failed to auto-start backend service: {}", e);
                    }
                });
            }

            app.manage(app_state);

            if let Some(copilot_window) = app.get_window("copilot") {
                copilot_window.hide()?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::show_window,
            commands::system::open_directory_picker,
            commands::system::get_active_window_info,
            commands::system::get_clipboard_content,
            commands::system::show_in_folder,
            commands::system::enable_autostart,
            commands::system::disable_autostart,
            commands::system::is_autostart_enabled,
            commands::system::paste_text,
            commands::settings::get_user_settings,
            commands::settings::update_user_settings,
            commands::knowledge_base::create_unique_file,
            commands::knowledge_base::rename_file,
            commands::knowledge_base::move_file,
            commands::knowledge_base::save_file_content,
            commands::knowledge_base::save_note_to_kb,
            commands::knowledge_base::read_file_content,
            commands::knowledge_base::list_files_in_directory,
            commands::knowledge_base::start_indexing,
            commands::knowledge_base::search_local_kb,
            commands::knowledge_base::search_online_kb,
            commands::knowledge_base::find_file_in_kb,
            commands::knowledge_base::remove_indexed_directory,
            commands::knowledge_base::clear_knowledge_base,
            commands::knowledge_base::rebuild_index,
            commands::knowledge_base::get_note_details,
            commands::knowledge_base::get_knowledge_graph_data,
            commands::knowledge_base::rebuild_knowledge_graph,
            commands::knowledge_base::delete_file,
            commands::knowledge_base::batch_convert_to_markdown,
            commands::knowledge_base::list_online_kbs,
            commands::knowledge_base::add_online_kb,
            commands::knowledge_base::update_online_kb,
            commands::knowledge_base::delete_online_kb,
            commands::chat::create_conversation,
            commands::chat::process_chat_message,
            commands::chat::stop_chat_generation,
            commands::chat::save_message,
            commands::chat::link_agent_task_to_message,
            commands::chat::delete_message,
            commands::chat::generate_title_for_conversation,
            commands::chat::get_conversation_history,
            commands::chat::list_conversations,
            commands::chat::get_conversation_with_artifacts,
            commands::chat::delete_conversation,
            commands::chat::clear_all_conversations,
            commands::chat::update_conversation_title,
            commands::chat::update_message_content,
            commands::chat::generate_artifact,
            commands::agent::process_agentic_instruction,
            commands::agent::stop_agent_task,
            commands::agent::restart_agent_task,
            commands::agent::resume_write_task,
            commands::agent::get_task_status,
            commands::agent::generate_research_node_content,
            commands::agent::refine_agent_task_section,
            commands::intent::get_intent_suggestions,
            commands::tools::list_tools,
            commands::tools::execute_tool,
            commands::tools::list_configured_tools,
            commands::tools::save_configured_tool,
            commands::tools::delete_configured_tool,
            commands::tools::execute_python_code,
            commands::tools::save_tool_script,
            commands::tools::execute_shell_command,
            commands::tools::execute_generic_code,
            commands::tools::setup_task_workspace,
            commands::tools::write_file_to_task_dir,
            commands::assets::save_temp_asset_with_hash,
            commands::assets::read_file_as_base64,
            commands::backup::export_data,
            commands::backup::import_data,
            commands::clipboard::get_clipboard_history,
            commands::clipboard::clear_clipboard_history,
            commands::clipboard::delete_clipboard_item,
            commands::clipboard::delete_clipboard_items,
            commands::clipboard::toggle_clipboard_item_pinned,
            commands::app_info::get_app_info,
            commands::app_info::get_readme_content,
            commands::dashboard::get_dashboard_stats,
            commands::dashboard::get_api_call_stats,
            commands::execution::check_backend_status,
            commands::execution::install_backend_service,
            commands::execution::start_backend_service,
            commands::execution::stop_backend_service
        ])
        .run(tauri::generate_context!());

    if let Err(e) = app_result {
        log::error!("Error while running Tauri application: {:?}", e);
    }
}