<!-- frontend/src/components/settings/SettingsGeneral.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">{{ $t('settings.general.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.general.description') }}</p>
    
    <div class="mt-6 space-y-6" v-if="editableSettings.appearance && editableSettings.shortcuts">
      <!-- General Settings -->
      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <div class="space-y-4">
          <div>
            <label for="language-select" class="block text-sm font-medium">{{ $t('settings.general.language') }}</label>
            <select 
              id="language-select" 
              v-model="editableSettings.appearance.language"
              class="mt-1 block w-full max-w-md input-style"
            >
              <option value="system">System</option>
              <option value="en">English</option>
              <option value="zh">中文</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium">{{ $t('settings.general.theme') }}</label>
            <div class="mt-2 flex space-x-4">
              <label v-for="themeOption in themes" :key="themeOption.value" class="flex items-center space-x-2 cursor-pointer">
                <input type="radio" name="theme" :value="themeOption.value" v-model="editableSettings.appearance.theme" class="form-radio h-4 w-4 text-blue-600">
                <span class="capitalize">{{ $t(themeOption.label) }}</span>
              </label>
            </div>
          </div>
          <div>
            <label for="copilot-delay" class="block text-sm font-medium">{{ $t('settings.general.copilotDelay') }}</label>
            <input 
              type="number" 
              id="copilot-delay" 
              v-model.number="editableSettings.appearance.copilotAutoHideDelay"
              class="mt-1 block w-full max-w-md input-style"
              min="1"
            />
          </div>
          <div>
            <label for="font-size-slider" class="block text-sm font-medium">UI Font Size</label>
            <n-slider
              id="font-size-slider"
              v-model:value="editableSettings.appearance.editorFontSize"
              class="mt-2 max-w-md"
              :min="1"
              :max="5"
              :step="1"
              :marks="{ 1: 'XS', 2: 'S', 3: 'M', 4: 'L', 5: 'XL' }"
            />
          </div>
          <div class="flex items-center justify-between max-w-md">
            <label for="autostart-switch" class="block text-sm font-medium">{{ $t('settings.general.autostart') }}</label>
            <n-switch id="autostart-switch" v-model:value="isAutostartOn" />
          </div>
        </div>
      </div>

      <!-- Shortcuts Settings -->
      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">{{ $t('settings.general.shortcutsTitle') }}</h3>
        <p class="text-sm text-gray-500 mt-1">{{ $t('settings.general.shortcutsDesc') }}</p>
        <div class="mt-4 space-y-4">
          <div>
            <label for="toggle-copilot" class="block text-sm font-medium">{{ $t('settings.general.toggleCopilot') }}</label>
            <input 
              type="text" 
              id="toggle-copilot" 
              :value="editableSettings.shortcuts.toggleCopilot"
              @keydown.prevent="recordShortcut('toggleCopilot', $event)"
              class="mt-1 block w-full max-w-md input-style font-mono"
              :placeholder="$t('settings.general.shortcutPlaceholder')"
            />
          </div>
          <div>
            <label for="show-main" class="block text-sm font-medium">{{ $t('settings.general.showMain') }}</label>
            <input 
              type="text" 
              id="show-main" 
              :value="editableSettings.shortcuts.showMainWindow"
              @keydown.prevent="recordShortcut('showMainWindow', $event)"
              class="mt-1 block w-full max-w-md input-style font-mono"
              :placeholder="$t('settings.general.shortcutPlaceholder')"
            />
          </div>
        </div>
      </div>

      <div class="pt-6 border-t border-gray-200 dark:border-gray-700">
        <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">{{ $t('settings.general.save') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '../../stores/settings';
import { useToasts } from '../../composables/useToasts';
import type { Settings, AppearanceSettings, ShortcutsSettings } from '../../types';
import { applyTheme } from '../../utils/theme';
import { NSlider, NSwitch } from 'naive-ui';
import { enableAutostart, disableAutostart, isAutostartEnabled } from '../../lib/api';

const settingsStore = useSettingsStore();
const { success } = useToasts();
const { locale, t } = useI18n();

const themes = [
  { value: 'light', label: 'settings.general.themes.light' },
  { value: 'dark', label: 'settings.general.themes.dark' },
  { value: 'system', label: 'settings.general.themes.system' },
];

const editableSettings = reactive<{ appearance?: AppearanceSettings, shortcuts?: ShortcutsSettings }>({});
const isAutostartOn = ref(false);

onMounted(async () => {
  const enabled = await isAutostartEnabled();
  isAutostartOn.value = enabled ?? false;
});

watch(isAutostartOn, async (newValue) => {
  if (newValue) {
    await enableAutostart();
  } else {
    await disableAutostart();
  }
});

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.appearance = JSON.parse(JSON.stringify(newSettings.appearance || {}));
    editableSettings.shortcuts = JSON.parse(JSON.stringify(newSettings.shortcuts || {}));
  }
}, { immediate: true, deep: true });

watch(() => editableSettings.appearance?.theme, (newTheme) => {
  if (newTheme) {
    applyTheme(newTheme);
  }
});

watch(() => editableSettings.appearance?.language, (newLang) => {
  if (newLang && newLang !== 'system') {
    locale.value = newLang;
  } else {
    const browserLang = navigator.language.split('-')[0];
    locale.value = browserLang === 'zh' ? 'zh' : 'en';
  }
});

const recordShortcut = (key: keyof ShortcutsSettings, event: KeyboardEvent) => {
  if (!editableSettings.shortcuts) return;

  const parts = [];
  if (event.metaKey) parts.push('Cmd');
  if (event.ctrlKey) parts.push('Ctrl');
  if (event.altKey) parts.push('Alt');
  if (event.shiftKey) parts.push('Shift');

  const keyName = event.key.toUpperCase();
  if (!['META', 'CONTROL', 'ALT', 'SHIFT', 'CAPSLOCK', 'DEAD'].includes(keyName)) {
    parts.push(keyName);
  }
  
  if (parts.length > 1) {
    editableSettings.shortcuts[key] = parts.join('+');
  } else {
    editableSettings.shortcuts[key] = '';
  }
};

const save = () => {
  if (settingsStore.settings && editableSettings.appearance && editableSettings.shortcuts) {
    const newSettingsToSave = JSON.parse(JSON.stringify(settingsStore.settings)) as Settings;
    newSettingsToSave.appearance = editableSettings.appearance;
    newSettingsToSave.shortcuts = editableSettings.shortcuts;
    settingsStore.saveSettings(newSettingsToSave);
    success(t('settings.general.saveSuccess'));
  }
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
</style>