<!-- frontend/src/components/settings/SettingsShortcuts.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">Global Shortcuts</h2>
    <p class="text-sm text-gray-500 mt-1">Configure global keyboard shortcuts to quickly access Nexus features.</p>
    
    <div v-if="editableSettings.shortcuts" class="mt-6 space-y-6">
      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">Core Actions</h3>
        <div class="mt-4 space-y-4">
          <div>
            <label for="toggle-copilot" class="block text-sm font-medium">Toggle Copilot Window</label>
            <input 
              type="text" 
              id="toggle-copilot" 
              :value="editableSettings.shortcuts.toggleCopilot"
              @keydown.prevent="recordShortcut('toggleCopilot', $event)"
              class="mt-1 block w-full max-w-md input-style font-mono"
              placeholder="Click and press a key combination"
            />
          </div>
          <div>
            <label for="show-main" class="block text-sm font-medium">Show Main Window</label>
            <input 
              type="text" 
              id="show-main" 
              :value="editableSettings.shortcuts.showMainWindow"
              @keydown.prevent="recordShortcut('showMainWindow', $event)"
              class="mt-1 block w-full max-w-md input-style font-mono"
              placeholder="Click and press a key combination"
            />
          </div>
        </div>
      </div>

      <div class="pt-6">
        <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">Save Shortcuts</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { useSettingsStore } from '../../stores/settings';
import { useToasts } from '../../composables/useToasts';
import type { Settings, ShortcutsSettings } from '../../types';

const settingsStore = useSettingsStore();
const { success } = useToasts();

const editableSettings = reactive<{ shortcuts?: ShortcutsSettings }>({});

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.shortcuts = JSON.parse(JSON.stringify(newSettings.shortcuts || {}));
  }
}, { immediate: true, deep: true });

const recordShortcut = (key: keyof ShortcutsSettings, event: KeyboardEvent) => {
  if (!editableSettings.shortcuts) return;

  const parts = [];
  if (event.metaKey) parts.push('Cmd');
  if (event.ctrlKey) parts.push('Ctrl');
  if (event.altKey) parts.push('Alt');
  if (event.shiftKey) parts.push('Shift');

  const keyName = event.key.toUpperCase();
  if (!['META', 'CONTROL', 'ALT', 'SHIFT'].includes(keyName)) {
    parts.push(keyName);
  }
  
  if (parts.length > 1) {
    editableSettings.shortcuts[key] = parts.join('+');
  } else {
    // Do not record single modifier keys
    editableSettings.shortcuts[key] = '';
  }
};

const save = () => {
  if (settingsStore.settings && editableSettings.shortcuts) {
    const newSettingsToSave = JSON.parse(JSON.stringify(settingsStore.settings));
    newSettingsToSave.shortcuts = editableSettings.shortcuts;
    settingsStore.saveSettings(newSettingsToSave);
    success('Shortcuts saved successfully! They will be active immediately.');
  }
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
</style>