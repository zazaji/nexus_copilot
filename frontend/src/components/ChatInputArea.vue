<!-- frontend/src/components/ChatInputArea.vue -->
<template>
  <div>
    <ChatInput 
      v-model="model"
      :is-loading="isLoading || chatStore.isAgentRunning"
      :mode="mode"
      @send="handleSend"
      @stop="handleStop"
    />
    <ChatConfigBar 
      v-model="selectedModel"
      v-model:knowledge-selection="knowledgeSelection"
      v-model:creation-type-value="activeCreationType"
      v-model:image-params-value="imageParams"
      v-model:audio-params-value="audioParams"
      :is-in-copilot="isInCopilot"
      :mode="mode"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useChatStore } from '../stores/chat';
import { useSettingsStore } from '../stores/settings';
import ChatInput from './ChatInput.vue';
import ChatConfigBar from './ChatConfigBar.vue';
import type { CreationType, ModelAssignments } from '../types';

const props = defineProps({
  isLoading: {
    type: Boolean,
    default: false,
  },
  conversationId: {
    type: String,
    required: true,
  },
  isInCopilot: {
    type: Boolean,
    default: false,
  },
  mode: {
    type: String as () => 'chat' | 'creation',
    default: 'chat',
  },
});

const model = defineModel<string>();
const emit = defineEmits(['send']);

const chatStore = useChatStore();
const settingsStore = useSettingsStore();

const selectedModel = ref<string | null>(null);
const knowledgeSelection = ref('none');

// State for creation mode
const activeCreationType = ref<CreationType>('image');
const imageParams = ref({ size: '1024x1024' });
const audioParams = ref({ voice: 'alloy' });

const syncStateFromStore = (convoId: string) => {
    if (props.mode === 'chat') {
      const config = chatStore.getConversationConfig(convoId);
      selectedModel.value = config.model;
      knowledgeSelection.value = config.knowledge;
    } else if (props.mode === 'creation') {
      // Set initial model based on default creation type
      updateModelForCreationType(activeCreationType.value);
    }
};

const updateModelForCreationType = (type: CreationType) => {
  if (!settingsStore.settings) return;
  const assignments = settingsStore.settings.apiConfig.assignments;
  
  const typeToAssignmentMap: Record<CreationType, keyof ModelAssignments | null> = {
    image: 'imageGen',
    audio: 'tts',
    video: 'videoGen',
    text: null,
  };

  const assignmentKey = typeToAssignmentMap[type];
  if (!assignmentKey) {
    selectedModel.value = null;
    return;
  }

  const assignment = assignments[assignmentKey];
  if (assignment) {
    selectedModel.value = `${assignment.providerId}::${assignment.modelName}`;
  } else {
    // Fallback: select the first available model for this capability
    const available = chatStore.getModelsByCapability(assignmentKey);
    selectedModel.value = available.length > 0 ? available[0].value : null;
  }
};

watch(() => props.conversationId, (newId) => {
    if (newId) {
        syncStateFromStore(newId);
    }
}, { immediate: true });

watch(selectedModel, (newModel) => {
    if (props.conversationId && props.mode === 'chat') {
        chatStore.setConversationConfig(props.conversationId, { model: newModel });
    }
});

watch(knowledgeSelection, (newSelection) => {
    if (props.conversationId && props.mode === 'chat') {
        chatStore.setConversationConfig(props.conversationId, { knowledge: newSelection });
    }
});

watch(activeCreationType, (newType) => {
  if (props.mode === 'creation') {
    updateModelForCreationType(newType);
  }
});

const handleSend = (payload: { text: string; attachments: any[] }) => {
  if (props.mode === 'creation') {
    emit('send', {
      ...payload,
      model: selectedModel.value,
      type: activeCreationType.value,
      params: activeCreationType.value === 'image' ? imageParams.value : audioParams.value,
    });
  } else {
    emit('send', {
      ...payload,
      model: selectedModel.value,
      knowledgeBaseSelection: knowledgeSelection.value,
    });
  }
};

const handleStop = () => {
  chatStore.stopGeneration();
};
</script>