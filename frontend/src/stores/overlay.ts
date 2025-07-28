// frontend/src/stores/overlay.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { OverlayContext } from '../types';

export const useOverlayStore = defineStore('overlay', () => {
  const context = ref<OverlayContext | null>(null);

  async function show(newContext: OverlayContext) {
    context.value = newContext;
  }

  function hide() {
    context.value = null;
  }

  function updateContextContent(newContent: string) {
    if (context.value && context.value.contentType === 'text') {
      context.value.content = newContent;
    }
  }

  return { 
    context, 
    show, 
    hide,
    updateContextContent,
  };
});