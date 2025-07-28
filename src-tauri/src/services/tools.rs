// src-tauri/src/services/tools.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    knowledge_base,
    state::AppState,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use tauri::{AppHandle, Manager};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct ToolMetadata {
    name: String,
    description: String,
}

fn get_runtime_from_extension(path: &Path) -> models::ToolRuntime {
    match path.extension().and_then(|s| s.to_str()) {
        Some("py") => models::ToolRuntime::Python,
        Some("js") => models::ToolRuntime::Node,
        Some("sh") | Some("bat") | Some("cmd") => models::ToolRuntime::Shell,
        _ => models::ToolRuntime::Shell,
    }
}

async fn parse_script_for_tool(path: &std::path::Path) -> Result<Option<crate::commands::tools::DynamicTool>> {
    let content = fs::read_to_string(path)?;
    let mut metadata_json = String::new();
    let mut in_metadata_block = false;

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line == "### NEXUS-TOOL-END ###" {
            break;
        }
        if in_metadata_block {
            metadata_json.push_str(trimmed_line.trim_start_matches('#').trim());
        }
        if trimmed_line == "### NEXUS-TOOL ###" {
            in_metadata_block = true;
        }
    }

    if metadata_json.is_empty() {
        log::debug!("No NEXUS-TOOL metadata block found in {:?}", path);
        return Ok(None);
    }

    let metadata: ToolMetadata = serde_json::from_str(&metadata_json)
        .map_err(|e| AppError::Parse(format!("Failed to parse tool metadata for {:?}: {}. Content: '{}'", path, e, metadata_json)))?;

    let script_path = path.to_string_lossy().to_string();
    let id = format!("dynamic::{}", script_path);
    let runtime = get_runtime_from_extension(path);

    Ok(Some(crate::commands::tools::DynamicTool {
        id,
        name: metadata.name,
        description: metadata.description,
        script_path,
        runtime,
    }))
}

