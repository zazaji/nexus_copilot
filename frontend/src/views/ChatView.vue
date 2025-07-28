<!-- frontend/src/views/ChatView.vue -->
<template>
  <div class="flex-1 flex overflow-hidden">
    <!-- Main Chat Area -->
    <div 
      class="flex flex-col overflow-hidden bg-gray-50 dark:bg-gray-900 relative transition-all duration-300 ease-in-out"
      :class="uiStore.isArtifactPanelVisible && taskForPreview ? 'w-1/2' : 'flex-1'"
    >
      <div class="flex-1 overflow-y-auto p-4 md:p-6" ref="chatContainer" @click="handleContainerClick">
        <div v-if="!chatStore.currentConversationId && !chatStore.isLoading" class="flex flex-col items-center justify-center h-full text-gray-500">
          <Bot class="w-24 h-24 mb-4 text-gray-400" />
          <h1 class="text-2xl font-semibold">Select a conversation</h1>
          <p class="mt-2">or start a new one to begin.</p>
        </div>
        <WelcomeGuide 
          v-else-if="chatStore.currentMessages.length === 0 && !chatStore.isLoading"
          mode="chat"
          @select-example="handleSelectExample"
        />
        <ChatThread
          v-else
          :messages="chatStore.currentMessages"
          :is-loading="chatStore.isLoading"
          :streaming-message-id="chatStore.streamingMessageId"
          @save-edit="handleSaveEdit"
          @refresh="handleRefresh"
          @delete="handleDelete"
          @send-suggestion="handleSendSuggestion"
          @execute-code="handleExecuteCode"
          @preview-files="handleWriteAndPreview"
        />
      </div>
      <div class="p-4 md:p-6 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
        <ChatInputArea
          v-if="chatStore.currentConversationId"
          v-model="chatInputText"
          :is-loading="chatStore.isLoading"
          :conversation-id="chatStore.currentConversationId"
          mode="chat"
          @send="handleSend"
        />
      </div>
    </div>

    <!-- Artifact Preview Panel -->
    <div v-if="uiStore.isArtifactPanelVisible && taskForPreview" class="w-1/2 flex-shrink-0 overflow-hidden transition-all duration-300 ease-in-out">
      <ArtifactPreviewPanel :task="taskForPreview" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useChatStore } from '../stores/chat';
import { useUiStore } from '../stores/ui';
import { useSettingsStore } from '../stores/settings';
import ChatInputArea from '../components/ChatInputArea.vue';
import ChatThread from '../components/ChatThread.vue';
import ArtifactPreviewPanel from '../components/ArtifactPreviewPanel.vue';
import WelcomeGuide from '../components/WelcomeGuide.vue';
import { Bot } from 'lucide-vue-next';
import { openExternalLink, setupTaskWorkspace, writeFileToTaskDir } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import type { ChatMessageContentPart, AgentTask } from '../types';

const route = useRoute();
const router = useRouter();
const chatStore = useChatStore();
const uiStore = useUiStore();
const settingsStore = useSettingsStore();
const { success, error } = useToasts();
const chatContainer = ref<HTMLElement | null>(null);
const chatInputText = ref('');

const taskForPreview = computed<AgentTask | null>(() => {
  if (!uiStore.previewedTaskId) return null;
  
  // Search through the entire message history for the task to preview.
  for (const message of chatStore.currentMessages) {
    if (message.agentTask && message.agentTask.id === uiStore.previewedTaskId) {
      return message.agentTask;
    }
  }
  return null;
});

const scrollToBottom = () => {
  nextTick(() => {
    if (chatContainer.value) {
      chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
    }
  });
};

const handleContainerClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  const link = target.closest('a');

  if (link && link.href && (link.href.startsWith('http://') || link.href.startsWith('https://'))) {
    event.preventDefault();
    openExternalLink(link.href);
  }
};

