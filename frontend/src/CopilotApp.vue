<!-- frontend/src/CopilotApp.vue -->
<template>
  <n-config-provider :theme="theme">
    <n-message-provider>
      <n-dialog-provider>
        <div class="h-screen w-screen overflow-hidden font-sans">
          <ToastProvider />
          <CopilotView />
          <FilePreviewModal />
        </div>
      </n-dialog-provider>
    </n-message-provider>
    <n-global-style />
  </n-config-provider>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { NConfigProvider, NGlobalStyle, NMessageProvider, NDialogProvider, darkTheme } from 'naive-ui';
import ToastProvider from './components/ToastProvider.vue';
import CopilotView from './views/CopilotView.vue';
import FilePreviewModal from './components/FilePreviewModal.vue';
import { useSettingsStore } from './stores/settings';

const settingsStore = useSettingsStore();

const theme = computed(() => {
  const currentTheme = settingsStore.settings?.appearance.theme;
  if (currentTheme === 'dark') {
    return darkTheme;
  }
  if (currentTheme === 'system') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? darkTheme : null;
  }
  return null;
});

onMounted(async () => {
  if (!settingsStore.settings) {
    await settingsStore.fetchSettings();
  }
});
</script>