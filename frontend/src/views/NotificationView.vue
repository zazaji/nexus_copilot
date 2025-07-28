<!-- frontend/src/views/NotificationView.vue -->
<template>
  <div class="w-full h-full bg-white/80 dark:bg-gray-800/80 backdrop-blur-xl rounded-lg p-4 flex flex-col space-y-3" @mouseleave="hide" @blur="hide">
    <div class="flex-1">
      <p class="text-sm font-semibold">New Content Copied</p>
      <p class="text-xs text-gray-600 dark:text-gray-400 line-clamp-2">
        {{ overlayStore.context?.content || '...' }}
      </p>
    </div>
    <div class="flex items-center justify-end space-x-2">
      <button @click="openCopilot" class="px-3 py-1 text-xs bg-gray-200 dark:bg-gray-700 rounded-md hover:bg-gray-300">Open Copilot</button>
      <button @click="openMain" class="px-3 py-1 text-xs bg-blue-500 text-white rounded-md hover:bg-blue-600">Open Main</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useOverlayStore } from '../stores/overlay';
import { onClipboardUpdate, show_window, hide_window } from '../lib/api';
import type { UnlistenFn } from '@tauri-apps/api/event';

const overlayStore = useOverlayStore();
let unlisten: UnlistenFn | null = null;
let hideTimer: number | null = null;

const hide = () => {
  hide_window('notification');
};

const resetHideTimer = () => {
  if (hideTimer) clearTimeout(hideTimer);
  hideTimer = setTimeout(hide, 5000); // Hide after 5 seconds of inactivity
};

const openCopilot = () => {
  show_window('copilot');
  hide();
};

const openMain = () => {
  show_window('main');
  hide();
};

onMounted(async () => {
  unlisten = await onClipboardUpdate((payload) => {
    overlayStore.show(payload);
    resetHideTimer();
  });
  resetHideTimer();
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (hideTimer) clearTimeout(hideTimer);
});
</script>