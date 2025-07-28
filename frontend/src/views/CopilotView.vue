<!-- frontend/src/views/CopilotView.vue -->
<template>
  <div 
    ref="copilotContainer"
    class="flex-1 flex flex-col h-full bg-gray-900/70 dark:bg-black/50 backdrop-blur-2xl rounded-lg border border-white/10 relative overflow-hidden"
    @mousemove="handleMouseMove"
    @mouseenter="clearHideTimer"
    @mouseleave="resetHideTimer"
  >
    <div 
      class="absolute top-0 left-0 w-full h-full pointer-events-none"
      :style="{
        background: `radial-gradient(600px at ${mouseX}px ${mouseY}px, rgba(29, 78, 216, 0.15), transparent 80%)`
      }"
    ></div>

    <div data-tauri-drag-region class="h-10 flex-shrink-0 flex items-center justify-between px-2 border-b border-white/10 z-10">
      <div class="flex items-center">
        <button v-if="viewState !== 'suggestions'" @click="goBackToSuggestions" class="p-2 rounded-md text-gray-400 hover:bg-white/10 hover:text-white" title="Back">
          <ArrowLeft class="w-4 h-4" />
        </button>
        <div data-tauri-drag-region class="flex items-center space-x-2 ml-1">
          <Sparkles class="w-5 h-5 text-blue-400" />
          <h2 data-tauri-drag-region class="font-bold text-gray-200">Copilot</h2>
        </div>
      </div>
      <div class="flex items-center">
        <button @click="toggleAutoHide" class="p-2 rounded-md text-gray-400 hover:bg-white/10 hover:text-white" :title="isAutoHidePaused ? 'Resume auto-hide' : 'Pause auto-hide'">
          <PinOff v-if="isAutoHidePaused" class="w-4 h-4 text-blue-400" />
          <Pin v-else class="w-4 h-4" />
        </button>
        <button @click="openMain" class="p-2 rounded-md text-gray-400 hover:bg-white/10 hover:text-white" title="Open Main Window">
          <ExternalLink class="w-4 h-4" />
        </button>
        <button @click="hide" class="p-2 rounded-md text-gray-400 hover:bg-white/10 hover:text-white" title="Hide Copilot">
          <ChevronDown class="w-4 h-4" />
        </button>
      </div>
    </div>

    <div class="flex-1 p-3 flex flex-col space-y-2 overflow-hidden z-10">
      <!-- Default Suggestion View -->
      <div v-if="viewState === 'suggestions'" class="flex-1 flex flex-col space-y-4 overflow-hidden">
        <ClipboardPreview :context="overlayStore.context" />
        <div v-if="toolResult" class="p-3 bg-black/20 rounded-lg max-h-40 overflow-y-auto border border-white/10 flex-shrink-0">
            <p class="text-xs font-semibold text-gray-300 mb-1">{{ $t('copilot.result') }}</p>
            <pre class="text-sm whitespace-pre-wrap font-mono text-gray-200">{{ toolResult }}</pre>
        </div>
        <h3 class="text-sm font-semibold text-gray-400 px-1 pt-2 flex-shrink-0">{{ $t('copilot.suggestions') }}</h3>
        <div class="flex-1 overflow-y-auto">
          <div class="grid grid-cols-2 gap-2">
            <SuggestionButton 
              v-for="suggestion in intentStore.suggestions" 
              :key="suggestion.action" 
              :suggestion="suggestion"
              @click="handleSuggestionClick(suggestion)"
            />
          </div>
        </div>
      </div>

      <!-- Clipboard History View -->
      <div v-else-if="viewState === 'clipboard_history'" class="flex-1 flex flex-col space-y-2 overflow-hidden">
        <div class="flex justify-between items-center flex-shrink-0">
          <h3 class="text-sm font-semibold text-gray-400 px-1">Clipboard History</h3>
          <div v-if="totalPages > 1" class="flex items-center space-x-1 text-gray-400">
            <button @click="goToFirstPage" :disabled="currentPage === 1" class="p-1 rounded-md hover:bg-white/10 disabled:opacity-50"><ChevronsLeft class="w-4 h-4" /></button>
            <button @click="prevPage" :disabled="currentPage === 1" class="p-1 rounded-md hover:bg-white/10 disabled:opacity-50"><ChevronLeft class="w-4 h-4" /></button>
            <span class="text-xs font-mono">{{ currentPage }} / {{ totalPages }}</span>
            <button @click="nextPage" :disabled="currentPage === totalPages" class="p-1 rounded-md hover:bg-white/10 disabled:opacity-50"><ChevronRight class="w-4 h-4" /></button>
            <button @click="goToEndPage" :disabled="currentPage === totalPages" class="p-1 rounded-md hover:bg-white/10 disabled:opacity-50"><ChevronsRight class="w-4 h-4" /></button>
          </div>
        </div>
        <div class="flex-1 overflow-y-auto bg-black/10 rounded-lg p-2 space-y-1">
          <div v-if="paginatedHistory.length === 0" class="text-center text-sm text-gray-400 pt-4">History is empty.</div>
          <div v-for="item in paginatedHistory" :key="item.id" 
            class="p-2 rounded-md hover:bg-white/10 text-sm cursor-pointer group"
            :class="{'bg-blue-900/30': item.isPinned}"
            @click="copyItem(item)"
            @dblclick="pasteItem(item)">
            <div class="flex justify-between items-start">
              <div class="flex-1 min-w-0">
                <p class="truncate font-mono text-gray-200">{{ getDisplayContent(item) }}</p>
                <p class="text-xs text-gray-500">{{ new Date(item.timestamp).toLocaleTimeString() }}</p>
              </div>
              <div class="flex items-center flex-shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
                <button @click.stop="clipboardStore.togglePin(item.id)" class="p-1 rounded-md hover:bg-white/20" :title="item.isPinned ? 'Unpin' : 'Pin'">
                  <Pin class="w-3 h-3" :class="item.isPinned ? 'text-blue-400 fill-current' : 'text-gray-300'" />
                </button>
                <button @click.stop="copyItem(item)" class="p-1 rounded-md hover:bg-white/20" title="Copy">
                  <Copy class="w-3 h-3 text-gray-300" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Execute View -->
      <div v-else-if="viewState === 'execute'" class="flex-1 flex flex-col space-y-3 overflow-hidden">
        <p class="text-sm font-semibold text-gray-400 px-1 flex-shrink-0">{{ $t('copilot.executeWith') }}</p>
        <div class="bg-black/10 rounded-lg p-3 space-y-3 flex-shrink-0">
            <button @click="handleExecute('python')" class="w-full p-3 bg-white/5 hover:bg-white/10 rounded-lg text-left flex items-center space-x-3">
                <Code class="w-5 h-5 text-blue-400" />
                <span class="font-medium text-gray-200">Python</span>
            </button>
            <button @click="handleExecute('node')" class="w-full p-3 bg-white/5 hover:bg-white/10 rounded-lg text-left flex items-center space-x-3">
                <Atom class="w-5 h-5 text-green-400" />
                <span class="font-medium text-gray-200">Node.js</span>
            </button>
            <button @click="handleExecute('shell')" class="w-full p-3 bg-white/5 hover:bg-white/10 rounded-lg text-left flex items-center space-x-3">
                <Terminal class="w-5 h-5 text-gray-400" />
                <span class="font-medium text-gray-200">Shell</span>
            </button>
        </div>
        <div class="flex-1 bg-black/20 rounded-lg p-3 overflow-y-auto border border-white/10">
            <pre class="text-sm whitespace-pre-wrap font-mono text-gray-200">{{ toolResult || 'Execution output will appear here...' }}</pre>
        </div>
      </div>

      <!-- Find File View -->
      <div v-else-if="viewState === 'find_file'" class="flex-1 flex flex-col space-y-3 overflow-hidden">
        <input 
          v-model="fileSearchQuery"
          @keydown.enter="handleFileSearch"
          :placeholder="$t('knowledge.searchPlaceholder')"
          class="w-full p-3 bg-black/20 rounded-lg border border-white/10 focus:outline-none focus:ring-2 focus:ring-blue-500 text-white flex-shrink-0"
          ref="fileSearchInputRef"
        />
        <div class="flex-1 overflow-y-auto bg-black/10 rounded-lg p-2 space-y-1">
          <div v-if="isSearchingFiles" class="flex justify-center items-center h-full">
            <Loader2 class="w-6 h-6 animate-spin text-gray-400" />
          </div>
          <div v-else-if="fileSearchResults.length > 0" v-for="file in fileSearchResults" :key="file" class="p-2 rounded-md hover:bg-white/10 text-sm font-mono cursor-pointer" @click="openFileInExplorer(file)">
            {{ file.split(/[/\\]/).pop() }}
            <p class="text-xs text-gray-400 truncate">{{ file }}</p>
          </div>
          <p v-else class="text-center text-sm text-gray-400 pt-4">{{ $t('knowledge.noResults') }}</p>
        </div>
      </div>

      <!-- Chat View -->
      <div v-else-if="viewState === 'chat'" class="flex-1 flex flex-col space-y-2 overflow-hidden">
        <div class="flex-1 overflow-y-auto space-y-4 pr-2">
          <ChatThread
            :messages="chatStore.copilotMessages"
            :is-loading="chatStore.isLoading"
            :streaming-message-id="chatStore.streamingMessageId"
            :is-in-copilot="true"
            @save-edit="handleSaveEdit"
            @refresh="handleRefresh"
            @delete="handleDelete"
            @send-suggestion="handleCopilotSend({ text: $event, attachments: [] })"
            @execute-code="handleExecuteCode"
          />
        </div>
        <div class="mt-auto pt-2 flex-shrink-0">
          <ChatInputArea
            v-model="copilotChatText"
            :is-loading="chatStore.isLoading"
            :conversation-id="COPILOT_CONVERSATION_ID"
            :is-in-copilot="true"
            mode="chat"
            @send="handleCopilotSend"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useOverlayStore } from '../stores/overlay';
