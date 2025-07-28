// frontend/src/lib/api.ts
import { invoke as tauriInvoke, event as tauriEvent } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/shell';
import { mockInvoke, mockEvent } from './mock-api';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { Settings, KnowledgeSource, ChatMessage, IntentSuggestion, KnowledgeNote, KnowledgeGraphData, DirectoryPickerResponse, FileNode, DynamicTool, ConfiguredTool, OverlayContext, Conversation, Workflow, WorkflowExecution, ClipboardItem, DashboardStats, AgentTask, AgentTaskStep, ApiConfig, ToolRuntime, OnlineKnowledgeBase, Integration, IntegrationTemplate, ApiCallStatsTimeseries, CreationArtifact, CreationType, ApiProvider } from '../types';
import { useToasts } from '../composables/useToasts';
import { appWindow } from '@tauri-apps/api/window';
import { getClient, Body } from '@tauri-apps/api/http';

const isTauri = !!window.__TAURI_IPC__;

const invoke = isTauri ? tauriInvoke : mockInvoke;
const listen = isTauri ? tauriEvent.listen : mockEvent.listen;

async function invokeWithFeedback<T>(command: string, args?: any): Promise<T | null> {
  const { error } = useToasts();
  try {
    console.log(`[API] Invoking command: '${command}' with args:`, args);
    const result = await invoke<T>(command, args);
    console.log(`[API] Command '${command}' succeeded.`);
    return result;
  } catch (err) {
    const errorMessage = typeof err === 'string' ? err : `An unknown error occurred for command: ${command}.`;
    error(errorMessage);
    console.error(`[API] Command '${command}' failed:`, err);
    return null;
  }
}

// --- System & Shell ---
export const openExternalLink = async (url: string) => {
  try {
    if (isTauri) {
      await open(url);
    } else {
      window.open(url, '_blank');
    }
  } catch (e) {
    console.error(`Failed to open external link: ${url}`, e);
    const { error } = useToasts();
    error(`Could not open link: ${e}`);
  }
};

// --- App Info ---
export const getAppInfo = () => invokeWithFeedback<{ name: string; version: string }>('get_app_info');
export const getReadmeContent = () => invokeWithFeedback<string>('get_readme_content');

// --- Dashboard ---
export const getDashboardStats = () => invokeWithFeedback<DashboardStats>('get_dashboard_stats');
export const getApiCallStats = (timeRange: 'day' | 'week' | 'month' | 'all') => invokeWithFeedback<ApiCallStatsTimeseries>('get_api_call_stats', { timeRange });

// --- Audio ---
export const getTtsAudio = async (text: string, settings: Settings): Promise<Blob | null> => {
    const { error } = useToasts();
    try {
        const backendUrl = settings.execution.backendUrl;
        const client = await getClient();
        const response = await client.post<ArrayBuffer>(`${backendUrl}/api/v1/audio/speech`, Body.json({ input: text, api_config: settings.apiConfig }), {
            responseType: 2 // ArrayBuffer
        });
        if (response.ok) {
            return new Blob([new Uint8Array(response.data)], { type: 'audio/mpeg' });
        } else {
            error(`TTS generation failed: ${response.status}`);
            return null;
        }
    } catch (e) {
        error(`TTS request failed: ${e}`);
        return null;
    }
};


// --- Window Management ---
export const show_window = (label: 'main' | 'copilot', focus: boolean = true) => invokeWithFeedback<void>('show_window', { label, focus });
export const hide_window = (label: 'main' | 'copilot') => {
  if (isTauri) {
    appWindow.hide();
  } else {
    console.log(`[Mock] Hide window: ${label}`);
  }
};

// --- Autostart ---
export const enableAutostart = () => invokeWithFeedback<void>('enable_autostart');
export const disableAutostart = () => invokeWithFeedback<void>('disable_autostart');
export const isAutostartEnabled = () => invokeWithFeedback<boolean>('is_autostart_enabled');

// --- Assets ---
export const saveTempAssetWithHash = (base64Data: string, hash: string, extension: string) => invokeWithFeedback<string>('save_temp_asset_with_hash', { base64Data, hash, extension });
export const readFileAsBase64 = (path: string) => invokeWithFeedback<string>('read_file_as_base64', { path });

