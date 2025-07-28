// frontend/src/stores/intent.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { IntentSuggestion, OverlayContext } from '../types';
import { getIntentSuggestions } from '../lib/api';

export const useIntentStore = defineStore('intent', () => {
  const suggestions = ref<IntentSuggestion[]>([]);
  const isLoading = ref(false);

  async function fetchSuggestions(context: OverlayContext) {
    if (!context) {
      suggestions.value = [];
      return;
    }
    
    isLoading.value = true;
    // Correctly pass the entire context, including contentType
    const result = await getIntentSuggestions({
      contentType: context.contentType,
      content: context.content,
      sourceApp: context.sourceApp,
    });
    
    if (result) {
      suggestions.value = result;
    } else {
      suggestions.value = [];
    }
    isLoading.value = false;
  }

  return {
    suggestions,
    isLoading,
    fetchSuggestions,
  };
});