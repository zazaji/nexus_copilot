// src-tauri/src/services/chat/llm_utils.rs
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    services::proxy_types::{ProxyChatPayload, ProxyMessage, ProxyResponse},
    state::AppState,
};

pub async fn generate_title_for_conversation(state: &AppState, user_query: &str, ai_response: &str) -> Result<String> {
    let title_prompt = format!(
        "Based on the following user query and AI response, generate a very short, concise title (5-10 words max) for this conversation. Do not use quotes. \n\nUSER: \"{}\"\nAI: \"{}\"\n\nTITLE:",
        user_query,
        ai_response.chars().take(200).collect::<String>()
    );
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let model_endpoint = settings.api_config.assignments.suggestion.as_ref().ok_or_else(|| AppError::Config("Suggestion model not assigned".to_string()))?;
    let provider = settings.api_config.providers.iter().find(|p| p.id == model_endpoint.provider_id).cloned().ok_or_else(|| AppError::Config("Provider not found".to_string()))?;

    let content_part = vec![models::ChatMessageContentPart::Text { text: title_prompt }];
    let messages = vec![ProxyMessage { role: "user".to_string(), content: &content_part }];
    
    let request_body = ProxyChatPayload {
        model: &model_endpoint.model_name,
        messages,
        stream: false,
        provider_config: &provider,
        knowledge_base_selection: None,
        api_config: None,
    };

    let url = format!("{}/api/v1/proxy/chat/completions", settings.execution.backend_url);
    let response = state.http_client.post(url).json(&request_body).send().await?.error_for_status()?;
    let response_data: ProxyResponse = response.json().await?;

    let title = response_data.choices.get(0).map_or_else(
        || "Untitled Chat".to_string(),
        |choice| choice.message.content.trim().trim_matches('"').to_string()
    );

    Ok(title)
}

pub async fn generate_suggestions(state: &AppState, context: &str) -> Result<Vec<String>> {
    let suggestion_prompt = format!(
        "Based on the following text, suggest three concise, actionable next steps for the user. Output only a single, valid JSON array of strings, like [\"Suggestion 1\", \"Suggestion 2\", \"Suggestion 3\"]. Text: \"{}\"",
        context
    );
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let model_endpoint = settings.api_config.assignments.suggestion.as_ref().ok_or_else(|| AppError::Config("Suggestion model not assigned".to_string()))?;
    let provider = settings.api_config.providers.iter().find(|p| p.id == model_endpoint.provider_id).cloned().ok_or_else(|| AppError::Config("Provider not found".to_string()))?;

    let content_part = vec![models::ChatMessageContentPart::Text { text: suggestion_prompt }];
    let messages = vec![ProxyMessage { role: "user".to_string(), content: &content_part }];
    
    let request_body = ProxyChatPayload {
        model: &model_endpoint.model_name,
        messages,
        stream: false,
        provider_config: &provider,
        knowledge_base_selection: None,
        api_config: None,
    };

    let url = format!("{}/api/v1/proxy/chat/completions", settings.execution.backend_url);
    let response = state.http_client.post(url).json(&request_body).send().await?.error_for_status()?;
    let response_data: ProxyResponse = response.json().await?;
    let response_text = response_data.choices.get(0).map_or("[]".to_string(), |c| c.message.content.clone());

    if let (Some(start), Some(end)) = (response_text.find('['), response_text.rfind(']')) {
        let json_str = &response_text[start..=end];
        serde_json::from_str(json_str).map_err(|e| {
            log::error!("Failed to parse suggestions from LLM response. Raw: '{}', Parsed: '{}', Error: {}", response_text, json_str, e);
            AppError::Internal("Failed to parse suggestions from proxy".to_string())
        })
    } else {
        log::warn!("No JSON array found in suggestions response: {}", response_text);
        Ok(vec![])
    }
}