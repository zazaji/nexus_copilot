// frontend/src/stores/creation.ts
import { defineStore } from 'pinia';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { useSettingsStore } from './settings';
import { useToasts } from '../composables/useToasts';
import type { CreationArtifact, CreationType, ApiProvider, Conversation } from '../types';
import {
  createConversation,
  getConversationWithArtifacts,
  generateArtifact as apiGenerateArtifact,
} from '../lib/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { useChatStore } from './chat';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

export const useCreationStore = defineStore('creation', () => {
  const router = useRouter();
  const settingsStore = useSettingsStore();
  const chatStore = useChatStore();
  const { success, error, info } = useToasts();

  const currentSession = ref<Conversation | null>(null);
  const isLoading = ref(false);
  const isGenerating = ref(false);
  const generationProgress = ref(0);
  const generationMessage = ref('');
  let unlistenProgress: UnlistenFn | null = null;

  const artifacts = computed(() => currentSession.value?.artifacts || []);

  onMounted(async () => {
    unlistenProgress = await listen('artifact-progress', (event: any) => {
      const payload = event.payload;
      generationMessage.value = payload.message || `Processing...`;
      generationProgress.value = payload.progress || generationProgress.value;
      if (payload.status === 'FAILED') {
        error(payload.message || 'Artifact generation failed during processing.');
        isGenerating.value = false;
      }
    });
  });

  onUnmounted(() => {
    unlistenProgress?.();
  });

  function getFileUrl(artifact: CreationArtifact): string {
    if (artifact.filePath.startsWith('http')) {
      return artifact.filePath;
    }
    return convertFileSrc(artifact.filePath);
  }

  async function selectSession(id: string) {
    isLoading.value = true;
    const result = await getConversationWithArtifacts(id);
    if (result) {
      for (const artifact of result.artifacts || []) {
        artifact.fileUrl = getFileUrl(artifact);
      }
      currentSession.value = result;
    } else {
      error('Could not load creation session.');
      router.push({ name: 'Dashboard' });
    }
    isLoading.value = false;
  }

  async function startNewSession() {
    const result = await createConversation('creation', 'New Creation');
    if (result) {
      chatStore.conversationList.unshift(result);
      router.push({ name: 'Creation', params: { id: result.id } });
    } else {
      error('Failed to create a new session.');
    }
  }

  async function generateArtifact(payload: {
    type: CreationType;
    prompt: string;
    params: { model: string; [key: string]: any };
  }) {
    if (!currentSession.value) {
      error('No active session selected.');
      return;
    }
    if (!settingsStore.settings) {
      error('Settings not loaded.');
      return;
    }

    isGenerating.value = true;
    generationProgress.value = 0;
    generationMessage.value = 'Initializing...';

    try {
      const [providerId, modelName] = payload.params.model.split('::');
      const provider = settingsStore.settings.apiConfig.providers.find(p => p.id === providerId);

      if (!provider) {
        throw new Error('Could not find the selected API provider.');
      }

      const result = await apiGenerateArtifact(
        currentSession.value.id,
        payload.type,
        payload.prompt,
        modelName,
        payload.params,
        provider
      );

      if (result) {
        result.fileUrl = getFileUrl(result);
        if (!currentSession.value.artifacts) {
          currentSession.value.artifacts = [];
        }
        currentSession.value.artifacts.unshift(result);
        success('Artifact generated successfully!');
      }
    } catch (err: any) {
      const errorMessage = err.toString();
      if (errorMessage.includes('501') || errorMessage.toLowerCase().includes('not implemented')) {
        info('This generation feature is coming soon and is not yet available.');
      } else {
        error('Artifact generation failed.');
      }
    } finally {
      isGenerating.value = false;
      generationProgress.value = 0;
      generationMessage.value = '';
    }
  }

  return {
    currentSession,
    isLoading,
    isGenerating,
    generationProgress,
    generationMessage,
    artifacts,
    selectSession,
    startNewSession,
    generateArtifact,
  };
});