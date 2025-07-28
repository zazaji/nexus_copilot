// src-tauri/src/knowledge_base/searcher.rs
use super::models::KnowledgeSource;
use crate::{
    database::{models, queries},
    error::{AppError, Result},
    services::{
        proxy_types::{EmbeddingResponse, ProxyEmbeddingPayload},
        vector_client,
    },
    state::AppState,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

async fn get_embeddings_from_proxy(state: &AppState, provider_config: &models::ApiProvider, model_name: &str, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    if texts.is_empty() { return Ok(vec![]); }
    let backend_url = {
        let conn = state.db.lock().unwrap();
        queries::get_settings(&conn)?.execution.backend_url
    };
    let url = format!("{}/api/v1/proxy/embeddings", backend_url);
    
    let payload = ProxyEmbeddingPayload { model: model_name, input: texts, provider_config };

    let response = state.http_client.post(url).json(&payload).send().await?.error_for_status()?;
    let mut response_data: EmbeddingResponse = response.json().await?;
    response_data.data.sort_by_key(|d| d.index);
    Ok(response_data.data.into_iter().map(|d| d.embedding).collect())
}

pub async fn search(
    state: &AppState,
    query: String,
    where_filter: Option<Value>,
    top_k: u32,
    score_threshold: f32,
) -> Result<Vec<KnowledgeSource>> {
    log::info!("Searching for: '{}' with filter: {:?}, top_k: {}, score_threshold: {}", query, where_filter, top_k, score_threshold);

    let (provider_config, model_name, backend_url) = {
        let settings = queries::get_settings(&state.db.lock().unwrap())?;
        let embedding_endpoint = settings.api_config.assignments.embedding.ok_or_else(|| AppError::Config("Embedding model not assigned".to_string()))?;
        let provider = settings.api_config.providers.iter().find(|p| p.id == embedding_endpoint.provider_id).cloned().ok_or_else(|| AppError::Config("Embedding provider not found".to_string()))?;
        (provider, embedding_endpoint.model_name, settings.execution.backend_url)
    };

    let query_embedding = get_embeddings_from_proxy(state, &provider_config, &model_name, &[query])
        .await?
        .pop()
        .ok_or_else(|| AppError::Internal("Failed to generate query embedding".to_string()))?;

    let payload = vector_client::QueryPayload {
        base: vector_client::VectorBase { database: "nexus_db", collection: "knowledge_base" },
        query_embeddings: vec![query_embedding],
        n_results: top_k,
        where_filter,
        score_threshold: Some(score_threshold),
    };

    let query_result = vector_client::query(state, &backend_url, &payload).await?;

    let ids = query_result.ids.into_iter().next().unwrap_or_default();
    let documents = query_result.documents.into_iter().next().flatten().unwrap_or_default();
    let metadatas = query_result.metadatas.into_iter().next().flatten().unwrap_or_default();
    let distances = query_result.distances.into_iter().next().flatten().unwrap_or_default();

    let mut best_results: HashMap<String, KnowledgeSource> = HashMap::new();

    for (((id, doc_opt), meta_opt), dist) in ids.into_iter().zip(documents).zip(metadatas).zip(distances) {
        if let (Some(document), Some(metadata)) = (doc_opt, meta_opt) {
            if let Some(file_path) = metadata.get("file_path").and_then(|v| v.as_str()) {
                let score = 1.0 / (1.0 + dist);

                let current_source = KnowledgeSource {
                    id,
                    file_path: file_path.to_string(),
                    source_name: Path::new(file_path).file_name().and_then(|s| s.to_str()).unwrap_or("").to_string(),
                    content_snippet: document,
                    score,
                };

                best_results.entry(file_path.to_string())
                    .and_modify(|existing| {
                        if current_source.score > existing.score {
                            *existing = current_source.clone();
                        }
                    })
                    .or_insert(current_source);
            }
        }
    }

    let mut final_sources: Vec<KnowledgeSource> = best_results.into_values().collect();
    final_sources.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    final_sources.truncate(top_k as usize);

    Ok(final_sources)
}

pub async fn search_online(
    state: &AppState,
    kb_id: &str,
    query: &str,
    top_k: u32,
    score_threshold: f32,
) -> Result<Vec<KnowledgeSource>> {
    let online_kb = {
        let conn = state.db.lock().unwrap();
        queries::get_online_kb_by_id(&conn, kb_id)?
            .ok_or_else(|| AppError::Config(format!("Online KB with ID {} not found", kb_id)))?
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