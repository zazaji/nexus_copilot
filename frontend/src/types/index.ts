// frontend/src/types/index.ts
export type ChatMessageContentPart =
  | { type: 'text'; text: string }
  | { type: 'image_url'; image_url: { url: string } };

export interface ChatMessageAttachment {
  name: string;
  url: string;
}

export interface ChatMessage {
  id: string;
  conversationId: string;
  role: 'user' | 'ai';
  content: ChatMessageContentPart[];
  timestamp: number;
  sources?: KnowledgeSource[];
  error?: string;
  suggestions?: string[];
  model?: string; // e.g., "provider-id::model-name" or "agent" or "agent-starting"
  knowledgeBaseSelection?: string; // 'none', 'all', or a specific path
  isExecuting?: boolean;
  executionOutput?: string;
  thinkingProcess?: string[];
  agentTaskId?: string; // Link to the agent task
  attachments?: ChatMessageAttachment[];
  agentTask?: AgentTask; // The entire task state is now part of the message
  integrationTaskId?: string;
  integrationTask?: IntegrationTask;
  isThinking?: boolean; // Transient state for stream parsing
}

export interface researchOutlineNode {
  id: string; // e.g., "1.1", "2.3.1"
  sub_goal: string;
  status: 'pending' | 'writing' | 'completed';
  steps?: researchOutlineNode[];
  word_count?: number;
}

export interface Refinement {
  prompt: string;
  content: string;
  timestamp: number;
}

export interface ContentNode {
  current: string;
  history?: Refinement[];
}

export interface AgentTask {
  id: string;
  conversationId: string;
  userGoal: string;
  status: 'running' | 'completed' | 'failed' | 'planning' | 'awaiting_user_input';
  mode: 'plan' | 'explore' | 'write' | 'research' | 'debate';
  createdAt: number;
  updatedAt?: number;
  steps: AgentTaskStep[];
  plan?: any; // Can be simple plan, research outline, or debate structure
  finalReport?: string;
  logFileUrl?: string;
  reportFileUrl?: string;
  researchContent?: Record<string, ContentNode>; // Key is outline node ID
}

export interface AgentTaskStep {
  id: string;
  taskId: string;
  step_index: number;
  thought?: string;
  action: string;
  action_input: any;
  observation?: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  result?: string;
}

export interface IntegrationTask {
  id: string;
  conversationId: string;
  integrationName: string;
  status: 'running' | 'completed' | 'failed';
  createdAt: number;
  updatedAt?: number;
  steps: IntegrationTaskStep[];
  finalReport?: string;
}

export interface IntegrationTaskStep {
  id: string;
  taskId: string;
  step_index: number;
  description: string;
  details?: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
}

export interface Conversation {
  id: string;
  title: string;
  created_at: number;
  sessionType: 'chat' | 'creation';
  artifacts?: CreationArtifact[];
}

export type CreationType = 'image' | 'audio' | 'video' | 'text';

export interface CreationArtifact {
  id: string;
  type: CreationType;
  filePath: string; // Relative path within the session directory
  fileUrl?: string; // Tauri asset URL for frontend display
  prompt: string;
  modelUsed: string;
  createdAt: number;
  metadata?: Record<string, any>; // e.g., image dimensions, audio duration
}

export interface CreationSession {
  id: string;
  title: string;
  creationType: CreationType;
  createdAt: number;
  artifacts: CreationArtifact[]; // Loaded from the filesystem at runtime
}


export interface KnowledgeSource {
  id: string;
  file_path: string;
  source_name: string;
  content_snippet: string;
  score: number;
}

export interface KnowledgeNote {
  id: string;
  title: string;
  content: string;
  created_at: number;
  updated_at: number;
  backlinks: { note_id: string; note_title: string }[];
  outgoing_links: { note_id: string; note_title: string }[];
}

export interface KnowledgeGraphData {
    nodes: { id: string; label: string; type: 'real' | 'ghost' }[];
    links: { source: string; target: string; }[];
}

export interface ModelEndpoint {
  providerId: string;
  modelName: string;
}

export type ModelCapability = 'chat' | 'vision' | 'image_gen' | 'embedding' | 'tts' | 'video_gen';

export interface ModelInfo {
  name: string;
  capabilities: ModelCapability[];
  maxTokens: number | null;
}

export interface ApiProvider {
  id: string;
  name: string;
  baseUrl: string;
  apiKey: string;
  models: ModelInfo[];
  proxy?: string;
}

export interface ModelAssignments {
  chat: ModelEndpoint | null;
  suggestion: ModelEndpoint | null;
  vision: ModelEndpoint | null;
  imageGen: ModelEndpoint | null;
  embedding: ModelEndpoint | null;
  tts: ModelEndpoint | null;
  videoGen: ModelEndpoint | null;
  plan: ModelEndpoint | null;
  debatePro: ModelEndpoint | null;
  debateCon: ModelEndpoint | null;
  debateJudge: ModelEndpoint | null;
  write: ModelEndpoint | null;
  refine: ModelEndpoint | null;
}

export interface OtherApiKeys {
  tavily?: string;
  bing?: string;
}

