<!-- frontend/src/views/CreationView.vue -->
<template>
  <div class="flex-1 flex flex-col h-full bg-gray-50 dark:bg-gray-900">
    <!-- Main Content -->
    <div class="flex-1 overflow-y-auto p-4 md:p-6 relative" ref="chatContainer">
      <div v-if="creationStore.isLoading" class="flex items-center justify-center h-full">
        <Loader2 class="w-8 h-8 animate-spin text-blue-500" />
      </div>
      <WelcomeGuide 
        v-else-if="messages.length === 0 && !creationStore.isGenerating"
        mode="creation"
        @select-example="handleSelectExample"
      />
      <ChatThread
        v-else
        :messages="messages"
        :is-loading="false"
      />
      <!-- Generation Progress Overlay -->
      <div v-if="creationStore.isGenerating" class="absolute inset-0 bg-gray-50/80 dark:bg-gray-900/80 backdrop-blur-sm flex flex-col items-center justify-center space-y-4 p-8 text-center">
        <Loader2 class="w-12 h-12 animate-spin text-blue-500" />
        <h3 class="text-lg font-semibold">Generating your creation...</h3>
        <div class="w-full max-w-md">
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" :style="{ width: `${creationStore.generationProgress}%` }"></div>
          </div>
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">{{ creationStore.generationMessage }}</p>
        </div>
      </div>
    </div>

    <!-- Unified Input Area -->
    <div class="p-4 md:p-6 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
      <ChatInputArea
        v-if="creationStore.currentSession"
        v-model="creationInputText"
        :conversation-id="creationStore.currentSession.id"
        :is-loading="creationStore.isGenerating"
        mode="creation"
        @send="handleGenerate"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { watch, onMounted, ref, computed, nextTick } from 'vue';
import { useRoute } from 'vue-router';
import { useCreationStore } from '../stores/creation';
import { Loader2 } from 'lucide-vue-next';
import ChatInputArea from '../components/ChatInputArea.vue';
import ChatThread from '../components/ChatThread.vue';
import WelcomeGuide from '../components/WelcomeGuide.vue';
import type { CreationType, ChatMessage } from '../types';

const route = useRoute();
const creationStore = useCreationStore();
const chatContainer = ref<HTMLElement | null>(null);
const creationInputText = ref('');

const scrollToBottom = () => {
  nextTick(() => {
    if (chatContainer.value) {
      chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
    }
  });
};

const messages = computed<ChatMessage[]>(() => {
  if (!creationStore.currentSession) return [];
  const result: ChatMessage[] = [];
  // Artifacts are newest first from the store, so we reverse to show oldest first in the chat.
  const sortedArtifacts = [...creationStore.artifacts].reverse();
  
  for (const artifact of sortedArtifacts) {
    // User Prompt
    result.push({
      id: `${artifact.id}-user`,
      conversationId: creationStore.currentSession.id,
      role: 'user',
      content: [{ type: 'text', text: artifact.prompt }],
      timestamp: artifact.createdAt - 1, // Ensure user message appears before AI response
    });
    // AI Artifact
    result.push({
      id: artifact.id,
      conversationId: creationStore.currentSession.id,
      role: 'ai',
      // We use 'image_url' content type which MediaBlock can handle for images, audio, and video
      content: [{ type: 'image_url', image_url: { url: artifact.fileUrl || artifact.filePath } }],
      timestamp: artifact.createdAt,
      model: artifact.modelUsed,
    });
  }
  return result;
});

const handleGenerate = async (payload: { text: string; attachments: any[]; model: string | null; params: Record<string, any>, type: CreationType }) => {
  if (!payload.model) {
    alert('Please select a model.');
    return;
  }

  const prompt = payload.text;
  
  await creationStore.generateArtifact({
    type: payload.type,
    prompt: prompt,
    params: {
      model: payload.model,
      ...payload.params
    }
  });
  creationInputText.value = '';
};

const handleSelectExample = (prompt: string) => {
  creationInputText.value = prompt;
};

watch(() => route.params.id, (newId) => {
  if (newId && typeof newId === 'string') {
    if (newId !== creationStore.currentSession?.id) {
      creationStore.selectSession(newId);
    }
  }
}, { immediate: true });

watch(messages, () => {
  scrollToBottom();
}, { deep: true });

onMounted(() => {
  scrollToBottom();
});
</script>