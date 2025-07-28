use crate::{
    database::queries,
    error::{AppError, Result},
    knowledge_base::{self, models::KnowledgeSource},
    state::AppState,
};
use serde_json::json;

pub async fn search_online_kb(
    state: &AppState,
    online_kb_id: &str,
    query: &str,
    top_k: u32,
    score_threshold: f32,
) -> Result<Vec<KnowledgeSource>> {
    let online_kb = {
        let conn = state.db.lock().unwrap();
        queries::get_online_kb_by_id(&conn, online_kb_id)?
            .ok_or_else(|| AppError::Config(format!("Online KB with ID {} not found", online_kb_id)))?
    };

    let request_payload = json!({
        "query": query,
        "top_k": top_k,
        "score_threshold": score_threshold
    });

    let response = state.http_client
        .post(&online_kb.url)
        .header("Authorization", format!("Bearer {}", online_kb.token))
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await?
        .error_for_status()?;

    let results: Vec<serde_json::Value> = response.json().await?;
    let sources = results.into_iter().map(|res| {
        let item_id = res.get("id").and_then(|v| v.as_str()).unwrap_or_default();
        KnowledgeSource {
            id: item_id.to_string(),
            file_path: format!("online-kb://{}/{}", online_kb.id, item_id),
            source_name: res.get("source_name").and_then(|v| v.as_str()).unwrap_or(&online_kb.name).to_string(),
            content_snippet: res.get("content").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
            score: res.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
        }
    }).collect();

    Ok(sources)
}

pub async fn perform_rag(
    state: &AppState,
    kb_selection: &str,
    user_query: &str,
) -> Result<(String, Option<Vec<KnowledgeSource>>)> {
    let settings = queries::get_settings(&state.db.lock().unwrap())?;
    let kb_settings = &settings.knowledge_base;

    let search_results = if kb_selection.starts_with("online::") {
        let online_kb_id = kb_selection.strip_prefix("online::").unwrap();
        search_online_kb(state, online_kb_id, user_query, kb_settings.top_k, kb_settings.score_threshold).await
    } else {
        let where_filter = if kb_selection == "all" { None } else { Some(json!({ "file_path": { "$like": format!("{}%", kb_selection) } })) };
        knowledge_base::searcher::search(state, user_query.to_string(), where_filter, kb_settings.top_k, kb_settings.score_threshold).await
    };

    match search_results {
        Ok(retrieved_sources) if !retrieved_sources.is_empty() => {
            let context_str = retrieved_sources.iter()
                .map(|s| format!("Source: {}\nContent: {}", s.source_name, s.content_snippet))
                .collect::<Vec<_>>()
                .join("\n\n");
            let augmented_query = format!("Use the following context to answer the user's question.\n\n--- CONTEXT ---\n{}\n\n--- END CONTEXT ---\n\nUser Question: {}", context_str, user_query);
            Ok((augmented_query, Some(retrieved_sources)))
        }
        _ => Ok((user_query.to_string(), None)),
    }
}