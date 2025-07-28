// src-tauri/src/services/proxy_types.rs
use crate::database::models::{ApiConfig, ApiProvider, ChatMessageContentPart};
use crate::knowledge_base::models::KnowledgeSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProxyChatPayload<'a> {
    pub model: &'a str,
    pub messages: Vec<ProxyMessage<'a>>,
    pub stream: bool,
    pub provider_config: &'a ApiProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_selection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_config: Option<&'a ApiConfig>,
}

#[derive(Serialize)]
pub struct ProxyMessage<'a> {
    pub role: String,
    pub content: &'a [ChatMessageContentPart],
}

#[derive(Serialize)]
pub struct ProxyEmbeddingPayload<'a> {
    pub model: &'a str,
    pub input: &'a [String],
    pub provider_config: &'a ApiProvider,
}

#[derive(Deserialize, Debug)]
pub struct ProxyStreamChunk {
    pub choices: Vec<StreamChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<KnowledgeSource>>,
}

#[derive(Deserialize, Debug)]
pub struct StreamChoice {
    pub delta: StreamDelta,
}

#[derive(Deserialize, Debug)]
pub struct StreamDelta {
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
}

#[derive(Deserialize)]
pub struct ProxyResponse {
    pub choices: Vec<ResponseChoice>,
}

#[derive(Deserialize)]
pub struct ResponseChoice {
    pub message: ResponseMessage,
}

#[derive(Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Deserialize)]
pub struct EmbeddingData {
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
}