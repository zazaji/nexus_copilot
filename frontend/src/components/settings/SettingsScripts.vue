<!-- frontend/src/components/settings/SettingsScripts.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">Actions & Integrations</h2>
    <p class="text-sm text-gray-500 mt-1">Configure custom actions, scripts, and integrations with external services.</p>

    <!-- Configured Actions -->
    <div class="pt-6 mt-6 border-t border-gray-200 dark:border-gray-700">
      <div class="flex justify-between items-center">
        <div>
          <h3 class="font-medium mb-2">Configured Actions</h3>
          <p class="text-xs text-gray-500 mb-3">These actions can be triggered by the AI or manually.</p>
        </div>
        <button @click="openToolForm(null)" class="flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm">
          <Plus class="w-4 h-4 mr-2" />
          Add Action
        </button>
      </div>
      <div class="mt-4">
        <div v-if="toolsStore.isLoading" class="text-center text-gray-500">{{ $t('tools.loading') }}</div>
        <div v-else-if="configuredTools.length === 0" class="p-4 border border-dashed dark:border-gray-600 rounded-lg text-center text-gray-500">
          No actions configured yet.
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div v-for="tool in configuredTools" :key="tool.id" class="p-4 border dark:border-gray-700 rounded-lg flex justify-between items-center">
            <div class="min-w-0">
              <div class="flex items-center space-x-2">
                  <span class="text-xs font-mono px-2 py-1 rounded-full capitalize" :class="runtimeClass(tool.runtime)">
                      {{ tool.runtime }}
                  </span>
                  <p class="font-semibold truncate">{{ tool.name }}</p>
              </div>
              <p class="text-sm text-gray-500 truncate mt-1">{{ tool.description }}</p>
            </div>
            <div class="flex items-center space-x-2 flex-shrink-0">
              <button @click="openToolForm(tool)" class="p-2 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-md"><Pencil class="w-4 h-4" /></button>
              <button @click="toolsStore.deleteTool(tool.id)" class="p-2 hover:bg-red-100 dark:hover:bg-red-900/50 text-red-500 rounded-md"><Trash2 class="w-4 h-4" /></button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Script Directories (Kept for script discovery) -->
    <div class="pt-6 mt-6 border-t border-gray-200 dark:border-gray-700">
      <h3 class="font-medium mb-2">{{ $t('settings.scripts.scriptDirs') }}</h3>
      <p class="text-xs text-gray-500 mb-3">{{ $t('settings.scripts.scriptDirsDesc') }}</p>
      <button @click="addDirectory" class="flex items-center px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition-colors text-sm">
        <FolderPlus class="w-4 h-4 mr-2" />
        {{ $t('settings.scripts.addFolder') }}
      </button>
      <ul class="mt-4 space-y-2 max-w-2xl">
        <li v-for="(dir, index) in editableSettings.knowledgeBase.scriptsDirectories" :key="dir" class="flex items-center justify-between p-2 bg-gray-100 dark:bg-gray-700 rounded-md text-sm">
          <span class="font-mono truncate">{{ dir }}</span>
          <button @click="removeDirectory(index)" aria-label="Remove directory" class="text-gray-400 hover:text-red-500">
            <Trash2 class="w-4 h-4" />
          </button>
        </li>
        <li v-if="!editableSettings.knowledgeBase.scriptsDirectories || editableSettings.knowledgeBase.scriptsDirectories.length === 0" class="text-center text-sm text-gray-500 py-4">
          {{ $t('settings.scripts.noFolders') }}
        </li>
      </ul>
       <div class="pt-6 mt-2">
        <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">{{ $t('settings.scripts.save') }}</button>
      </div>
    </div>

    <ToolConfigForm
      v-model="isToolFormVisible"
      :tool="selectedTool"
      @save="handleSaveTool"
    />
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, onMounted, ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '../../stores/settings';
import { useToolsStore } from '../../stores/tools';
import { useToasts } from '../../composables/useToasts';
import { openDirectoryPicker } from '../../lib/api';
import type { Settings, ConfiguredTool, ToolRuntime } from '../../types';
import { FolderPlus, Trash2, Plus, Pencil } from 'lucide-vue-next';
import ToolConfigForm from './ToolConfigForm.vue';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const toolsStore = useToolsStore();
const { success, error } = useToasts();

const editableSettings = reactive<Partial<Settings>>({});
const isToolFormVisible = ref(false);
const selectedTool = ref<ConfiguredTool | null>(null);

onMounted(() => {
  toolsStore.fetchAllTools();
});

const configuredTools = computed(() => {
    return toolsStore.allTools.filter(tool => tool.type === 'configured') as ConfiguredTool[];
});

const runtimeClass = (runtime: ToolRuntime) => {
    switch(runtime) {
        case 'python': return 'bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300';
        case 'node': return 'bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300';
        case 'shell': return 'bg-gray-200 text-gray-800 dark:bg-gray-600 dark:text-gray-200';
        case 'webhook': return 'bg-orange-100 text-orange-800 dark:bg-orange-900/50 dark:text-orange-300';
        default: return 'bg-gray-100 text-gray-700';
    }
};

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    Object.assign(editableSettings, JSON.parse(JSON.stringify(newSettings)));
    if (!editableSettings.knowledgeBase?.scriptsDirectories) {
      editableSettings.knowledgeBase = { ...editableSettings.knowledgeBase, indexedDirectories: editableSettings.knowledgeBase?.indexedDirectories || [], scriptsDirectories: [] };
    }
  }
}, { immediate: true, deep: true });

const addDirectory = async () => {
  const response = await openDirectoryPicker();
  if (response?.success && response.path) {
    if (editableSettings.knowledgeBase?.scriptsDirectories) {
      if (!editableSettings.knowledgeBase.scriptsDirectories.includes(response.path)) {
        editableSettings.knowledgeBase.scriptsDirectories.push(response.path);
      } else {
        error('This directory is already added.');
      }
    }
  }
};

const removeDirectory = (index: number) => {
  editableSettings.knowledgeBase?.scriptsDirectories?.splice(index, 1);
};

const openToolForm = (tool: ConfiguredTool | null) => {
  selectedTool.value = tool;
  isToolFormVisible.value = true;
};

const handleSaveTool = (tool: ConfiguredTool) => {
  toolsStore.saveTool(tool);
};

const save = () => {
  if (settingsStore.settings && editableSettings.knowledgeBase) {
    const newSettings = { ...settingsStore.settings, knowledgeBase: editableSettings.knowledgeBase } as Settings;
    settingsStore.saveSettings(newSettings);
    success(t('settings.scripts.saveSuccess'));
  }
};
</script>