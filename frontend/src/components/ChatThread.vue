<!-- frontend/src/components/ChatThread.vue -->
<template>
  <div class="space-y-8">
    <div v-for="(message, index) in messages" :key="message.id">
      <AgentExecutionView v-if="message.model === 'agent' && message.agentTask" :task="message.agentTask" />
      <div v-else-if="message.model === 'agent-starting'" class="flex items-center space-x-3 text-gray-500 dark:text-gray-400">
        <Loader2 class="w-5 h-5 animate-spin" />
        <p class="text-sm italic">{{ message.content[0].text }}</p>
      </div>
      <Message 
        v-else
        :message="message" 
        :is-streaming="message.id === streamingMessageId"
        :is-in-copilot="isInCopilot"
        @save-edit="emit('save-edit', $event)"
        @refresh="emit('refresh', $event)"
        @delete="emit('delete', $event)"
        @parsed-content-updated="parsedContentCache[message.id] = $event"
      />
      <ChatSuggestions
        v-if="index === messages.length - 1 && message.role === 'ai' && !isLoading && message.model !== 'agent' && message.model !== 'agent-starting'"
        :suggestions="message.suggestions"
        :parsed-content="parsedContentCache[message.id]"
        @send="emit('send-suggestion', $event)"
        @execute-code="emit('execute-code', { messageId: message.id, code: $event })"
        @preview-files="emit('preview-files', { messageId: message.id, codeBlocks: $event })"
      />
    </div>
    <MessageSkeleton v-if="isLoading && !streamingMessageId" />
  </div>
</template>

<script setup lang="ts">
import { PropType, reactive } from 'vue';
import type { ChatMessage } from '../types';
import Message from './Message.vue';
import ChatSuggestions from './ChatSuggestions.vue';
import MessageSkeleton from './MessageSkeleton.vue';
import AgentExecutionView from './AgentExecutionView.vue';
import { Loader2 } from 'lucide-vue-next';

defineProps({
  messages: {
    type: Array as PropType<ChatMessage[]>,
    required: true,
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
  streamingMessageId: {
    type: String as PropType<string | null>,
    default: null,
  },
  isInCopilot: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['save-edit', 'refresh', 'delete', 'send-suggestion', 'execute-code', 'preview-files']);

const parsedContentCache = reactive<Record<string, any[]>>({});
</script>