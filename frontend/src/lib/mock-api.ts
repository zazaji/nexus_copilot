// frontend/src/lib/mock-api.ts
import type { Settings, DirectoryPickerResponse, ChatMessage, KnowledgeSource, FileNode, DynamicTool, ConfiguredTool, Conversation, IntegrationTemplate } from '../types';
import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event';
import { v4 as uuidv4 } from 'uuid';
import { useToasts } from '../composables/useToasts';
import { useChatStore } from '../stores/chat';

console.log(
  '%c[DEV] Using Mock API for browser development.',
  'background: #222; color: #bada55; padding: 2px 4px; border-radius: 3px;'
);

// --- Mock Event System ---
const eventListeners: Record<string, Map<string, EventCallback<any>>> = {};

function mockEmit(event: string, payload: any) {
  if (eventListeners[event]) {
    eventListeners[event].forEach(handler => handler({ event, payload, windowLabel: 'main' }));
  }
}

async function mockListen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn> {
  if (!eventListeners[event]) {
    eventListeners[event] = new Map();
  }
  const id = uuidv4();
  eventListeners[event].set(id, handler);
  console.log(`[Mock Listen] Registered for event: ${event}`);
  return () => {
    eventListeners[event]?.delete(id);
    console.log(`[Mock Listen] Unlistened from event: ${event}`);
  };
}

export const mockEvent = {
  listen: mockListen,
};

// --- Mock Invoke Logic ---

async function simulateStreamingResponse(message: ChatMessage) {
  const chatStore = useChatStore();
  const conversation = chatStore.conversations[message.conversationId];

  // Find the placeholder AI message ID
  const aiMessage = conversation?.slice().reverse().find(m => m.role === 'ai');
  const aiMessageId = aiMessage?.id;

  if (!aiMessageId) {
    console.error("[Mock Error] Could not find placeholder AI message to stream to.");
    return;
  }

  const response = `This is a simulated streaming response. You asked: "${(message.content.find(p => p.type === 'text') as any)?.text}".`;
  const chunks = response.split(' ');

  let i = 0;
  const interval = setInterval(() => {
    if (i < chunks.length) {
      const chunk = (i > 0 ? ' ' : '') + chunks[i];
      mockEmit('stream-chunk', { messageId: aiMessageId, chunk });
      i++;
    } else {
      clearInterval(interval);
      const finalMessage = {
        id: aiMessageId,
        conversationId: message.conversationId,
        sources: message.knowledgeBaseSelection !== 'none' ? [
            { id: 'mock-src-1', file_path: '/mock/path/doc.md', source_name: 'doc.md', content_snippet: 'This is a mock snippet from the knowledge base.', score: 0.95 }
        ] : [],
        suggestions: ['Mock Suggestion 1', 'Mock Suggestion 2'],
      };
      mockEmit('stream-end', { messageId: aiMessageId, finalMessage });
    }
  }, 50);
}

const mockConfiguredTools: ConfiguredTool[] = [
    {
        id: 'configured-tool-1',
        name: 'Translate to French',
        description: 'A configured tool to translate text specifically to French.',
        scriptPath: '/mock/scripts/translate.py',
        runtime: 'python',
        parameters: [
            { name: 'target_language', label: 'Target Language', paramType: 'text', defaultValue: 'fr', required: true },
            { name: 'source_text', label: 'Source Text', paramType: 'textarea', defaultValue: '', required: true },
        ],
        showInCopilot: true,
        isFavorite: false,
        inputSource: 'user_input',
        requiresAiPreProcessing: false,
        preProcessingPrompt: '',
        outputHandling: 'raw_text',
        requiresAiPostProcessing: false,
        postProcessingPrompt: '',
    }
];

