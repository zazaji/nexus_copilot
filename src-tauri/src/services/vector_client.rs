// src-tauri/src/services/vector_client.rs
use crate::{
    error::{AppError, Result},
    state::AppState,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const VECTOR_DB_NAME: &str = "nexus_db";
const VECTOR_COLLECTION_NAME: &str = "knowledge_base";

#[derive(Serialize)]
pub struct VectorBase<'a> {
    pub database: &'a str,
    pub collection: &'a str,
}

#[derive(Serialize)]
pub struct EnsureCollectionPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
}

#[derive(Serialize)]
pub struct AddPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
    pub ids: Vec<String>,
    pub embeddings: Vec<Vec<f32>>,
    pub documents: Vec<&'a str>,
    pub metadatas: Vec<Value>,
}

#[derive(Serialize)]
pub struct QueryPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
    pub query_embeddings: Vec<Vec<f32>>,
    pub n_results: u32,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub where_filter: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
}

#[derive(Serialize)]
pub struct DeletePayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
    #[serde(rename = "where")]
    pub where_metadata: Value,
}

#[derive(Serialize)]
pub struct UpdateMetadataPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
    #[serde(rename = "where")]
    pub where_metadata: Value,
    #[serde(rename = "new_metadata")]
    pub new_metadata: Value,
}

#[derive(Serialize)]
pub struct ClearCollectionPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
}

#[derive(Serialize)]
pub struct CountPayload<'a> {
    #[serde(flatten)]
    pub base: VectorBase<'a>,
}


#[derive(Deserialize, Debug)]
pub struct QueryResponse {
    pub ids: Vec<Vec<String>>,
    pub documents: Vec<Option<Vec<Option<String>>>>,
    pub metadatas: Vec<Option<Vec<Option<Value>>>>,
    pub distances: Vec<Option<Vec<f32>>>,
}

#[derive(Deserialize, Debug)]
pub struct CountResponse {
    pub count: i64,
}

async fn post<T: Serialize>(state: &AppState, url: &str, body: &T) -> Result<()> {
    log::info!("[VectorClient] Sending POST request to: {}", url);
    let res = state.http_client.post(url).json(body).send().await?;
    let status = res.status();
    if !status.is_success() {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".into());
        log::error!("[VectorClient] Request to {} failed with status {}: {}", url, status, error_text);
        return Err(AppError::VectorService(format!(
            "Failed request to {}: {}",
            url, error_text
        )));
    }
    log::info!("[VectorClient] Request to {} succeeded with status {}", url, status);
    Ok(())
}

pub async fn ensure_collection(state: &AppState, backend_url: &str) -> Result<()> {
    let url = format!("{}/api/v1/vector/ensure-collection", backend_url);
    let payload = EnsureCollectionPayload {
        base: VectorBase {
            database: VECTOR_DB_NAME,
            collection: VECTOR_COLLECTION_NAME,
        },
    };
    post(state, &url, &payload).await
}

pub async fn add(state: &AppState, backend_url: &str, payload: &AddPayload<'_>) -> Result<()> {
    let url = format!("{}/api/v1/vector/add", backend_url);
    post(state, &url, payload).await
}

pub async fn query(
    state: &AppState,
    backend_url: &str,
    payload: &QueryPayload<'_>,
) -> Result<QueryResponse> {
    let url = format!("{}/api/v1/vector/query", backend_url);
    log::info!("[VectorClient] Sending POST request to: {}", url);
    let res = state.http_client.post(&url).json(payload).send().await?;
    let status = res.status();

    if !status.is_success() {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".into());
        log::error!("[VectorClient] Query to {} failed with status {}: {}", url, status, error_text);
        return Err(AppError::VectorService(format!(
            "Failed query to {}: {}",
            url, error_text
        )));
    }
    log::info!("[VectorClient] Query to {} succeeded with status {}", url, status);
    res.json::<QueryResponse>().await.map_err(|e| e.into())
}

pub async fn delete(
    state: &AppState,
    backend_url: &str,
    payload: &DeletePayload<'_>,
) -> Result<()> {
    let url = format!("{}/api/v1/vector/delete", backend_url);
    post(state, &url, payload).await
}

pub async fn update_metadata(
    state: &AppState,
    backend_url: &str,
    payload: &UpdateMetadataPayload<'_>,
) -> Result<()> {
    let url = format!("{}/api/v1/vector/update-metadata", backend_url);
    post(state, &url, payload).await
}

pub async fn clear_collection(
    state: &AppState,
    backend_url: &str,
) -> Result<()> {
    let url = format!("{}/api/v1/vector/clear-collection", backend_url);
    let payload = ClearCollectionPayload {
        base: VectorBase {
            database: VECTOR_DB_NAME,
            collection: VECTOR_COLLECTION_NAME,
        },
    };
    post(state, &url, &payload).await
}

pub async fn count(state: &AppState, backend_url: &str) -> Result<CountResponse> {
    let url = format!("{}/api/v1/vector/count", backend_url);
    let payload = CountPayload {
        base: VectorBase {
            database: VECTOR_DB_NAME,
            collection: VECTOR_COLLECTION_NAME,
        },
    };
    let res = state.http_client.post(&url).json(&payload).send().await?;
    let status = res.status();
    if !status.is_success() {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".into());
        return Err(AppError::VectorService(format!(
            "Failed count from {}: {}",
            url, error_text
        )));
    }
    res.json::<CountResponse>().await.map_err(|e| e.into())
}