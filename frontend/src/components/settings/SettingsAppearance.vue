<!-- frontend/src/components/settings/SettingsAppearance.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">Appearance</h2>
    <p class="text-sm text-gray-500 mt-1">Customize the look and feel of Nexus.</p>
    <div class="mt-6 space-y-4" v-if="editableSettings.appearance">
      <div>
        <label class="block text-sm font-medium">Theme</label>
        <div class="mt-2 flex space-x-4">
          <label v-for="themeOption in themes" :key="themeOption.value" class="flex items-center space-x-2 cursor-pointer">
            <input type="radio" name="theme" :value="themeOption.value" v-model="editableSettings.appearance.theme" class="form-radio h-4 w-4 text-blue-600">
            <span class="capitalize">{{ themeOption.label }}</span>
          </label>
        </div>
      </div>
      <div class="pt-6 border-t border-gray-200 dark:border-gray-700">
        <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">Save Appearance</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
import { useSettingsStore } from '../../stores/settings';
import { useToasts } from '../../composables/useToasts';
import type { AppearanceSettings } from '../../types';
import { applyTheme } from '../../utils/theme';

const settingsStore = useSettingsStore();
const { success } = useToasts();

const themes = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
];

const editableSettings = reactive<{ appearance?: AppearanceSettings }>({});

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.appearance = JSON.parse(JSON.stringify(newSettings.appearance));
  }
}, { immediate: true, deep: true });

watch(() => editableSettings.appearance?.theme, (newTheme) => {
  if (newTheme) {
    applyTheme(newTheme);
  }
});

const save = () => {
  if (settingsStore.settings && editableSettings.appearance) {
    const newSettingsToSave = JSON.parse(JSON.stringify(settingsStore.settings));
    newSettingsToSave.appearance = editableSettings.appearance;
    settingsStore.saveSettings(newSettingsToSave);
    success('Appearance settings saved!');
  }
};
</script>