const mockDatabase: Record<string, any> = {
  'get_user_settings': (): Settings => ({
    apiConfig: {
      providers: [
        { id: 'provider-1', name: 'Mock OpenAI', baseUrl: 'https://api.openai.com/v1', apiKey: 'sk-mock', models: [{name: 'gpt-4-turbo', capabilities: ['chat', 'vision'], maxTokens: 4096}, {name: 'gpt-3.5-turbo', capabilities: ['chat'], maxTokens: 4096}, {name: 'text-embedding-3-small', capabilities: ['embedding'], maxTokens: null}] },
        { id: 'provider-2', name: 'Mock Anthropic', baseUrl: 'https://api.anthropic.com/v1', apiKey: 'sk-mock-claude', models: [{name: 'claude-3-opus', capabilities: ['chat', 'vision'], maxTokens: 4096}] }
      ],
      assignments: {
        chat: { providerId: 'provider-1', modelName: 'gpt-4-turbo' },
        suggestion: { providerId: 'provider-1', modelName: 'gpt-3.5-turbo' },
        vision: { providerId: 'provider-1', modelName: 'gpt-4-turbo' },
        imageGen: null,
        embedding: { providerId: 'provider-1', modelName: 'text-embedding-3-small' },
        tts: null,
        videoGen: null,
      },
      keys: { tavily: '' },
    },
    knowledgeBase: {
        indexedDirectories: ['/mock/path/one', '/mock/path/two'],
        scriptsDirectories: ['/mock/scripts'],
        defaultSaveDirectory: '/mock/path/one',
        topK: 5,
        scoreThreshold: 0.6,
        defaultInternetSearchEngine: 'tavily',
    },
    appearance: { theme: 'system', language: 'system', copilotAutoHideDelay: 10, editorFontSize: 3 },
    execution: { pythonPath: '', nodePath: '', workingDirectory: '', autoStartBackend: false, backendUrl: 'http://127.0.0.1:8008' },
    shortcuts: { toggleCopilot: 'CmdOrCtrl+Shift+C', showMainWindow: '' },
  }),
  'list_conversations': (): Conversation[] => [
    { id: 'conv-1', title: 'Mock Conversation 1', created_at: Date.now() - 10000, sessionType: 'chat' },
    { id: 'conv-2', title: 'A much longer mock conversation title', created_at: Date.now(), sessionType: 'chat' },
  ],
  'get_conversation_history': () => [],
  'create_conversation': (args: { sessionType: 'chat' | 'creation' }): Conversation => ({
    id: uuidv4(),
    title: args.sessionType === 'creation' ? 'New Creation' : 'New Chat',
    created_at: Date.now(),
    sessionType: args.sessionType,
  }),
  'process_chat_message': (args: { userMessage: ChatMessage, aiMessageId: string }) => {
    simulateStreamingResponse(args.userMessage);
  },
  'update_message_content': () => { console.log('[Mock] Message content updated.'); },
  'get_all_notes': () => [],
  'get_knowledge_graph_data': () => ({ nodes: [], links: [] }),
  'rebuild_knowledge_graph': () => { console.log('[Mock] Knowledge graph rebuild requested.'); },
  'get_intent_suggestions': () => [],
  'search_local_kb': (args: { query: string }): KnowledgeSource[] => {
    if (!args.query) return [];
    return [
        { id: 'mock-1', file_path: '/mock/file1.md', source_name: 'file1.md', content_snippet: `Content related to ${args.query}`, score: 0.91 },
        { id: 'mock-2', file_path: '/mock/file2.txt', source_name: 'file2.txt', content_snippet: `More content about ${args.query}`, score: 0.88 },
    ];
  },
  'list_files_in_directory': (args: { path: string }): FileNode[] => [
    { key: `${args.path}/note1.md`, title: 'note1.md', isLeaf: true, children: undefined },
    { key: `${args.path}/note2.md`, title: 'note2.md', isLeaf: true, children: undefined },
  ],
  'read_file_content': (args: { path: string }): string => `# Mock Content for ${args.path}\n\nThis is some markdown content.`,
  'save_file_content': () => { console.log('[Mock] File saved'); },
  'create_unique_file': (args: { dirPath: string }): string => `${args.dirPath}/Untitled.md`,
  'list_tools': (): DynamicTool[] => [
    { id: 'dynamic::/mock/scripts/translate.py', name: 'Translate Text', description: 'Translates text to a specified language.', script_path: '/mock/scripts/translate.py', runtime: 'python' },
    { id: 'dynamic::/mock/scripts/summarize.py', name: 'Summarize Text', description: 'Generates a concise summary of the input text.', script_path: '/mock/scripts/summarize.py', runtime: 'python' },
  ],
  'list_configured_tools': (): ConfiguredTool[] => mockConfiguredTools,
  'save_configured_tool': (args: { tool: ConfiguredTool }) => {
      const index = mockConfiguredTools.findIndex(t => t.id === args.tool.id);
      if (index > -1) {
          mockConfiguredTools[index] = args.tool;
      } else {
          mockConfiguredTools.push(args.tool);
      }
  },
  'delete_configured_tool': (args: { id: string }) => {
      const index = mockConfiguredTools.findIndex(t => t.id === args.id);
      if (index > -1) {
          mockConfiguredTools.splice(index, 1);
      }
  },
  'execute_tool': (args: any): string => {
    const toolName = args.payload.toolName;
    if (toolName === 'built_in::save_to_kb') {
        return `Note saved to /mock/path/one/mock_note.md`;
    }
    return `Mock result for ${toolName} with input: ${JSON.stringify(args.payload.params)}`;
  },
  'open_directory_picker': (): DirectoryPickerResponse => ({ success: true, path: '/mock/selected/path', error: null }),
  'get_active_window_info': () => ({ appName: 'Browser', windowTitle: 'Nexus Dev' }),
  'get_clipboard_content': () => ({ contentType: 'text', content: 'This is mock clipboard content fetched on demand.' }),
  'get_integration_templates': (): IntegrationTemplate[] => [
    { id: 'zapier_inbound_note', name: 'Zapier: Create Note', description: 'Create a new note from a Zapier webhook.', serviceType: 'inbound_webhook' },
    { id: 'custom_inbound_note', name: 'Custom Webhook: Create Note', description: 'Create a new note from any service that can send a POST request.', serviceType: 'inbound_webhook' }
  ],
};

export async function mockInvoke<T>(command: string, args?: any): Promise<T> {
  const { error } = useToasts();
  console.log(`[Mock Invoke] Called: ${command}`, args);
  await new Promise(resolve => setTimeout(resolve, 200)); // Simulate network delay
  try {
    if (command in mockDatabase) {
      const result = mockDatabase[command](args);
      return Promise.resolve(result);
    }
    console.warn(`[Mock Invoke] Unhandled command: ${command}`);
    return Promise.resolve(null as any);
  } catch (err) {
    const errorMessage = typeof err === 'string' ? err : `An unknown error occurred for mocked command: ${command}.`;
    error(errorMessage);
    console.error(`Mock Command '${command}' failed:`, err);
    return Promise.reject(err);
  }
}