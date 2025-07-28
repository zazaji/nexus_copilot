// src-tauri/src/prelude.rs
//! A module containing commonly used types and traits.
//! This module re-exports commonly used types and traits to reduce boilerplate.

// Standard library re-exports
pub use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

// External crate re-exports
pub use anyhow::{anyhow, Result as AnyResult};
pub use chrono::{DateTime, Utc};
pub use log::{debug, error, info, trace, warn};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value as JsonValue};
pub use thiserror::Error;
pub use tokio;
pub use uuid::Uuid;

// Crate re-exports
pub use crate::error::{AppError, Result};
pub use crate::state::AppState;