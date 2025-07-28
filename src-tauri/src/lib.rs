// src-tauri/src/lib.rs

// This lib.rs file serves as the root of the `nexus` library crate.
// It declares all the public modules that make up the application's core logic.
// This allows `main.rs` to be a thin wrapper that simply initializes and runs the application,
// promoting better code organization and separation of concerns.

// Declare all the core modules of the application as public.
// This makes them accessible to other parts of the crate (like sub-modules)
// and to the binary entry point in `main.rs`.

pub mod commands;
pub mod database;
pub mod error;
pub mod knowledge_base;
pub mod services;
pub mod state;
pub mod system;

// It's common practice to re-export key types at the top level of the library
// for easier access.
pub use error::{AppError, Result};
pub use state::{AppState, AppStateInner};