pub async fn list_dynamic_tools(state: &AppState) -> Result<Vec<crate::commands::tools::DynamicTool>> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let script_dirs = settings.knowledge_base.scripts_directories;
    let mut tools = Vec::new();

    if script_dirs.is_empty() {
        log::info!("No script directories configured. Skipping dynamic tool discovery.");
        return Ok(tools);
    }

    for dir in script_dirs {
        log::info!("Searching for dynamic tools in directory: {}", dir);
        for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                match parse_script_for_tool(&path).await {
                    Ok(Some(tool)) => {
                        log::info!("Successfully parsed tool: '{}' from {:?}", tool.name, path);
                        tools.push(tool);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        log::warn!("Failed to parse tool from {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(tools)
}

pub async fn execute_tool_from_payload(app: AppHandle, state: &AppState, payload: Value) -> Result<String> {
    let tool_id = payload["toolName"]
        .as_str()
        .ok_or_else(|| AppError::Internal("Missing 'toolName' in payload".to_string()))?;
    let params = payload["params"].clone();
    let task_id = Uuid::new_v4().to_string();

    let state_clone = state.clone();
    let tool_id_clone = tool_id.to_string();
    let app_clone = app.clone();
    let task_id_clone = task_id.clone();

    tokio::spawn(async move {
        let result = execute(&state_clone, &tool_id_clone, params, &task_id_clone, &app_clone).await;
        let event_name = if result.is_ok() { "tool-complete" } else { "tool-error" };
        let payload = result.unwrap_or_else(|e| e.to_string());

        app.emit_all(event_name, json!({ "taskId": task_id_clone, "payload": payload })).ok();
    });

    Ok(task_id)
}

async fn execute(state: &AppState, tool_id: &str, params: Value, task_id: &str, app: &AppHandle) -> Result<String> {
    log::info!("Executing tool: {} with task ID: {}", tool_id, task_id);

    if tool_id.starts_with("built_in::") {
        return match tool_id {
            "built_in::find_file" => find_file_in_indexed_dirs(state, params).await,
            "built_in::save_to_kb" => save_to_kb(state, params).await,
            _ => Err(AppError::Internal(format!("Unknown built-in tool: {}", tool_id))),
        }
    }

    let tool = if let Some(tool) = queries::get_configured_tool_by_id(&state.db.lock().unwrap(), tool_id)? {
        tool
    } else {
        return Err(AppError::Internal(format!("Tool with ID {} not found", tool_id)));
    };

    let (args, stdin_data) = extract_args_and_stdin(params.clone());

    match tool.runtime {
        models::ToolRuntime::Python => {
            let script_path = tool.script_path.ok_or_else(|| AppError::Config("Tool is Python runtime but has no script_path".to_string()))?;
            run_python_script_async(state, &script_path, stdin_data, args, task_id, app).await
        },
        models::ToolRuntime::Node => {
            let script_path = tool.script_path.ok_or_else(|| AppError::Config("Tool is Node runtime but has no script_path".to_string()))?;
            run_node_script_async(state, &script_path, stdin_data, args, task_id, app).await
        },
        models::ToolRuntime::Shell => {
            let script_path = tool.script_path.ok_or_else(|| AppError::Config("Tool is Shell runtime but has no script_path".to_string()))?;
            run_shell_script_async(&script_path, stdin_data, args, task_id, app).await
        },
        models::ToolRuntime::Webhook => {
            let webhook_url = tool.webhook_url.ok_or_else(|| AppError::Config("Tool is Webhook runtime but has no webhook_url".to_string()))?;
            call_fastapi_webhook_executor(state, &webhook_url, params, task_id, app).await
        },
    }
}

async fn call_fastapi_webhook_executor(state: &AppState, url: &str, params: Value, task_id: &str, app: &AppHandle) -> Result<String> {
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let execution_url = format!("{}/api/v1/tools/execute-webhook-tool", backend_url);

    let payload = json!({
        "url": url,
        "params": params
    });

    let response = state.http_client.post(&execution_url).json(&payload).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown backend error".into());
        return Err(AppError::Internal(format!("Backend webhook executor failed: {}", error_text)));
    }

    let response_data: Value = response.json().await?;
    let result_str = serde_json::to_string_pretty(&response_data.get("result"))?;

    // Emit the output as a single chunk since it's not a stream
    app.emit_all("tool-output", json!({ "taskId": task_id, "chunk": &result_str })).ok();

    Ok(result_str)
}

fn extract_args_and_stdin(params: Value) -> (Vec<String>, Option<String>) {
    let mut args = Vec::new();
    let mut stdin_data: Option<String> = None;
    if let Some(map) = params.as_object() {
        for (key, value) in map {
            if key == "stdin" {
                stdin_data = value.as_str().map(String::from);
            } else if let Some(value_str) = value.as_str() {
                if !value_str.is_empty() {
                    args.push(format!("--{}", key));
                    args.push(value_str.to_string());
                }
            }
        }
    }
    (args, stdin_data)
}

async fn save_to_kb(state: &AppState, params: Value) -> Result<String> {
    let content = params["stdin"].as_str().ok_or_else(|| AppError::Internal("Missing content for save_to_kb".to_string()))?;

    let settings = queries::get_settings(&state.db.lock().unwrap())?;

    let root_dir = settings.knowledge_base.default_save_directory
        .or_else(|| settings.knowledge_base.indexed_directories.get(0).cloned())
        .ok_or_else(|| AppError::Config("No default save directory set and no indexed directories available.".to_string()))?;

    let filename = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let new_path = Path::new(&root_dir).join(format!("{}.md", filename));

    fs::write(&new_path, content)?;

    let path_str = new_path.to_string_lossy().to_string();
    knowledge_base::indexer::reindex_file(state.clone(), path_str.clone()).await?;

    Ok(format!("Note saved to {}", path_str))
}

async fn find_file_in_indexed_dirs(state: &AppState, params: Value) -> Result<String> {
    let file_name = params["stdin"].as_str().ok_or_else(|| AppError::Internal("Missing 'stdin' parameter for find_file".to_string()))?;
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let indexed_dirs = settings.knowledge_base.indexed_directories;
    let mut found_files = Vec::new();
    for dir in indexed_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name().to_string_lossy().to_lowercase().contains(&file_name.to_lowercase()) {
                found_files.push(entry.path().to_string_lossy().to_string());
            }
        }
    }
    Ok(if found_files.is_empty() { format!("No files found matching '{}'.", file_name) } else { format!("Found files:\n{}", found_files.join("\n")) })
}

async fn run_python_script_async(state: &AppState, script_path: &str, stdin_data: Option<String>, args: Vec<String>, task_id: &str, app: &AppHandle) -> Result<String> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let python_path = if settings.execution.python_path.is_empty() {
        "python".to_string()
    } else {
        settings.execution.python_path
    };
    run_interpreter_script(python_path, script_path, stdin_data, args, task_id, app, &settings.execution.working_directory).await
}