// --- File System & KB ---
export const readFileContent = (path: string) => invokeWithFeedback<string>('read_file_content', { path });
export const saveFileContent = (path: string, content: string) => invokeWithFeedback<void>('save_file_content', { path, content });
export const listFilesInDirectory = (path: string) => invokeWithFeedback<FileNode[]>('list_files_in_directory', { path });
export const createUniqueFile = (dirPath: string) => invokeWithFeedback<string>('create_unique_file', { dirPath });
export const renameFile = (oldPath: string, newPath: string) => invokeWithFeedback<void>('rename_file', { oldPath, newPath });
export const moveFile = (oldPath: string, newParentDir: string) => invokeWithFeedback<void>('move_file', { oldPath, newParentDir });
export const deleteFile = (path: string) => invokeWithFeedback<void>('delete_file', { path });
export const startIndexing = (path: string) => invokeWithFeedback<void>('start_indexing', { path });
export const searchLocalKb = (query: string, topK: number, scoreThreshold: number) => invokeWithFeedback<KnowledgeSource[]>('search_local_kb', { query, topK, scoreThreshold });
export const searchOnlineKb = (kbId: string, query: string, topK: number, scoreThreshold: number) => invokeWithFeedback<KnowledgeSource[]>('search_online_kb', { kbId, query, topK, scoreThreshold });
export const findFileInKb = (query: string) => invokeWithFeedback<string[]>('find_file_in_kb', { query });
export const openDirectoryPicker = () => invokeWithFeedback<DirectoryPickerResponse>('open_directory_picker');
export const removeIndexedDirectory = (path: string) => invokeWithFeedback<void>('remove_indexed_directory', { path });
export const getNoteDetails = (noteId: string) => invokeWithFeedback<KnowledgeNote | null>('get_note_details', { noteId });
export const getKnowledgeGraphData = () => invokeWithFeedback<KnowledgeGraphData>('get_knowledge_graph_data');
export const rebuildKnowledgeGraph = () => invokeWithFeedback<void>('rebuild_knowledge_graph');
export const clearKnowledgeBase = () => invokeWithFeedback<void>('clear_knowledge_base');
export const rebuildIndex = (path: string) => invokeWithFeedback<void>('rebuild_index', { path });
export const batchConvertToMarkdown = (inputDir: string, outputDir: string) => invokeWithFeedback<void>('batch_convert_to_markdown', { inputDir, outputDir });
export const listOnlineKbs = () => invokeWithFeedback<OnlineKnowledgeBase[]>('list_online_kbs');
export const addOnlineKb = (kb: OnlineKnowledgeBase) => invokeWithFeedback<void>('add_online_kb', { kb });
export const updateOnlineKb = (kb: OnlineKnowledgeBase) => invokeWithFeedback<void>('update_online_kb', { kb });
export const deleteOnlineKb = (id: string) => invokeWithFeedback<void>('delete_online_kb', { id });

// --- Settings ---
export const getUserSettings = () => invokeWithFeedback<Settings>('get_user_settings');
export const updateUserSettings = (settings: Settings) => invokeWithFeedback<void>('update_user_settings', { settings });

// --- Chat, Creation & Agent ---
export const createConversation = (sessionType: 'chat' | 'creation', title?: string) => invokeWithFeedback<Conversation>('create_conversation', { sessionType, title });
export const processChatMessage = (userMessage: ChatMessage, aiMessageId: string, apiConfig: ApiConfig) => invokeWithFeedback<void>('process_chat_message', { userMessage, aiMessageId, apiConfig });
export const processAgenticInstruction = (conversationId: string, instruction: string, apiConfig: ApiConfig, mode: 'plan' | 'explore' | 'write' | 'research' | 'debate', knowledgeBaseSelection: string) => invokeWithFeedback<{ task_id: string }>('process_agentic_instruction', { conversationId, instruction, apiConfig, mode, knowledgeBaseSelection });
export const stopChatGeneration = (messageId: string) => invokeWithFeedback<void>('stop_chat_generation', { messageId });
export const stopAgentTask = (taskId: string) => invokeWithFeedback<void>('stop_agent_task', { taskId });
export const restartAgentTask = (taskId: string) => invokeWithFeedback<void>('restart_agent_task', { taskId });
export const resumeWriteTask = (taskId: string, elaboration: any, plan: any) => invokeWithFeedback<void>('resume_write_task', { taskId, elaboration, plan });
export const getConversationHistory = (conversationId: string) => invokeWithFeedback<ChatMessage[]>('get_conversation_history', { conversationId });
export const listConversations = () => invokeWithFeedback<Conversation[]>('list_conversations');
export const getConversationWithArtifacts = (conversationId: string) => invokeWithFeedback<Conversation>('get_conversation_with_artifacts', { conversationId });
export const deleteConversation = (conversationId: string) => invokeWithFeedback<void>('delete_conversation', { conversationId });
export const deleteMessage = (messageId: string) => invokeWithFeedback<void>('delete_message', { messageId });
export const clearAllConversations = () => invokeWithFeedback<void>('clear_all_conversations');
export const updateConversationTitle = (conversationId: string, newTitle: string) => invokeWithFeedback<void>('update_conversation_title', { conversationId, newTitle });
export const updateMessageContent = (messageId: string, newContent: string) => invokeWithFeedback<void>('update_message_content', { messageId, newContent });
export const getTaskStatus = (taskId: string) => invokeWithFeedback<string>('get_task_status', { taskId });
export const linkAgentTaskToMessage = (messageId: string, agentTaskId: string) => invokeWithFeedback<void>('link_agent_task_to_message', { messageId, agentTaskId });
export const saveMessage = (message: ChatMessage) => invokeWithFeedback<void>('save_message', { message });
export const generateArtifact = (conversationId: string, creationType: CreationType, prompt: string, modelName: string, params: Record<string, any>, provider: ApiProvider) => invokeWithFeedback<CreationArtifact>('generate_artifact', { conversationId, creationType, prompt, modelName, params, provider });
export const generateResearchNodeContent = (taskId: string, nodeId: string) => invokeWithFeedback<void>('generate_research_node_content', { taskId, nodeId });
export const refineAgentTaskSection = (taskId: string, nodeId: string, prompt: string, model: string) => invokeWithFeedback<void>('refine_agent_task_section', { taskId, nodeId, prompt, model });

