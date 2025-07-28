// src-tauri/src/error.rs
use enigo::InputError;
use reqwest_eventsource::CannotCloneRequestError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Vector Service error: {0}")]
    VectorService(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("File parsing error: {0}")]
    Parse(String),
    #[error("API client error: {0}")]
    ApiClient(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_connect() || e.is_timeout() {
            return AppError::VectorService(format!("Connection error: {}", e));
        }
        AppError::ApiClient(e.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(e: tauri::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<CannotCloneRequestError> for AppError {
    fn from(e: CannotCloneRequestError) -> Self {
        AppError::ApiClient(format!("Failed to build streaming request: {}", e))
    }
}

impl From<url::ParseError> for AppError {
    fn from(e: url::ParseError) -> Self {
        AppError::Config(format!("URL parsing error: {}", e))
    }
}

impl From<InputError> for AppError {
    fn from(e: InputError) -> Self {
        AppError::Internal(format!("Input simulation error: {}", e))
    }
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;