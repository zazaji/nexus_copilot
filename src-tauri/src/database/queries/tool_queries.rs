use crate::database::models::*;
use crate::error::{AppError, Result};
use rusqlite::{params, Connection, OptionalExtension};

pub fn list_configured_tools(conn: &Connection) -> Result<Vec<ConfiguredTool>> {
    let mut stmt = conn.prepare("SELECT id, name, description, script_path, webhook_url, webhook_method, webhook_headers, webhook_body_template, input_schema, runtime, parameters, show_in_copilot, is_favorite, input_source, requires_ai_pre_processing, pre_processing_prompt, output_handling, requires_ai_post_processing, post_processing_prompt FROM configured_tools")?;
    let tool_iter = stmt.query_map([], |row| {
        Ok(ConfiguredTool {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            script_path: row.get(3)?,
            webhook_url: row.get(4)?,
            webhook_method: row.get(5)?,
            webhook_headers: row.get(6)?,
            webhook_body_template: row.get(7)?,
            input_schema: row.get(8)?,
            runtime: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(9)?)).unwrap_or_default(),
            parameters: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            show_in_copilot: row.get(11)?,
            is_favorite: row.get(12)?,
            input_source: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(13)?)).unwrap_or(ToolInputSource::UserInput),
            requires_ai_pre_processing: row.get(14)?,
            pre_processing_prompt: row.get(15)?,
            output_handling: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(16)?)).unwrap_or(ToolOutputHandling::RawText),
            requires_ai_post_processing: row.get(17)?,
            post_processing_prompt: row.get(18)?,
        })
    })?;
    tool_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn get_copilot_tools(conn: &Connection) -> Result<Vec<ConfiguredTool>> {
    let mut stmt = conn.prepare("SELECT id, name, description, script_path, webhook_url, webhook_method, webhook_headers, webhook_body_template, input_schema, runtime, parameters, show_in_copilot, is_favorite, input_source, requires_ai_pre_processing, pre_processing_prompt, output_handling, requires_ai_post_processing, post_processing_prompt FROM configured_tools WHERE show_in_copilot = TRUE")?;
    let tool_iter = stmt.query_map([], |row| {
        Ok(ConfiguredTool {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            script_path: row.get(3)?,
            webhook_url: row.get(4)?,
            webhook_method: row.get(5)?,
            webhook_headers: row.get(6)?,
            webhook_body_template: row.get(7)?,
            input_schema: row.get(8)?,
            runtime: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(9)?)).unwrap_or_default(),
            parameters: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            show_in_copilot: row.get(11)?,
            is_favorite: row.get(12)?,
            input_source: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(13)?)).unwrap_or(ToolInputSource::UserInput),
            requires_ai_pre_processing: row.get(14)?,
            pre_processing_prompt: row.get(15)?,
            output_handling: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(16)?)).unwrap_or(ToolOutputHandling::RawText),
            requires_ai_post_processing: row.get(17)?,
            post_processing_prompt: row.get(18)?,
        })
    })?;
    tool_iter.collect::<rusqlite::Result<Vec<_>>>().map_err(Into::into)
}

pub fn get_configured_tool_by_id(conn: &Connection, id: &str) -> Result<Option<ConfiguredTool>> {
    let mut stmt = conn.prepare("SELECT id, name, description, script_path, webhook_url, webhook_method, webhook_headers, webhook_body_template, input_schema, runtime, parameters, show_in_copilot, is_favorite, input_source, requires_ai_pre_processing, pre_processing_prompt, output_handling, requires_ai_post_processing, post_processing_prompt FROM configured_tools WHERE id = ?1")?;
    let tool = stmt.query_row(params![id], |row| {
        Ok(ConfiguredTool {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            script_path: row.get(3)?,
            webhook_url: row.get(4)?,
            webhook_method: row.get(5)?,
            webhook_headers: row.get(6)?,
            webhook_body_template: row.get(7)?,
            input_schema: row.get(8)?,
            runtime: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(9)?)).unwrap_or_default(),
            parameters: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            show_in_copilot: row.get(11)?,
            is_favorite: row.get(12)?,
            input_source: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(13)?)).unwrap_or(ToolInputSource::UserInput),
            requires_ai_pre_processing: row.get(14)?,
            pre_processing_prompt: row.get(15)?,
            output_handling: serde_json::from_str(&format!("\"{}\"", row.get::<_, String>(16)?)).unwrap_or(ToolOutputHandling::RawText),
            requires_ai_post_processing: row.get(17)?,
            post_processing_prompt: row.get(18)?,
        })
    }).optional()?;
    Ok(tool)
}

pub fn save_configured_tool(conn: &Connection, tool: &ConfiguredTool) -> Result<()> {
    let params_json = serde_json::to_string(&tool.parameters)?;
    let runtime_str = serde_json::to_string(&tool.runtime)?.trim_matches('"').to_string();
    let input_source_str = serde_json::to_string(&tool.input_source)?.trim_matches('"').to_string();
    let output_handling_str = serde_json::to_string(&tool.output_handling)?.trim_matches('"').to_string();

    conn.execute(
        "INSERT OR REPLACE INTO configured_tools (id, name, description, script_path, webhook_url, webhook_method, webhook_headers, webhook_body_template, input_schema, runtime, parameters, show_in_copilot, is_favorite, input_source, requires_ai_pre_processing, pre_processing_prompt, output_handling, requires_ai_post_processing, post_processing_prompt) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            &tool.id,
            &tool.name,
            &tool.description,
            &tool.script_path,
            &tool.webhook_url,
            &tool.webhook_method,
            &tool.webhook_headers,
            &tool.webhook_body_template,
            &tool.input_schema,
            runtime_str,
            params_json,
            &tool.show_in_copilot,
            &tool.is_favorite,
            input_source_str,
            &tool.requires_ai_pre_processing,
            &tool.pre_processing_prompt,
            output_handling_str,
            &tool.requires_ai_post_processing,
            &tool.post_processing_prompt
        ],
    )?;
    Ok(())
}

pub fn delete_configured_tool(conn: &Connection, id: &str) -> Result<()> {
    let affected = conn.execute("DELETE FROM configured_tools WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::Database("Tool not found for deletion".to_string()));
    }
    Ok(())
}