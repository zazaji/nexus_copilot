<!-- frontend/src/App.vue -->
<template>
  <n-config-provider :theme="theme">
    <n-message-provider>
      <n-dialog-provider>
        <div class="h-screen w-screen overflow-hidden font-sans">
          <ToastProvider />
          
          <!-- Dynamically render content based on the window type -->
          <template v-if="isCopilot">
            <CopilotView />
            <FilePreviewModal />
          </template>
          <template v-else>
            <router-view />
          </template>

        </div>
      </n-dialog-provider>
    </n-message-provider>
    <n-global-style />
  </n-config-provider>
</template>

<script setup lang="ts">
import { onMounted, computed, watchEffect } from 'vue';
import { NConfigProvider, NGlobalStyle, NMessageProvider, NDialogProvider, darkTheme } from 'naive-ui';
import ToastProvider from './components/ToastProvider.vue';
import CopilotView from './views/CopilotView.vue';
import FilePreviewModal from './components/FilePreviewModal.vue';
import { useSettingsStore } from './stores/settings';

const settingsStore = useSettingsStore();

// Determine if this is the Copilot window or the main application window
const isCopilot = computed(() => window.location.pathname === '/copilot');

const theme = computed(() => {
  const currentTheme = settingsStore.settings?.appearance.theme;
  if (currentTheme === 'dark') {
    return darkTheme;
  }
  if (currentTheme === 'system') {
    // This check needs to be reactive to system changes
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? darkTheme : null;
  }
  return null;
});

// Watch for changes in font size setting and apply it to the root element
watchEffect(() => {
  const fontSizeLevel = settingsStore.settings?.appearance.editorFontSize ?? 3;
  const baseSize = 100; // Base font size percentage
  let percentage;

  switch (fontSizeLevel) {
    case 1: percentage = 80; break;  // XS
    case 2: percentage = 90; break;  // S
    case 3: percentage = 100; break; // M
    case 4: percentage = 110; break; // L
    case 5: percentage = 120; break; // XL
    default: percentage = 100;
  }
  
  document.documentElement.style.fontSize = `${percentage}%`;
});

onMounted(async () => {
  // Fetch settings if they are not already populated
  if (!settingsStore.settings) {
    await settingsStore.fetchSettings();
  }
});
</script>