// --- System ---
export const getActiveWindowInfo = () => invokeWithFeedback<{ appName: string; windowTitle: string } | null>('get_active_window_info');
export const getClipboardContent = () => invokeWithFeedback<OverlayContext | null>('get_clipboard_content');
export const showInFolder = (path: string) => invokeWithFeedback<void>('show_in_folder', { path });
export const pasteText = (text: string) => invokeWithFeedback<void>('paste_text', { text });

// --- Intent Engine ---
export const getIntentSuggestions = (context: { contentType: string; content: any; sourceApp?: string }) =>
  invokeWithFeedback<IntentSuggestion[]>('get_intent_suggestions', { context });

// --- Tools & Code Execution ---
export const executeTool = (toolName: string, params: Record<string, any>) => invokeWithFeedback<string>('execute_tool', { payload: { toolName, params } });
export const listTools = () => invokeWithFeedback<DynamicTool[]>('list_tools');
export const listConfiguredTools = () => invokeWithFeedback<ConfiguredTool[]>('list_configured_tools');
export const saveConfiguredTool = (tool: ConfiguredTool) => invokeWithFeedback<void>('save_configured_tool', { tool });
export const deleteConfiguredTool = (id: string) => invokeWithFeedback<void>('delete_configured_tool', { id });
export const getFavoriteTools = () => invokeWithFeedback<ConfiguredTool[]>('get_favorite_tools');
export const toggleToolFavorite = (id: string, isFavorite: boolean) => invokeWithFeedback<void>('toggle_tool_favorite', { id, isFavorite });
export const executePythonCode = (executionId: string, code: string) => invokeWithFeedback<void>('execute_python_code', { executionId, code });
export const saveToolScript = (scriptName: string, scriptContent: string) => invokeWithFeedback<string>('save_tool_script', { scriptName, scriptContent });
export const executeShellCommand = (command: string) => invokeWithFeedback<string>('execute_shell_command', { command });
export const executeGenericCode = (runtime: ToolRuntime, code: string) => invokeWithFeedback<string>('execute_generic_code', { runtime, code });
export const setupTaskWorkspace = (taskId: string) => invokeWithFeedback<string>('setup_task_workspace', { taskId });
export const writeFileToTaskDir = (filename: string, content: string) => invokeWithFeedback<void>('write_file_to_task_dir', { filename, content });

// --- Backend Management ---
export const checkBackendStatus = (pythonPath: string) => invokeWithFeedback<{ status: 'unknown' | 'not_installed' | 'installing' | 'stopped' | 'running' | 'error' }>('check_backend_status', { pythonPath });
export const installBackendService = () => invokeWithFeedback<void>('install_backend_service');
export const startBackendService = (pythonPath: string) => invokeWithFeedback<void>('start_backend_service', { pythonPath });
export const stopBackendService = () => invokeWithFeedback<void>('stop_backend_service');

// --- Workflows (Legacy) ---
export const getAvailableWorkflows = () => invokeWithFeedback<Workflow[]>('get_available_workflows');
export const startWorkflow = (flowId: string, params: Record<string, any>) => invokeWithFeedback<string>('start_workflow', { flowId, params });
export const getWorkflowHistory = () => invokeWithFeedback<WorkflowExecution[]>('get_workflow_history');

// --- Backup & Restore ---
export const exportData = (filePath: string) => invokeWithFeedback<void>('export_data', { filePath });
export const importData = (filePath: string) => invokeWithFeedback<void>('import_data', { filePath });

// --- Clipboard ---
export const getClipboardHistory = () => invokeWithFeedback<ClipboardItem[]>('get_clipboard_history');
export const clearClipboardHistory = () => invokeWithFeedback<void>('clear_clipboard_history');
export const deleteClipboardItem = (id: string) => invokeWithFeedback<void>('delete_clipboard_item', { id });
export const deleteClipboardItems = (ids: string[]) => invokeWithFeedback<void>('delete_clipboard_items', { ids });
export const toggleClipboardItemPinned = (id: string, isPinned: boolean) => invokeWithFeedback<void>('toggle_clipboard_item_pinned', { id, isPinned });

