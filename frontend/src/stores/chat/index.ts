// frontend/src/stores/chat/index.ts
import { defineStore } from 'pinia';
import { toRefs } from 'vue';
import { useChatState, COPILOT_CONVERSATION_ID } from './state';
import { useChatActions } from './actions';
import { setupChatListeners } from './listeners';
import { useAgentStore } from '../agent';

// Re-export constants for external use
export { COPILOT_CONVERSATION_ID };

// This helps TypeScript understand the return type of useChatState
export type ReturnType_useChatState = ReturnType<typeof useChatState>;

export const useChatStore = defineStore('chat', () => {
  const state = useChatState();
  const actions = useChatActions(toRefs(state));
  
  // Pass both state and actions to listeners
  setupChatListeners(toRefs(state), actions);

  // Initial data loading and setup
  const agentStore = useAgentStore();
  actions.loadConversationList().then(() => {
    const lastId = localStorage.getItem('lastConversationId');
    if (lastId && state.conversationList.value.some(c => c.id === lastId)) {
      actions.selectConversation(lastId);
    } else if (state.conversationList.value.length > 0) {
      actions.selectConversation(state.conversationList.value[0].id);
    }
    agentStore.resumePollingForActiveTasks();
  });

  return {
    ...toRefs(state),
    ...actions,
  };
});