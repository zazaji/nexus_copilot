// src-tauri/src/state.rs
use crate::database;
use crate::error::Result;
use crate::system::context::AppContext;
use reqwest::Client;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: Mutex<Connection>,
    pub http_client: Client,
    pub context: AppContext,
    pub running_chat_tasks: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
    pub current_task_working_dir: Mutex<Option<PathBuf>>,
}

impl AppStateInner {
    pub fn new(db_path: &Path, context: AppContext) -> Result<Self> {
        log::info!("Initializing application state...");

        let open_and_migrate = |path: &Path| -> Result<Connection> {
            let mut conn = Connection::open(path)?;
            if let Err(e) = database::migrations::run(&mut conn) {
                log::error!("Database migration failed: {}. This might indicate a corrupt database.", e);
                conn.close().map_err(|(_, err)| err)?;
                fs::remove_file(path)?;
                log::info!("Corrupt database file removed. Re-initializing.");

                let mut new_conn = Connection::open(path)?;
                database::migrations::run(&mut new_conn)?;
                Ok(new_conn)
            } else {
                Ok(conn)
            }
        };

        let conn = open_and_migrate(db_path)?;

        log::info!("Application state initialized successfully.");
        Ok(Self {
            db: Mutex::new(conn),
            http_client: Client::new(),
            context,
            running_chat_tasks: Arc::new(Mutex::new(HashMap::new())),
            current_task_working_dir: Mutex::new(None),
        })
    }
}