// --- Integrations ---
export const listIntegrations = () => invokeWithFeedback<Integration[]>('list_integrations');
export const addIntegration = (integration: Integration) => invokeWithFeedback<Integration>('add_integration', { integration });
export const deleteIntegration = (id: string) => invokeWithFeedback<void>('delete_integration', { id });
export const getIntegrationTemplates = () => invokeWithFeedback<IntegrationTemplate[]>('get_integration_templates');

// --- Event Listeners ---
export const onCopilotContextUpdate = (handler: (payload: OverlayContext) => void): Promise<UnlistenFn> => listen('copilot-context-update', (event) => handler(event.payload as OverlayContext));
export const onCopilotShown = (handler: () => void): Promise<UnlistenFn> => listen('copilot-shown', () => handler());
export const onChatMessageChunk = (handler: (payload: { messageId: string; chunk: string }) => void): Promise<UnlistenFn> => listen('stream-chunk', (event) => handler(event.payload as any));
export const onChatMessageEnd = (handler: (payload: { messageId: string; finalMessage: any }) => void): Promise<UnlistenFn> => listen('stream-end', (event) => handler(event.payload as any));
export const onIndexingProgress = (handler: (payload: { file: string; progress: number }) => void): Promise<UnlistenFn> => listen('indexing-progress', (event) => handler(event.payload as any));
export const onConversationCreated = (handler: (payload: Conversation) => void): Promise<UnlistenFn> => listen('conversation-created', (event) => handler(event.payload as Conversation));
export const onConversationTitleUpdated = (handler: (payload: Conversation) => void): Promise<UnlistenFn> => listen('conversation-title-updated', (event) => handler(event.payload as Conversation));
export const onInstructionComplete = (handler: (payload: ChatMessage) => void): Promise<UnlistenFn> => listen('instruction-complete', (event) => handler(event.payload as ChatMessage));
export const onToolOutput = (handler: (payload: { taskId: string; chunk: string }) => void): Promise<UnlistenFn> => listen('tool-output', (event) => handler(event.payload as any));
export const onToolComplete = (handler: (payload: { taskId: string; payload: string }) => void): Promise<UnlistenFn> => listen('tool-complete', (event) => handler(event.payload as any));
export const onToolError = (handler: (payload: { taskId: string; payload: string }) => void): Promise<UnlistenFn> => listen('tool-error', (event) => handler(event.payload as any));
export const onCodeExecutionOutput = (handler: (payload: { executionId: string; chunk: string }) => void): Promise<UnlistenFn> => listen('code-execution-output', (event) => handler(event.payload as any));
export const onCodeExecutionComplete = (handler: (payload: { executionId: string; status: 'success' | 'error'; error?: string }) => void): Promise<UnlistenFn> => listen('code-execution-complete', (event) => handler(event.payload as any));
export const onWorkflowProgress = (handler: (payload: any) => void): Promise<UnlistenFn> => listen('workflow-progress', (event) => handler(event.payload));
export const onWorkflowComplete = (handler: (payload: any) => void): Promise<UnlistenFn> => listen('workflow-complete', (event) => handler(event.payload));
export const onClipboardUpdate = (handler: (payload: OverlayContext) => void): Promise<UnlistenFn> => listen('clipboard-updated', (event) => handler(event.payload as OverlayContext));
export const onClipboardItemAdded = (handler: (payload: ClipboardItem) => void): Promise<UnlistenFn> => listen('clipboard-item-added', (event) => handler(event.payload as ClipboardItem));
export const onBackendInstallProgress = (handler: (payload: { progress: number; message: string }) => void): Promise<UnlistenFn> => listen('backend-install-progress', (event) => handler(event.payload as any));
export const onConversionProgress = (handler: (payload: { progress: number; message: string; error?: boolean }) => void): Promise<UnlistenFn> => listen('conversion-progress', (event) => handler(event.payload as any));
export const onTaskStarted = (handler: (payload: AgentTask) => void): Promise<UnlistenFn> => listen('task-started', (event) => handler(event.payload as AgentTask));
export const onPlanUpdated = (handler: (payload: any) => void): Promise<UnlistenFn> => listen('plan-updated', (event) => handler(event.payload as any));
export const onTaskCompleted = (handler: (payload: ChatMessage) => void): Promise<UnlistenFn> => listen('task-completed', (event) => handler(event.payload as ChatMessage));
export const onArtifactCreated = (handler: (payload: CreationArtifact) => void): Promise<UnlistenFn> => listen('artifact-created', (event) => handler(event.payload as CreationArtifact));