import { useChatStore, COPILOT_CONVERSATION_ID } from '../stores/chat';
import { useIntentStore } from '../stores/intent';
import { useKnowledgeExplorerStore } from '../stores/knowledgeExplorer';
import { useSettingsStore } from '../stores/settings';
import { useClipboardStore } from '../stores/clipboard';
import { useToasts } from '../composables/useToasts';
import { show_window, executeTool, findFileInKb, onCopilotContextUpdate, onToolOutput, onToolComplete, onToolError, executeGenericCode, onCopilotShown, pasteText } from '../lib/api';
import { appWindow } from '@tauri-apps/api/window';
import { ExternalLink, Sparkles, ArrowLeft, Loader2, Code, Atom, Terminal, ChevronDown, Pin, PinOff, Copy, ChevronsLeft, ChevronLeft, ChevronRight, ChevronsRight } from 'lucide-vue-next';
import ChatThread from '../components/ChatThread.vue';
import ChatInputArea from '../components/ChatInputArea.vue';
import ClipboardPreview from '../components/copilot/ClipboardPreview.vue';
import SuggestionButton from '../components/copilot/SuggestionButton.vue';
import type { IntentSuggestion, OverlayContext, ToolRuntime, ChatMessageContentPart, ClipboardItem } from '../types';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';

