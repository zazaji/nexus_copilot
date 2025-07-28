// frontend/src/stores/chat/state.ts
import { ref, computed, reactive } from 'vue';
import { useSettingsStore } from '../settings';
import type { ChatMessage, Conversation, ConversationConfig, ModelCapability } from '../../types';

export const COPILOT_CONVERSATION_ID = 'copilot-chat';

export function useChatState() {
  const conversations = ref<Record<string, ChatMessage[]>>({});
  const conversationList = ref<Conversation[]>([]);
  const currentConversationId = ref<string | null>(null);
  const isLoading = ref(false);
  const streamingMessageId = ref<string | null>(null);
  const conversationConfigs = reactive<Record<string, ConversationConfig>>({});

  const settingsStore = useSettingsStore();

  const isAgentRunning = computed(() => {
    if (!currentConversationId.value) return false;
    const currentConvoMessages = conversations.value[currentConversationId.value] || [];
    const agentMessage = currentConvoMessages.find(m => m.model === 'agent' && m.agentTask);
    if (!agentMessage || !agentMessage.agentTask) return false;
    // A task is considered "running" only if it's actively planning or executing.
    return agentMessage.agentTask.status === 'running' || agentMessage.agentTask.status === 'planning';
  });

  const isTaskPaused = computed(() => {
    if (!currentConversationId.value) return false;
    const currentConvoMessages = conversations.value[currentConversationId.value] || [];
    const agentMessage = currentConvoMessages.find(m => m.model === 'agent' && m.agentTask);
    if (!agentMessage || !agentMessage.agentTask) return false;
    // A task is paused if it's awaiting user input.
    return agentMessage.agentTask.status === 'awaiting_user_input';
  });

  const getModelsByCapability = (capability: ModelCapability) => {
    const options: { value: string, label: string }[] = [];
    if (settingsStore.settings && settingsStore.settings.apiConfig) {
        settingsStore.settings.apiConfig.providers.forEach(p => {
          p.models.forEach(m => {
            if (m.capabilities.includes(capability)) {
              let label = `${p.name} - ${m.name}`;
              if (m.capabilities.includes('vision')) {
                label += ' 👁️'; // Add a visual indicator for vision models
              }
              options.push({
                value: `${p.id}::${m.name}`,
                label: label
              });
            }
          });
        });
    }
    return options;
  };

  const availableModels = computed(() => getModelsByCapability('chat'));

  const currentMessages = computed(() => {
    return currentConversationId.value ? conversations.value[currentConversationId.value] || [] : [];
  });

  const copilotMessages = computed(() => {
    return conversations.value[COPILOT_CONVERSATION_ID] || [];
  });

  return {
    conversations,
    conversationList,
    currentConversationId,
    isLoading,
    streamingMessageId,
    conversationConfigs,
    isAgentRunning,
    isTaskPaused,
    availableModels,
    currentMessages,
    copilotMessages,
    getModelsByCapability,
  };
}