export interface OnlineKnowledgeBase {
  id: string;
  name: string;
  url: string;
  token: string;
}

export interface KnowledgeBaseSettings {
  indexedDirectories: string[];
  scriptsDirectories: string[];
  defaultSaveDirectory: string | null;
  topK: number;
  scoreThreshold: number;
  defaultInternetSearchEngine: 'tavily' | 'bing';
}

export interface ExecutionSettings {
  pythonPath: string;
  nodePath: string;
  workingDirectory: string;
  autoStartBackend: boolean;
  backendUrl: string;
}

export interface ShortcutsSettings {
  toggleCopilot: string;
  showMainWindow: string;
}

export interface AppearanceSettings {
  theme: 'light' | 'dark' | 'system';
  language: 'system' | 'en' | 'zh';
  copilotAutoHideDelay: number;
  editorFontSize: number;
}

export interface Settings {
  apiConfig: ApiConfig;
  knowledgeBase: KnowledgeBaseSettings;
  appearance: AppearanceSettings;
  execution: ExecutionSettings;
  shortcuts: ShortcutsSettings;
}

export interface ApiConfig {
  providers: ApiProvider[];
  assignments: ModelAssignments;
  keys: OtherApiKeys;
  onlineKbs?: OnlineKnowledgeBase[];
  knowledgeBase?: KnowledgeBaseSettings;
  execution?: ExecutionSettings;
  appearance?: AppearanceSettings;
}

export interface DashboardWidget {
    id: string;
    type: 'calendar';
    x: number;
    y: number;
    w: number;
    h: number;
}

export interface DashboardStats {
    notesCount: number;
    vectorsCount: number;
    conversationsCount: number;
    toolsCount: number;
    dbSize: number;
    vectorDbSize: number;
    backendDbSize: number;
}

export interface ApiCallStatPeriod {
  date: string;
  stats: Record<string, Record<string, number>>;
}

export type ApiCallStatsTimeseries = ApiCallStatPeriod[];

export interface Workflow {
  id: string;
  name: string;
  description: string;
  icon: string;
  parameters: WorkflowParameter[];
  requiresInternet: boolean;
}

export interface WorkflowParameter {
  name: string;
  label: string;
  type: 'text' | 'textarea' | 'number';
  defaultValue?: string;
  required: boolean;
}

export interface WorkflowExecution {
  id: string;
  workflowId: string;
  status: 'running' | 'completed' | 'failed' | 'pending';
  startedAt: number;
  finishedAt?: number;
  parameters: Record<string, any>;
  steps: WorkflowStep[];
  result?: any;
}

export interface WorkflowStep {
  name: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  details?: string;
}

export type ContentType = 'text' | 'image' | 'files';

export interface ClipboardItem {
  id: string;
  type: 'text' | 'image' | 'files';
  content: any;
  timestamp: number;
  isPinned: boolean;
}

export interface OverlayContext {
  contentType: string;
  content: any;
  sourceApp?: string;
}

export interface IntentSuggestion {
  action: string;
  label: string;
  icon: string;
}

export interface DirectoryPickerResponse {
    success: boolean;
    path: string | null;
    error: string | null;
}

export interface FileNode {
  key: string;
  title: string;
  isLeaf: boolean;
  children?: FileNode[];
  fileType?: 'text' | 'pdf' | 'doc' | 'ppt';
}

export type ToolParameterType = 'text' | 'textarea';

export interface ToolParameter {
  name: string;
  label: string;
  paramType: ToolParameterType;
  defaultValue: string;
  required: boolean;
}

export type ToolInputSource = 'user_input' | 'clipboard' | 'chat_selection';
export type ToolOutputHandling = 'raw_text' | 'markdown' | 'to_clipboard' | 'new_chat';
export type ToolRuntime = 'shell' | 'python' | 'node' | 'webhook';

export interface ConfiguredTool {
  id: string;
  name: string;
  description: string;
  scriptPath?: string;
  webhookUrl?: string;
  webhookMethod?: 'GET' | 'POST' | 'PUT' | 'PATCH';
  webhookHeaders?: string; // JSON string for headers
  webhookBodyTemplate?: string; // JSON string
  inputSchema?: string; // JSON Schema string
  runtime: ToolRuntime;
  parameters: ToolParameter[];
  showInCopilot: boolean;
  isFavorite: boolean;
  inputSource: ToolInputSource;
  requiresAiPreProcessing: boolean;
  preProcessingPrompt: string;
  outputHandling: ToolOutputHandling;
  requiresAiPostProcessing: boolean;
  postProcessingPrompt: string;
}

export interface DynamicTool {
  id: string;
  name: string;
  description: string;
  script_path: string;
  runtime: ToolRuntime;
}

export interface Integration {
  id: string;
  name: string;
  service: string;
  configJson: string;
  isEnabled: boolean;
  createdAt: number;
}

export interface IntegrationTemplate {
  id: string;
  name: string;
  description: string;
  serviceType: 'inbound_webhook';
}

export interface ConversationConfig {
  model: string | null;
  knowledge: string;
}