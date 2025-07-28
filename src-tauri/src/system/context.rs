// src-tauri/src/system/context.rs
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub app_data_dir: PathBuf,
    pub scripts_dir: PathBuf,
    pub creations_dir: PathBuf,
}

impl AppContext {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let scripts_dir = app_data_dir.join("scripts");
        let creations_dir = app_data_dir.join("creations");
        Self {
            app_data_dir,
            scripts_dir,
            creations_dir,
        }
    }
}