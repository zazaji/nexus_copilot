// src-tauri/src/database/models.rs
use crate::knowledge_base;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTokens")]
    pub max_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiProvider {
    pub id: String,
    pub name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub models: Vec<ModelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelEndpoint {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ModelAssignments {
    pub chat: Option<ModelEndpoint>,
    pub suggestion: Option<ModelEndpoint>,
    pub vision: Option<ModelEndpoint>,
    #[serde(rename = "imageGen")]
    pub image_gen: Option<ModelEndpoint>,
    pub embedding: Option<ModelEndpoint>,
    pub tts: Option<ModelEndpoint>,
    #[serde(rename = "videoGen")]
    pub video_gen: Option<ModelEndpoint>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OtherApiKeys {
    #[serde(default)]
    pub tts: String,
    #[serde(rename = "imageGen", default)]
    pub image_gen: String,
    #[serde(default)]
    pub search: String,
    #[serde(default)]
    pub tavily: String,
    #[serde(default)]
    pub bing: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnlineKnowledgeBase {
    pub id: String,
    pub name: String,
    pub url: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseSettings {
    #[serde(default)]
    pub indexed_directories: Vec<String>,
    #[serde(default)]
    pub scripts_directories: Vec<String>,
    #[serde(default)]
    pub default_save_directory: Option<String>,
    #[serde(default = "default_top_k")]
    pub top_k: u32,
    #[serde(default = "default_score_threshold")]
    pub score_threshold: f32,
    #[serde(default = "default_search_engine")]
    pub default_internet_search_engine: String,
}

fn default_top_k() -> u32 { 5 }
fn default_score_threshold() -> f32 { 0.6 }
fn default_search_engine() -> String { "tavily".to_string() }

impl Default for KnowledgeBaseSettings {
    fn default() -> Self {
        Self {
            indexed_directories: Vec::new(),
            scripts_directories: Vec::new(),
            default_save_directory: None,
            top_k: default_top_k(),
            score_threshold: default_score_threshold(),
            default_internet_search_engine: default_search_engine(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
    pub theme: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_copilot_delay")]
    pub copilot_auto_hide_delay: u32,
    #[serde(default = "default_font_size")]
    pub editor_font_size: u32,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: default_language(),
            copilot_auto_hide_delay: default_copilot_delay(),
            editor_font_size: default_font_size(),
        }
    }
}

fn default_language() -> String { "system".to_string() }
fn default_copilot_delay() -> u32 { 10 }
fn default_font_size() -> u32 { 3 }


fn default_backend_url() -> String {
    "http://127.0.0.1:8008".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSettings {
    #[serde(default)]
    pub python_path: String,
    #[serde(default)]
    pub node_path: String,
    #[serde(default)]
    pub working_directory: String,
    #[serde(default)]
    pub auto_start_backend: bool,
    #[serde(default = "default_backend_url")]
    pub backend_url: String,
}

impl Default for ExecutionSettings {
    fn default() -> Self {
        Self {
            python_path: "".to_string(),
            node_path: "".to_string(),
            working_directory: "".to_string(),
            auto_start_backend: false,
            backend_url: default_backend_url(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutsSettings {
    #[serde(default = "default_toggle_copilot")]
    pub toggle_copilot: String,
    #[serde(default = "default_show_main_window")]
    pub show_main_window: String,
}

fn default_toggle_copilot() -> String { "CmdOrCtrl+Shift+C".to_string() }
fn default_show_main_window() -> String { "".to_string() }

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    #[serde(default)]
    pub providers: Vec<ApiProvider>,
    #[serde(default)]
    pub assignments: ModelAssignments,
    #[serde(default)]
    pub keys: OtherApiKeys,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_kbs: Option<Vec<OnlineKnowledgeBase>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<KnowledgeBaseSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<AppearanceSettings>,
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Settings {
    #[serde(rename = "apiConfig", default)]
    pub api_config: ApiConfig,
    #[serde(rename = "knowledgeBase", default)]
    pub knowledge_base: KnowledgeBaseSettings,
    #[serde(default)]
    pub appearance: AppearanceSettings,
    #[serde(default)]
    pub execution: ExecutionSettings,
    #[serde(default)]
    pub shortcuts: ShortcutsSettings,
}

impl Settings {
    pub fn new_default() -> Self {
        let default_provider_id = Uuid::new_v4().to_string();
        let default_chat_model = "gpt-4-turbo".to_string();
        let default_suggestion_model = "gpt-3.5-turbo".to_string();
        let default_embedding_model = "text-embedding-3-small".to_string();
        Self {
            api_config: ApiConfig {
                providers: vec![ApiProvider {
                    id: default_provider_id.clone(),
                    name: "Default OpenAI".to_string(),
                    base_url: "https://api.openai.com/v1".to_string(),
                    api_key: "".to_string(),
                    models: vec![
                        ModelInfo { name: default_chat_model.clone(), capabilities: vec!["chat".to_string()], max_tokens: Some(4096) },
                        ModelInfo { name: default_suggestion_model.clone(), capabilities: vec!["chat".to_string()], max_tokens: Some(4096) },
                        ModelInfo { name: default_embedding_model.clone(), capabilities: vec!["embedding".to_string()], max_tokens: None },
                    ],
                    proxy: None,
                }],
                assignments: ModelAssignments {
                    chat: Some(ModelEndpoint { provider_id: default_provider_id.clone(), model_name: default_chat_model }),
                    suggestion: Some(ModelEndpoint { provider_id: default_provider_id.clone(), model_name: default_suggestion_model }),
                    embedding: Some(ModelEndpoint { provider_id: default_provider_id, model_name: default_embedding_model }),
                    ..Default::default()
                },
                keys: OtherApiKeys::default(),
                online_kbs: None,
                knowledge_base: None,
                execution: None,
                appearance: None,
            },
            knowledge_base: KnowledgeBaseSettings::default(),
            appearance: AppearanceSettings::default(),
            execution: ExecutionSettings::default(),
            shortcuts: ShortcutsSettings::default(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatMessageContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub role: String,
    pub content: Vec<ChatMessageContentPart>,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<knowledge_base::models::KnowledgeSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "knowledgeBaseSelection", skip_serializing_if = "Option::is_none")]
    pub knowledge_base_selection: Option<String>,
    #[serde(rename = "isExecuting", default, skip_serializing_if = "Option::is_none")]
    pub is_executing: Option<bool>,
    #[serde(rename = "executionOutput", default, skip_serializing_if = "Option::is_none")]
    pub execution_output: Option<String>,
    #[serde(rename = "agentTaskId", default, skip_serializing_if = "Option::is_none")]
    pub agent_task_id: Option<String>,
    #[serde(rename = "agentTask", default, skip_serializing_if = "Option::is_none")]
    pub agent_task: Option<AgentTask>,
}

impl Default for ChatMessage {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            conversation_id: "".to_string(),
            role: "ai".to_string(),
            content: vec![],
            timestamp: Utc::now().timestamp_millis(),
            sources: None,
            error: None,
            suggestions: None,
            model: None,
            knowledge_base_selection: None,
            is_executing: None,
            execution_output: None,
            agent_task_id: None,
            agent_task: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub session_type: String, // 'chat' or 'creation'
    #[serde(default)]
    pub artifacts: Vec<CreationArtifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreationArtifact {
    pub id: String,
    pub conversation_id: String,
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub prompt: String,
    pub model_used: String,
    pub file_path: String,
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>, // JSON string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeNote {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub backlinks: Vec<NoteLinkInfo>,
    #[serde(rename = "outgoingLinks")]
    pub outgoing_links: Vec<NoteLinkInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteLinkInfo {
    pub note_id: String,
    pub note_title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeGraphData {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IntentContext {
    pub content_type: String,
    pub content: String,
    pub source_app: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct IntentSuggestion {
    pub action: String,
    pub label: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolParameterType {
    Text,
    Textarea,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ToolParameter {
    pub name: String,
    pub label: String,
    pub param_type: ToolParameterType,
    pub default_value: String,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolInputSource {
    UserInput,
    Clipboard,
    ChatSelection,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolOutputHandling {
    RawText,
    Markdown,
    ToClipboard,
    NewChat,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolRuntime {
    Shell,
    Python,
    Node,
    Webhook,
}

impl Default for ToolRuntime {
    fn default() -> Self { ToolRuntime::Python }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfiguredTool {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_headers: Option<String>, // JSON string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_body_template: Option<String>, // JSON string
    #[serde(default)]
    pub input_schema: Option<String>, // JSON Schema string
    #[serde(default)]
    pub runtime: ToolRuntime,
    pub parameters: Vec<ToolParameter>,
    #[serde(default)]
    pub show_in_copilot: bool,
    #[serde(default)]
    pub is_favorite: bool,
    #[serde(default = "default_input_source")]
    pub input_source: ToolInputSource,
    #[serde(default)]
    pub requires_ai_pre_processing: bool,
    #[serde(default)]
    pub pre_processing_prompt: String,
    #[serde(default = "default_output_handling")]
    pub output_handling: ToolOutputHandling,
    #[serde(default)]
    pub requires_ai_post_processing: bool,
    #[serde(default)]
    pub post_processing_prompt: String,
}

fn default_input_source() -> ToolInputSource { ToolInputSource::UserInput }
fn default_output_handling() -> ToolOutputHandling { ToolOutputHandling::RawText }


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardHistoryItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub content: String,
    pub timestamp: i64,
    pub id: String,
    #[serde(default)]
    pub is_pinned: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AgentTask {
    pub id: String,
    pub conversation_id: String,
    pub user_goal: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub steps: Vec<AgentTaskStep>,
    pub plan: Option<Vec<serde_json::Value>>,
    pub final_report: Option<String>,
    pub log_file_url: Option<String>,
    pub report_file_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub research_content: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanStep {
    pub sub_goal: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentTaskStep {
    pub id: String,
    pub task_id: String,
    pub step_index: i32,
    pub thought: Option<String>,
    pub action: String,
    pub action_input: String,
    pub observation: Option<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationTask {
    pub id: String,
    pub conversation_id: String,
    pub integration_name: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub steps: Vec<IntegrationTaskStep>,
    pub final_report: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationTaskStep {
    pub id: String,
    pub task_id: String,
    pub step_index: i32,
    pub description: String,
    pub details: Option<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiCallStatPeriod {
    pub date: String,
    pub stats: HashMap<String, HashMap<String, i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Integration {
    pub id: String,
    pub name: String,
    pub service: String,
    #[serde(rename = "configJson")]
    pub config_json: String,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_type: String,
}