const handleSend = (payload: { text: string; attachments: any[], model: string | null, knowledgeBaseSelection: string }) => {
  const text = payload.text ? payload.text.trim() : '';
  
  if (text.startsWith('/plan ') && chatStore.currentConversationId) {
    const instruction = text.substring('/plan '.length);
    if (instruction) {
      chatStore.handleAgentCommand(chatStore.currentConversationId, instruction, payload.model, payload.knowledgeBaseSelection, 'plan');
    }
  } else if (text.startsWith('/explore ') && chatStore.currentConversationId) {
    const instruction = text.substring('/explore '.length);
    if (instruction) {
      chatStore.handleAgentCommand(chatStore.currentConversationId, instruction, payload.model, payload.knowledgeBaseSelection, 'explore');
    }
  } else if (text.startsWith('/write ') && chatStore.currentConversationId) {
    const instruction = text.substring('/write '.length);
    if (instruction) {
      chatStore.handleAgentCommand(chatStore.currentConversationId, instruction, payload.model, payload.knowledgeBaseSelection, 'write');
    }
  } else if (text.startsWith('/research ') && chatStore.currentConversationId) {
    const instruction = text.substring('/research '.length);
    if (instruction) {
      chatStore.handleAgentCommand(chatStore.currentConversationId, instruction, payload.model, payload.knowledgeBaseSelection, 'research');
    }
  } else if (text.startsWith('/debate ') && chatStore.currentConversationId) {
    const instruction = text.substring('/debate '.length);
    if (instruction) {
      chatStore.handleAgentCommand(chatStore.currentConversationId, instruction, payload.model, payload.knowledgeBaseSelection, 'debate');
    }
  } else {
    const content: ChatMessageContentPart[] = [];
    if (text) {
      content.push({ type: 'text', text });
    }
    payload.attachments.forEach(attachment => {
      if (attachment.type.startsWith('image/') && attachment.localPath) {
        content.push({
          type: 'image_url',
          image_url: { url: attachment.localPath }
        });
      }
    });

    chatStore.sendMessage({
      content: content,
      modelOverride: payload.model,
      kbSelectionOverride: payload.knowledgeBaseSelection,
    });
  }
  chatInputText.value = '';
};

const handleSelectExample = (prompt: string) => {
  chatInputText.value = prompt;
};

const handleSendSuggestion = (suggestion: string) => {
  chatStore.sendMessage({ content: [{ type: 'text', text: suggestion }] });
};

const handleExecuteCode = (payload: { messageId: string, code: string }) => {
  chatStore.executeCodeInMessage(payload.messageId, payload.code);
};

const handleWriteAndPreview = async (payload: { messageId: string, codeBlocks: any[] }) => {
    const taskId = payload.messageId;
    await setupTaskWorkspace(taskId);

    let firstHtmlFile: string | null = null;
    for (const block of payload.codeBlocks) {
        const firstLine = block.content.split('\n')[0];
        const match = firstLine.match(/^(?:#|<!--)\s*([a-zA-Z0-9_./-]+\.[a-zA-Z0-9]+)\s*(?:-->)?$/);
        const filename = match ? match[1] : null;

        if (filename) {
            await writeFileToTaskDir(filename, block.content);
            if (!firstHtmlFile && filename.endsWith('.html')) {
                firstHtmlFile = filename;
            }
        }
    }

    if (firstHtmlFile) {
        const backendUrl = settingsStore.settings?.execution.backendUrl;
        if (backendUrl) {
            const previewUrl = `${backendUrl}/tasks/${taskId}/${firstHtmlFile}`;
            openExternalLink(previewUrl);
            success('Files saved. Opening preview...');
        } else {
            error('Backend URL not configured. Cannot open preview.');
        }
    } else {
        error('No HTML file with a valid filename comment found to preview.');
    }
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

watch(() => chatStore.currentMessages, () => {
  scrollToBottom();
}, { deep: true });

watch(() => route.params.id, (newId) => {
  const newIdStr = newId as string | undefined;
  if (newIdStr && chatStore.currentConversationId !== newIdStr) {
    chatStore.selectConversation(newIdStr);
  }
}, { immediate: true });

watch(() => chatStore.currentConversationId, (newId, oldId) => {
    if (newId && newId !== oldId && newId !== route.params.id) {
        router.push({ name: 'Chat', params: { id: newId } });
    } else if (!newId && oldId) {
        router.push({ name: 'Dashboard' });
    }
});
</script>