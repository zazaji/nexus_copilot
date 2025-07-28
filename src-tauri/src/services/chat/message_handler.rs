// src-tauri/src/services/chat/message_handler.rs
use super::llm_utils;
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    knowledge_base,
    services::proxy_types::{
        ProxyChatPayload, ProxyMessage, ProxyStreamChunk,
    },
    state::AppState,
};
use futures_util::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use base64::{engine::general_purpose, Engine as _};
use std::fs;

fn extract_text_from_content(content: &[models::ChatMessageContentPart]) -> String {
    content.iter()
        .filter_map(|part| match part {
            models::ChatMessageContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

async fn process_content_for_llm(content: &[models::ChatMessageContentPart]) -> Result<Vec<models::ChatMessageContentPart>> {
    let mut processed_parts = Vec::new();
    for part in content {
        match part {
            models::ChatMessageContentPart::ImageUrl { image_url } => {
                let url = &image_url.url;
                if !url.starts_with("http") && !url.starts_with("data:") {
                    // Assume it's a local file path
                    log::info!("Found local image path, converting to base64 data URL: {}", url);
                    let image_bytes = fs::read(url)?;
                    let base64_str = general_purpose::STANDARD.encode(&image_bytes);
                    let data_url = format!("data:image/webp;base64,{}", base64_str);
                    processed_parts.push(models::ChatMessageContentPart::ImageUrl {
                        image_url: models::ImageUrl { url: data_url },
                    });
                } else {
                    processed_parts.push(part.clone());
                }
            }
            _ => {
                processed_parts.push(part.clone());
            }
        }
    }
    Ok(processed_parts)
}

pub async fn handle_message(
    app: AppHandle,
    state: AppState,
    user_message: models::ChatMessage,
    ai_message_id: String,
    api_config: models::ApiConfig,
) -> Result<()> {
    log::info!("[ChatService] Handling user message ID: {}", user_message.id);

    {
        let db_conn = state.db.lock().unwrap();
        queries::ensure_conversation_exists(&db_conn, &user_message.conversation_id, "New Chat...", "chat")?;
        queries::save_message(&db_conn, &user_message)?;
    }

    let model_identifier = user_message.model.clone().ok_or_else(|| AppError::Config("No model specified in the request".to_string()))?;
    let user_query = extract_text_from_content(&user_message.content);

    let history = {
        let db_conn = state.db.lock().unwrap();
        queries::get_conversation_history(&db_conn, &user_message.conversation_id)?
    };

    let parts: Vec<&str> = model_identifier.split("::").collect();
    if parts.len() != 2 { return Err(AppError::Config(format!("Invalid model identifier: {}", model_identifier))); }
    let (provider_id, model_name) = (parts[0], parts[1]);

    let provider = api_config.providers.iter().find(|p| p.id == provider_id).cloned().ok_or_else(|| AppError::Config(format!("Provider with ID {} not found", provider_id)))?;

    let mut processed_history = Vec::new();
    for msg in &history {
        let processed_content = process_content_for_llm(&msg.content).await?;
        processed_history.push((msg.role.clone(), processed_content));
    }

    let proxy_messages: Vec<ProxyMessage> = processed_history.iter().map(|(role, content)| {
        let proxy_role = if role == "ai" { "assistant" } else { role };
        ProxyMessage {
            role: proxy_role.to_string(),
            content: content
        }
    }).collect();
    
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let request_body = ProxyChatPayload {
        model: &model_name,
        messages: proxy_messages,
        stream: true,
        provider_config: &provider,
        knowledge_base_selection: user_message.knowledge_base_selection.clone(),
        api_config: Some(&api_config),
    };

    let url = format!("{}/api/v1/proxy/chat/completions", settings.execution.backend_url);
    let request_builder = state.http_client.post(url).json(&request_body);
    let mut es = EventSource::new(request_builder)?;

    let mut thinking_parts = String::new();
    let mut content_parts = String::new();
    let mut sources: Option<Vec<knowledge_base::models::KnowledgeSource>> = None;

    let stop_flag = Arc::new(AtomicBool::new(false));
    state.running_chat_tasks.lock().unwrap().insert(ai_message_id.clone(), stop_flag.clone());

    while let Some(event) = es.next().await {
        if stop_flag.load(Ordering::SeqCst) {
            log::info!("[ChatService] Stop signal received for message ID: {}. Closing stream.", ai_message_id);
            es.close();
            break;
        }

        match event {
            Ok(Event::Open) => log::info!("[ChatService] SSE connection opened to proxy for message ID: {}", ai_message_id),
            Ok(Event::Message(message)) => {
                if message.event == "sources" {
                    sources = serde_json::from_str(&message.data).ok();
                    log::info!("[ChatService] Received {} sources from backend.", sources.as_ref().map_or(0, |s| s.len()));
                    continue;
                }

                if message.data == "[DONE]" {
                    log::info!("[ChatService] SSE stream [DONE] received from proxy.");
                    break;
                }
                app.emit_all("stream-chunk", json!({ "messageId": ai_message_id, "chunk": &message.data }))?;

                if let Ok(parsed_chunk) = serde_json::from_str::<ProxyStreamChunk>(&message.data) {
                    if let Some(choice) = parsed_chunk.choices.get(0) {
                        if let Some(reasoning) = &choice.delta.reasoning_content {
                            thinking_parts.push_str(reasoning);
                        }
                        if let Some(content) = &choice.delta.content {
                            content_parts.push_str(content);
                        }
                    }
                }
            }
            Err(e) => {
                let error_string = e.to_string();
                if error_string.contains("Stream ended") {
                    log::info!("[ChatService] SSE stream ended normally for message ID: {}", ai_message_id);
                } else {
                    log::error!("[ChatService] SSE error from proxy: {}", error_string);
                    // We will still proceed to the end to let the frontend handle the partial data.
                }
                es.close();
                break; // Break the loop on any error, including normal stream end.
            }
        }
    }

    state.running_chat_tasks.lock().unwrap().remove(&ai_message_id);

    let full_response = if !thinking_parts.is_empty() {
        format!("<think>{}</think>{}", thinking_parts, content_parts)
    } else {
        content_parts
    };

    let suggestions = llm_utils::generate_suggestions(&state, &full_response).await.unwrap_or_default();

    let final_ai_message = models::ChatMessage {
        id: ai_message_id.clone(),
        conversation_id: user_message.conversation_id.clone(),
        role: "ai".to_string(),
        content: vec![models::ChatMessageContentPart::Text { text: full_response.clone() }],
        timestamp: chrono::Utc::now().timestamp_millis(),
        sources: sources.clone(),
        error: None,
        suggestions: Some(suggestions.clone()),
        model: user_message.model.clone(),
        knowledge_base_selection: user_message.knowledge_base_selection.clone(),
        is_executing: None,
        execution_output: None,
        agent_task_id: None,
        agent_task: None,
    };

    {
        let db_conn = state.db.lock().unwrap();
        queries::save_message(&db_conn, &final_ai_message)?;
    }

    app.emit_all("stream-end", json!({ "messageId": ai_message_id, "finalMessage": { "content": full_response, "sources": sources, "suggestions": suggestions, "error": null } }))?;

    let convo_result = {
        let db_conn = state.db.lock().unwrap();
        queries::get_conversation_by_id(&db_conn, &user_message.conversation_id)?
    };

    if let Some(convo) = convo_result {
        if convo.title == "New Chat..." {
            let title = llm_utils::generate_title_for_conversation(&state, &user_query, &full_response).await.unwrap_or_else(|_| "Untitled Chat".to_string());
            let updated_convo = {
                let db_conn = state.db.lock().unwrap();
                queries::update_conversation_title(&db_conn, &user_message.conversation_id, &title)?;
                queries::get_conversation_by_id(&db_conn, &user_message.conversation_id)?.unwrap()
            };
            app.emit_all("conversation-title-updated", updated_convo)?;
        }
    }

    Ok(())
}