const { t } = useI18n();
const overlayStore = useOverlayStore();
const chatStore = useChatStore();
const intentStore = useIntentStore();
const explorerStore = useKnowledgeExplorerStore();
const settingsStore = useSettingsStore();
const clipboardStore = useClipboardStore();
const { success } = useToasts();
const router = useRouter();

type ViewState = 'suggestions' | 'chat' | 'find_file' | 'execute' | 'clipboard_history';
const viewState = ref<ViewState>('suggestions');
const toolResult = ref<string | null>(null);
const runningTaskId = ref<string | null>(null);
const hideTimer = ref<number | null>(null);
const copilotChatText = ref('');
const isActivated = ref(false);
const isAutoHidePaused = ref(false);

const fileSearchQuery = ref('');
const fileSearchResults = ref<string[]>([]);
const isSearchingFiles = ref(false);
const fileSearchInputRef = ref<HTMLInputElement | null>(null);

const copilotContainer = ref<HTMLElement | null>(null);
const mouseX = ref(0);
const mouseY = ref(0);

// --- Clipboard History Pagination ---
const currentPage = ref(1);
const itemsPerPage = 10;
const totalPages = computed(() => Math.ceil(clipboardStore.history.length / itemsPerPage));
const paginatedHistory = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return clipboardStore.history.slice(start, end);
});