async fn run_node_script_async(state: &AppState, script_path: &str, stdin_data: Option<String>, args: Vec<String>, task_id: &str, app: &AppHandle) -> Result<String> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let node_path = if settings.execution.node_path.is_empty() {
        "node".to_string()
    } else {
        settings.execution.node_path
    };
    run_interpreter_script(node_path, script_path, stdin_data, args, task_id, app, &settings.execution.working_directory).await
}

async fn run_shell_script_async(script_path: &str, stdin_data: Option<String>, args: Vec<String>, task_id: &str, app: &AppHandle) -> Result<String> {
    let command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(script_path);
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg(script_path);
        cmd
    };
    run_command_async(command, stdin_data, args, task_id, app).await
}

async fn run_interpreter_script(interpreter: String, script_path: &str, stdin_data: Option<String>, args: Vec<String>, task_id: &str, app: &AppHandle, working_dir: &str) -> Result<String> {
    let mut command = Command::new(interpreter);
    command.arg(script_path).args(args);
    if !working_dir.is_empty() {
        command.current_dir(working_dir);
    }
    run_command_async(command, stdin_data, vec![], task_id, app).await
}

async fn run_command_async(mut command: Command, stdin_data: Option<String>, args: Vec<String>, task_id: &str, app: &AppHandle) -> Result<String> {
    command.args(args);
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(data) = stdin_data {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(data.as_bytes()).await?;
        }
    }

    let stdout = child.stdout.take().ok_or_else(|| AppError::Internal("Failed to capture stdout".to_string()))?;
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    let mut full_output = String::new();

    while let Some(line) = lines.next_line().await? {
        let line_with_newline = format!("{}\n", line);
        app.emit_all("tool-output", json!({ "taskId": task_id, "chunk": &line_with_newline })).ok();
        full_output.push_str(&line_with_newline);
    }

    let status = child.wait().await?;
    if status.success() {
        Ok(full_output)
    } else {
        let stderr = child.stderr.take().ok_or_else(|| AppError::Internal("Failed to capture stderr".to_string()))?;
        let err_reader = BufReader::new(stderr);
        let mut err_lines_stream = err_reader.lines();
        let mut err_output = String::new();
        while let Some(line) = err_lines_stream.next_line().await? {
            err_output.push_str(&line);
            err_output.push('\n');
        }
        Err(AppError::Internal(format!("Script failed: {}", err_output)))
    }
}

pub async fn execute_python_code(app: AppHandle, state: AppState, execution_id: String, code: String) -> Result<()> {
    tokio::spawn(async move {
        let settings = match queries::get_settings(&state.db.lock().unwrap()) {
            Ok(s) => s,
            Err(e) => {
                let _ = app.emit_all("code-execution-complete", json!({ "executionId": execution_id, "status": "error", "error": e.to_string() }));
                return;
            }
        };

        let python_path = if settings.execution.python_path.is_empty() {
            "python".to_string()
        } else {
            settings.execution.python_path
        };

        let mut command = Command::new(python_path);
        command.arg("-c").arg(code);

        if !settings.execution.working_directory.is_empty() {
            command.current_dir(settings.execution.working_directory);
        }

        let mut child = match command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = app.emit_all("code-execution-complete", json!({ "executionId": execution_id, "status": "error", "error": e.to_string() }));
                return;
            }
        };

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        loop {
            tokio::select! {
                line = stdout_reader.next_line() => {
                    match line {
                        Ok(Some(line)) => {
                            let _ = app.emit_all("code-execution-output", json!({ "executionId": &execution_id, "chunk": format!("{}\n", line) }));
                        },
                        Ok(None) => break, // stdout closed
                        Err(e) => {
                             let _ = app.emit_all("code-execution-output", json!({ "executionId": &execution_id, "chunk": format!("[ERROR] Failed to read stdout: {}\n", e) }));
                             break;
                        }
                    }
                },
                line = stderr_reader.next_line() => {
                     match line {
                        Ok(Some(line)) => {
                            let _ = app.emit_all("code-execution-output", json!({ "executionId": &execution_id, "chunk": format!("[STDERR] {}\n", line) }));
                        },
                        Ok(None) => {}, // stderr might close before stdout
                        Err(e) => {
                             let _ = app.emit_all("code-execution-output", json!({ "executionId": &execution_id, "chunk": format!("[ERROR] Failed to read stderr: {}\n", e) }));
                        }
                    }
                }
            }
        }

        match child.wait().await {
            Ok(status) if status.success() => {
                let _ = app.emit_all("code-execution-complete", json!({ "executionId": execution_id, "status": "success" }));
            },
            Ok(status) => {
                let _ = app.emit_all("code-execution-complete", json!({ "executionId": execution_id, "status": "error", "error": format!("Process exited with status: {}", status) }));
            },
            Err(e) => {
                let _ = app.emit_all("code-execution-complete", json!({ "executionId": execution_id, "status": "error", "error": e.to_string() }));
            }
        }
    });
    Ok(())
}

