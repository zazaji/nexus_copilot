// frontend/src/stores/chat/actions.ts
import { v4 as uuidv4 } from 'uuid';
import { useRouter } from 'vue-router';
import { useSettingsStore } from '../settings';
import { useAgentStore } from '../agent';
import { useToasts } from '../../composables/useToasts';
import type { ChatMessage, ChatMessageContentPart, Conversation, ConversationConfig, ModelEndpoint, ApiConfig, AgentTask, ModelInfo } from '../../types';
import {
    listConversations,
    getConversationHistory,
    deleteConversation as apiDeleteConversation,
    deleteMessage as apiDeleteMessage,
    clearAllConversations as apiClearAll,
    createConversation as apiCreateConversation,
    processChatMessage,
    stopChatGeneration,
    updateConversationTitle as apiUpdateConversationTitle,
    updateMessageContent,
    executePythonCode,
    stopAgentTask,
    linkAgentTaskToMessage,
    saveMessage,
} from '../../lib/api';
import type { ToRefs } from 'vue';
import type { ReturnType_useChatState } from './index';

interface SendMessagePayload {
  content: ChatMessageContentPart[];
  conversationIdOverride?: string;
  modelOverride?: string;
  kbSelectionOverride?: string;
}

export function useChatActions(state: ToRefs<ReturnType_useChatState>) {
  const {
    conversations,
    conversationList,
    currentConversationId,
    isLoading,
    streamingMessageId,
    conversationConfigs,
    isAgentRunning,
    getModelsByCapability,
  } = state;

  const settingsStore = useSettingsStore();
  const agentStore = useAgentStore();
  const router = useRouter();
  const { error, info } = useToasts();

  function findMessageById(messageId: string): ChatMessage | undefined {
    for (const convoId in conversations.value) {
        const message = conversations.value[convoId].find(m => m.id === messageId);
        if (message) return message;
    }
    return undefined;
  }

  function findMessageByAgentTaskId(agentTaskId: string): ChatMessage | undefined {
    for (const convoId in conversations.value) {
        const message = conversations.value[convoId].find(m => m.agentTaskId === agentTaskId);
        if (message) return message;
    }
    return undefined;
  }

  const getModelInfo = (modelIdentifier: string): ModelInfo | null => {
    if (!settingsStore.settings) return null;
    const [providerId, modelName] = modelIdentifier.split('::');
    const provider = settingsStore.settings.apiConfig.providers.find(p => p.id === providerId);
    if (!provider) return null;
    return provider.models.find(m => m.name === modelName) || null;
  };

  const getDefaultModel = (capability: 'chat' | 'vision' = 'chat') => {
    if (!settingsStore.settings) return null;
    const assignment = settingsStore.settings.apiConfig.assignments[capability];
    if (assignment) {
      return `${assignment.providerId}::${assignment.modelName}`;
    }
    // Fallback to the first available model with the required capability
    const capableModels = getModelsByCapability.value(capability);
    return capableModels.length > 0 ? capableModels[0].value : null;
  };

  function getConversationConfig(id: string): ConversationConfig {
    if (!conversationConfigs.value[id]) {
        conversationConfigs.value[id] = {
            model: getDefaultModel('chat'),
            knowledge: 'none',
        };
    }
    return conversationConfigs.value[id];
  }
  
  function setConversationConfig(id: string, config: Partial<ConversationConfig>) {
    if (!conversationConfigs.value[id]) {
        getConversationConfig(id);
    }
    Object.assign(conversationConfigs.value[id], config);
  }

  async function loadConversationList() {
    if (conversationList.value.length > 0) return;
    const convos = await listConversations();
    if (convos) {
      conversationList.value = convos;
    }
  }

  async function startNewConversation() {
    const newConvo = await apiCreateConversation('chat');
    if (newConvo) {
      conversationList.value.unshift(newConvo);
      conversations.value[newConvo.id] = [];
      selectConversation(newConvo.id);
      router.push({ name: 'Chat', params: { id: newConvo.id } });
    }
  }

  async function selectConversation(id: string | null) {
    if (!id) {
      currentConversationId.value = null;
      return;
    }
    
    if (conversationList.value.some(c => c.id === id)) {
        if (!conversations.value[id] || conversations.value[id].length === 0) {
            isLoading.value = true;
            const history = await getConversationHistory(id);
            
            if (history) {
                // Fetch full agent task states before setting the conversation
                const enrichedHistory = await agentStore.fetchInitialTaskStates(history);
                conversations.value[id] = enrichedHistory;
            } else {
                conversations.value[id] = [];
            }
            isLoading.value = false;
        }
    } else if (!conversations.value[id]) {
        conversations.value[id] = [];
    }

    currentConversationId.value = id;
    if (id !== 'copilot-chat') {
      localStorage.setItem('lastConversationId', id);
    }
  }

  async function deleteCurrentConversation() {
    const idToDelete = currentConversationId.value;
    if (!idToDelete) return;

    const nextConversation = conversationList.value.find(c => c.id !== idToDelete);
    await apiDeleteConversation(idToDelete);
    
    delete conversations.value[idToDelete];
    if (idToDelete in conversationConfigs.value) {
        delete conversationConfigs.value[idToDelete];
    }
    conversationList.value = conversationList.value.filter(c => c.id !== idToDelete);

    if (nextConversation) {
        router.push({ name: 'Chat', params: { id: nextConversation.id } });
    } else {
        router.push({ name: 'Dashboard' });
    }
  }

  async function deleteMessage(messageId: string) {
    const message = findMessageById(messageId);
    if (!message) return;

    const convoId = message.conversationId;
    const messageIndex = conversations.value[convoId]?.findIndex(m => m.id === messageId);
    
    if (messageIndex > -1) {
      conversations.value[convoId].splice(messageIndex, 1);
      await apiDeleteMessage(messageId);
    }
  }

  async function deleteSubsequentError(taskId: string) {
    if (!currentConversationId.value) return;
    const messages = conversations.value[currentConversationId.value] || [];
    const agentMessageIndex = messages.findIndex(m => m.agentTaskId === taskId && m.model === 'agent');
    
    if (agentMessageIndex > -1 && agentMessageIndex + 1 < messages.length) {
      const nextMessage = messages[agentMessageIndex + 1];
      if (nextMessage.model === 'agent-result' && nextMessage.agentTaskId === taskId && nextMessage.error) {
        await deleteMessage(nextMessage.id);
      }
    }
  }

  async function clearAllConversations() {
    await apiClearAll();
    conversations.value = {};
    conversationList.value = [];
    for (const key in conversationConfigs.value) {
        delete conversationConfigs.value[key];
    }
    currentConversationId.value = null;
    router.push({ name: 'Dashboard' });
  }

  async function updateConversationTitle(id: string, title: string) {
    const convo = conversationList.value.find(c => c.id === id);
    if (convo && convo.title !== title) {
      convo.title = title; // Optimistic UI update
      await apiUpdateConversationTitle(id, title);
    }
  }

  function addMessage(message: ChatMessage) {
    const convoId = message.conversationId;
    if (!conversations.value[convoId]) {
      conversations.value[convoId] = [];
    }
    conversations.value[convoId].push(message);
  }

  async function addAndSaveMessage(message: ChatMessage) {
    addMessage(message);
    await saveMessage(message);
  }

  async function updateAndSaveMessage(updatedMessage: ChatMessage) {
    const message = findMessageById(updatedMessage.id);
    if (message) {
        Object.assign(message, updatedMessage);
        await saveMessage(message);
    }
  }

  async function handleAgentCommand(conversationId: string, instruction: string, model: string | null, knowledgeSelection: string, mode: 'plan' | 'explore' | 'write' | 'research' | 'debate') {
    if (!settingsStore.settings) {
        error("Settings are not loaded. Cannot start agent.");
        return;
    }

    // Check if this is the first message in a new chat and update title
    const convo = conversationList.value.find(c => c.id === conversationId);
    if (convo && convo.title === 'New Chat...') {
        updateConversationTitle(conversationId, instruction);
    }

    const userMessage: ChatMessage = {
      id: uuidv4(),
      conversationId,
      role: 'user',
      content: [{ type: 'text', text: `/${mode} ${instruction}` }],
      timestamp: Date.now(),
    };
    await addAndSaveMessage(userMessage);

    const placeholderMessage: ChatMessage = {
      id: uuidv4(),
      conversationId,
      role: 'ai',
      content: [],
      timestamp: Date.now(),
      model: 'agent',
      agentTask: {
        id: 'temp-id',
        conversationId,
        userGoal: instruction,
        status: 'planning',
        mode: mode,
        createdAt: Date.now(),
        steps: [],
      }
    };
    await addAndSaveMessage(placeholderMessage);

    agentStore.startAgentTask(conversationId, instruction, placeholderMessage, model, knowledgeSelection, mode);
  }

  async function sendMessage(payload: SendMessagePayload) {
    const { content, conversationIdOverride, modelOverride, kbSelectionOverride } = payload;
    const conversationId = conversationIdOverride || currentConversationId.value;

    if (!conversationId) return;
    if (!settingsStore.settings) {
        error("Settings are not loaded. Cannot send message.");
        return;
    }

    const config = getConversationConfig(conversationId);
    const hasImage = content.some(p => p.type === 'image_url');
    
    let modelToSend = modelOverride || config.model;
    let finalContent = content;

    if (hasImage) {
        const visionModel = getDefaultModel('vision');
        if (!visionModel) {
            alert("No vision-capable model is assigned in Settings. Please configure a model with the 'Vision' capability.");
            return;
        }
        const selectedModelInfo = modelToSend ? getModelInfo(modelToSend) : null;
        if (!selectedModelInfo || !selectedModelInfo.capabilities.includes('vision')) {
            info(`Image detected. Switched to default vision model.`);
            modelToSend = visionModel;
        }
    } else if (!modelToSend) {
        modelToSend = getDefaultModel('chat');
    }

    if (!modelToSend) {
      alert("No chat model available. Please configure an API provider in Settings.");
      return;
    }
    
    const kbSelectionToSend = kbSelectionOverride || config.knowledge;
    
    const userMessage: ChatMessage = {
      id: uuidv4(),
      conversationId,
      role: 'user',
      content: finalContent,
      timestamp: Date.now(),
      model: modelToSend,
      knowledgeBaseSelection: kbSelectionToSend,
    };
    addMessage(userMessage);
    isLoading.value = true;

    const aiMessageId = uuidv4();
    const aiMessage: ChatMessage = {
      id: aiMessageId,
      conversationId,
      role: 'ai',
      content: [{ type: 'text', text: '' }],
      timestamp: Date.now(),
    };
    addMessage(aiMessage);
    streamingMessageId.value = aiMessageId;

    const apiConfigToSend: ApiConfig = JSON.parse(JSON.stringify(settingsStore.settings.apiConfig));
    apiConfigToSend.knowledgeBase = settingsStore.settings.knowledgeBase;
    apiConfigToSend.execution = settingsStore.settings.execution;

    processChatMessage(userMessage, aiMessageId, apiConfigToSend);
  }

  async function stopGeneration() {
    if (streamingMessageId.value) {
      await stopChatGeneration(streamingMessageId.value);
      isLoading.value = false;
      streamingMessageId.value = null;
    } else if (isAgentRunning.value && currentConversationId.value) {
      const currentConvoMessages = conversations.value[currentConversationId.value] || [];
      const agentMessage = [...currentConvoMessages].reverse().find(m => m.model === 'agent');
      if (agentMessage?.agentTaskId) {
        await stopAgentTask(agentMessage.agentTaskId);
      }
    }
  }

  async function executeCodeInMessage(messageId: string, code: string) {
    const message = findMessageById(messageId);
    if (!message) return;

    message.isExecuting = true;
    message.executionOutput = '';

    await executePythonCode(messageId, code);
  }

  function clearCopilotConversation() {
    conversations.value['copilot-chat'] = [];
  }

  async function saveEditedMessage(payload: { messageId: string, newContent: string }) {
    const message = findMessageById(payload.messageId);
    if (message) {
      message.content = [{ type: 'text', text: payload.newContent }];
      await updateMessageContent(payload.messageId, payload.newContent);
    }
  }

  async function refreshMessage(messageId: string) {
    const message = findMessageById(messageId);
    if (!message) return;

    const convoId = message.conversationId;
    const messages = conversations.value[convoId];
    const messageIndex = messages.findIndex(m => m.id === messageId);

    if (messageIndex > 0 && messages[messageIndex].role === 'ai') {
      const userMessageToResend = messages[messageIndex - 1];
      
      messages.splice(messageIndex, messages.length - messageIndex);

      await sendMessage({
        content: userMessageToResend.content,
        conversationIdOverride: convoId,
        modelOverride: userMessageToResend.model,
        kbSelectionOverride: userMessageToResend.knowledgeBaseSelection,
      });
    }
  }

  async function handleInteractionCompletion(message: ChatMessage) {
    const convo = conversationList.value.find(c => c.id === message.conversationId);
    if (convo && convo.title === 'New Chat...') {
        // This logic will now be handled by the backend via the `stream-end` event's final payload.
        // The frontend listener will then update the title.
    }
  }

  return {
    findMessageById,
    findMessageByAgentTaskId,
    getDefaultModel,
    getConversationConfig,
    setConversationConfig,
    loadConversationList,
    startNewConversation,
    selectConversation,
    deleteCurrentConversation,
    deleteMessage,
    deleteSubsequentError,
    clearAllConversations,
    updateConversationTitle,
    addMessage,
    addAndSaveMessage,
    updateAndSaveMessage,
    handleAgentCommand,
    sendMessage,
    stopGeneration,
    executeCodeInMessage,
    clearCopilotConversation,
    saveEditedMessage,
    refreshMessage,
    handleInteractionCompletion,
  };
}