watch(totalPages, (newTotal) => {
  if (currentPage.value > newTotal) {
    currentPage.value = newTotal || 1;
  }
});

const nextPage = () => { if (currentPage.value < totalPages.value) currentPage.value++; };
const prevPage = () => { if (currentPage.value > 1) currentPage.value--; };
const goToFirstPage = () => { currentPage.value = 1; };
const goToEndPage = () => { currentPage.value = totalPages.value; };
// ---

const autoHideDelay = computed(() => (settingsStore.settings?.appearance.copilotAutoHideDelay ?? 10) * 1000);

const handleMouseMove = (event: MouseEvent) => {
  if (copilotContainer.value) {
    const rect = copilotContainer.value.getBoundingClientRect();
    mouseX.value = event.clientX - rect.left;
    mouseY.value = event.clientY - rect.top;
  }
};

const hide = () => {
  isActivated.value = false;
  appWindow.hide();
};

const clearHideTimer = () => {
    if (hideTimer.value) {
        clearTimeout(hideTimer.value);
        hideTimer.value = null;
    }
};

const resetHideTimer = () => {
    clearHideTimer();
    if (isActivated.value && !isAutoHidePaused.value) {
        hideTimer.value = window.setTimeout(hide, autoHideDelay.value);
    }
};

const toggleAutoHide = () => {
  isAutoHidePaused.value = !isAutoHidePaused.value;
  if (isAutoHidePaused.value) {
    clearHideTimer();
  } else {
    resetHideTimer();
  }
};

const updateContext = (context: OverlayContext) => {
  overlayStore.show(context);
  viewState.value = 'suggestions';
  toolResult.value = null;
  intentStore.fetchSuggestions(overlayStore.context!);
  isActivated.value = true;
  resetHideTimer();
};

const goBackToSuggestions = () => {
  viewState.value = 'suggestions';
  toolResult.value = null;
  fileSearchResults.value = [];
  fileSearchQuery.value = '';
};

const handleSuggestionClick = async (suggestion: IntentSuggestion) => {
  isAutoHidePaused.value = true;
  clearHideTimer();
  
  const content = overlayStore.context?.content || '';
  toolResult.value = null;

  if (suggestion.action === 'ask_ai') {
    viewState.value = 'chat';
    if (overlayStore.context?.contentType === 'text') {
      copilotChatText.value = overlayStore.context.content;
    }
    return;
  }
  
  if (suggestion.action === 'built_in::find_file') {
    viewState.value = 'find_file';
    fileSearchQuery.value = typeof content === 'string' ? content : '';
    fileSearchResults.value = [];
    nextTick(() => fileSearchInputRef.value?.focus());
    if (fileSearchQuery.value) {
      handleFileSearch();
    }
    return;
  }

  if (suggestion.action === 'built_in::execute') {
    viewState.value = 'execute';
    return;
  }

  if (suggestion.action === 'built_in::view_clipboard_history') {
    viewState.value = 'clipboard_history';
    return;
  }

  toolResult.value = `Executing ${suggestion.label}...`;
  const taskId = await executeTool(suggestion.action, { stdin: content });
  if (taskId) {
    runningTaskId.value = taskId;
  } else {
    toolResult.value = 'Failed to start tool.';
  }
};

const handleExecute = async (runtime: ToolRuntime) => {
    const code = overlayStore.context?.content;
    if (typeof code !== 'string' || !code.trim()) {
        toolResult.value = "Nothing to execute.";
        return;
    }
    toolResult.value = `Executing with ${runtime}...`;
    const taskId = await executeGenericCode(runtime, code);
    if (taskId) {
        runningTaskId.value = taskId;
    } else {
        toolResult.value = `Failed to start ${runtime} execution.`;
    }
};

const handleFileSearch = async () => {
  isSearchingFiles.value = true;
  const results = await findFileInKb(fileSearchQuery.value);
  fileSearchResults.value = results || [];
  isSearchingFiles.value = false;
};

const openFileInExplorer = (filePath: string) => {
  router.push({ name: 'KnowledgeBase' });
  show_window('main', true);
  nextTick(() => {
    explorerStore.selectFile(filePath);
  });
  hide();
};