pub async fn execute_shell_command_service(app: AppHandle, command: String) -> Result<String> {
    let task_id = Uuid::new_v4().to_string();
    let app_clone = app.clone();
    let task_id_clone = task_id.clone();

    tokio::spawn(async move {
        let cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg(command.clone());
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(command.clone());
            cmd
        };

        let result = run_command_async(cmd, None, vec![], &task_id_clone, &app_clone).await;
        let event_name = if result.is_ok() { "tool-complete" } else { "tool-error" };
        let payload = result.unwrap_or_else(|e| e.to_string());

        app.emit_all(event_name, json!({ "taskId": task_id_clone, "payload": payload })).ok();
    });

    Ok(task_id)
}

pub async fn execute_generic_code(
    app: AppHandle,
    state: AppState,
    runtime: models::ToolRuntime,
    code: String,
) -> Result<String> {
    let task_id = Uuid::new_v4().to_string();
    let app_clone = app.clone();
    let task_id_clone = task_id.clone();
    let state_clone = state.clone();

    tokio::spawn(async move {
        let result = async {
            let settings = queries::get_settings(&state_clone.db.lock().unwrap())?;
            let working_dir = if !settings.execution.working_directory.is_empty() {
                Some(settings.execution.working_directory)
            } else {
                None
            };

            let mut command = match runtime {
                models::ToolRuntime::Shell => {
                    log::info!("[Tool Service] Executing generic shell command directly: {}", code);
                    if cfg!(target_os = "windows") {
                        let mut cmd = Command::new("cmd");
                        cmd.arg("/C").arg(&code);
                        cmd
                    } else {
                        let mut cmd = Command::new("sh");
                        cmd.arg("-c").arg(&code);
                        cmd
                    }
                }
                models::ToolRuntime::Python | models::ToolRuntime::Node => {
                    log::info!("[Tool Service] Executing generic code via temp file for runtime: {:?}", runtime);
                    
                    let script_dir: std::path::PathBuf = working_dir
                        .clone()
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(env::temp_dir);

                    fs::create_dir_all(&script_dir)?;

                    let extension = if runtime == models::ToolRuntime::Python { "py" } else { "js" };
                    let script_path = script_dir.join(format!("{}.{}", task_id_clone, extension));
                    fs::write(&script_path, &code)?;

                    let interpreter_path = if runtime == models::ToolRuntime::Python {
                        if settings.execution.python_path.is_empty() { "python".to_string() } else { settings.execution.python_path }
                    } else {
                        if settings.execution.node_path.is_empty() { "node".to_string() } else { settings.execution.node_path }
                    };
                    
                    let mut cmd = Command::new(interpreter_path);
                    cmd.arg(&script_path);
                    
                    let _cleaner = TempFileCleaner(script_path);

                    cmd
                }
                models::ToolRuntime::Webhook => return Err(AppError::Internal("Webhook runtime cannot be executed directly.".to_string())),
            };

            if let Some(dir) = working_dir {
                command.current_dir(dir);
            }

            run_command_async(command, None, vec![], &task_id_clone, &app_clone).await
        }.await;

        let event_name = if result.is_ok() { "tool-complete" } else { "tool-error" };
        let payload = result.unwrap_or_else(|e| e.to_string());
        app.emit_all(event_name, json!({ "taskId": task_id_clone, "payload": payload })).ok();
    });

    Ok(task_id)
}

// A helper struct to ensure temporary files are cleaned up even on panic.
struct TempFileCleaner(std::path::PathBuf);
impl Drop for TempFileCleaner {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.0) {
            log::warn!("Failed to clean up temporary script file {:?}: {}", self.0, e);
        }
    }
}