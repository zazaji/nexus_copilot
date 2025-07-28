// src-tauri/src/database/queries/note_queries.rs
// This file is intentionally left with only struct definitions.
// The responsibility for querying and managing notes and their links
// has been moved to the FastAPI backend to maintain a clear architectural boundary.
// Rust commands will now fetch this data via API calls.

// Note: The functions that directly queried the Rust-managed DB have been removed.
// The structs are kept as they are used for deserializing API responses from FastAPI.