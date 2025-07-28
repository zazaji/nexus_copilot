use crate::database::models::*;
use crate::error::Result;
use rusqlite::{params, Connection};

const SETTINGS_KEY: &str = "app_settings";

pub fn get_settings(conn: &Connection) -> Result<Settings> {
    let res: rusqlite::Result<String> = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        [SETTINGS_KEY],
        |row| row.get(0),
    );

    match res {
        Ok(settings_json) => serde_json::from_str(&settings_json).map_err(Into::into),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(Settings::default()),
        Err(e) => Err(e.into()),
    }
}

pub fn save_settings(conn: &Connection, settings: &Settings) -> Result<()> {
    let json_val = serde_json::to_string_pretty(settings)?;
    conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)", params![SETTINGS_KEY, json_val])?;
    Ok(())
}