const handleCopilotSend = (payload: { text: string; attachments: any[], model?: string | null, knowledgeBaseSelection?: string }) => {
  const content: ChatMessageContentPart[] = [];
  if (payload.text) {
    content.push({ type: 'text', text: payload.text });
  }
  payload.attachments.forEach(attachment => {
    if (attachment.type.startsWith('image/') && attachment.localPath) {
      content.push({
        type: 'image_url',
        image_url: { url: attachment.localPath }
      });
    }
  });

  if (content.length === 0) return;

  chatStore.sendMessage({ 
    content: content,
    conversationIdOverride: COPILOT_CONVERSATION_ID,
    modelOverride: payload.model,
    kbSelectionOverride: payload.knowledgeBaseSelection,
  });
  copilotChatText.value = '';
};

const handleSaveEdit = (payload: { messageId: string, newContent: string }) => {
  chatStore.saveEditedMessage(payload);
};

const handleRefresh = (messageId: string) => {
  chatStore.refreshMessage(messageId);
};

const handleDelete = (messageId: string) => {
  chatStore.deleteMessage(messageId);
};

const handleExecuteCode = (payload: { messageId: string, code: string }) => {
  chatStore.executeCodeInMessage(payload.messageId, payload.code);
};

const openMain = () => {
  show_window('main', true);
};

const getDisplayContent = (item: ClipboardItem) => {
  if (item.type === 'text') return JSON.parse(item.content);
  if (item.type === 'image') return '[Image]';
  if (item.type === 'files') {
    try {
      const files = JSON.parse(item.content);
      if (Array.isArray(files) && files.length > 0) {
        return `[Files] ${files[0].split(/[/\\]/).pop()}${files.length > 1 ? ` + ${files.length - 1} more` : ''}`;
      }
    } catch (e) { /* ignore */ }
    return '[Files]';
  }
  return '';
};

const copyItem = async (item: ClipboardItem) => {
  if (item.type === 'text') {
    await navigator.clipboard.writeText(JSON.parse(item.content));
    success('Text copied!');
  }
};

const pasteItem = async (item: ClipboardItem) => {
  if (item.type === 'text') {
    const textToPaste = JSON.parse(item.content);
    await pasteText(textToPaste);
    success('Pasted!');
    hide();
  }
};

let unlistenContextUpdate: UnlistenFn;
let unlistenToolOut: UnlistenFn;
let unlistenToolDone: UnlistenFn;
let unlistenToolErr: UnlistenFn;
let unlistenFocusChange: UnlistenFn;
let unlistenVisibleChange: UnlistenFn;
let unlistenCopilotShown: UnlistenFn;

onMounted(async () => {
  unlistenContextUpdate = await onCopilotContextUpdate(updateContext);
  unlistenCopilotShown = await onCopilotShown(() => {
    isActivated.value = true;
    resetHideTimer();
  });
  
  unlistenFocusChange = await appWindow.onFocusChanged(({ payload: focused }) => {
    if (focused) {
      isActivated.value = true;
      clearHideTimer();
    } else {
      resetHideTimer();
    }
  });

  unlistenVisibleChange = await appWindow.onVisibleChanged(({ payload: visible }) => {
    if (visible) {
      isActivated.value = true;
      resetHideTimer();
    } else {
      isActivated.value = false;
      clearHideTimer();
    }
  });

  unlistenToolOut = await onToolOutput(({ taskId, chunk }) => {
    if (taskId === runningTaskId.value) {
      if (toolResult.value?.startsWith('Executing')) toolResult.value = '';
      toolResult.value += chunk;
    }
  });
  unlistenToolDone = await onToolComplete(({ taskId }) => {
    if (taskId === runningTaskId.value) {
        runningTaskId.value = null;
    }
  });
  unlistenToolErr = await onToolError(({ taskId, payload }) => {
    if (taskId === runningTaskId.value) {
      toolResult.value += `\nERROR: ${payload}`;
      runningTaskId.value = null;
    }
  });
});

onUnmounted(() => {
  unlistenContextUpdate?.();
  unlistenCopilotShown?.();
  unlistenFocusChange?.();
  unlistenVisibleChange?.();
  unlistenToolOut?.();
  unlistenToolDone?.();
  unlistenToolErr?.();
  clearHideTimer();
});
</script>