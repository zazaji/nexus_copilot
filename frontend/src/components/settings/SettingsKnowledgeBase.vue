<!-- frontend/src/components/settings/SettingsKnowledgeBase.vue -->
<template>
  <div v-if="editableSettings.knowledgeBase">
    <h2 class="text-lg font-semibold">{{ $t('settings.knowledge.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.knowledge.description') }}</p>
    
    <!-- Search Configuration -->
    <div class="mt-6 p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">Search Configuration</h3>
        <p class="text-sm text-gray-500 mt-1 mb-4">These settings affect both local and online knowledge base searches.</p>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <label for="top-k" class="block text-sm font-medium">Top K</label>
                <n-input-number 
                    id="top-k" 
                    v-model:value="editableSettings.knowledgeBase.topK" 
                    class="mt-1"
                    :min="1" 
                    :max="50"
                />
                <p class="text-xs text-gray-500 mt-1">Number of results to retrieve (1-50).</p>
            </div>
            <div>
                <label for="score-threshold" class="block text-sm font-medium">Score Threshold</label>
                <n-select
                    id="score-threshold"
                    v-model:value="computedThreshold"
                    class="mt-1"
                    :options="thresholdOptions"
                />
                <p class="text-xs text-gray-500 mt-1">Minimum similarity score for results.</p>
            </div>
            <div class="md:col-span-2">
                <label for="search-engine" class="block text-sm font-medium">Default Internet Search Engine</label>
                <select
                    id="search-engine"
                    v-model="editableSettings.knowledgeBase.defaultInternetSearchEngine"
                    class="mt-1 block w-full input-style"
                >
                  <option value="tavily">Tavily</option>
                  <option value="bing">Bing</option>
                </select>
                <p class="text-xs text-gray-500 mt-1">This engine will be used for "Internet Search" in chat and by agents.</p>
            </div>
        </div>
    </div>

    <!-- Local Directories -->
    <div class="mt-6">
      <h3 class="font-semibold">Local Directories</h3>
      <div class="flex space-x-3 mt-2">
        <button @click="addDirectory" class="flex items-center px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors text-sm">
          <FolderPlus class="w-4 h-4 mr-2" />
          {{ $t('settings.knowledge.addFolder') }}
        </button>
        <button @click="clearKnowledgeBase" class="flex items-center px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors text-sm">
          <Trash2 class="w-4 h-4 mr-2" />
          {{ $t('settings.knowledge.clearAll') }}
        </button>
      </div>

      <div class="mt-4">
        <div v-if="editableSettings.knowledgeBase.indexedDirectories.length === 0" class="text-center text-sm text-gray-500 py-4 border border-dashed rounded-lg">
          {{ $t('settings.knowledge.noFolders') }}
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div v-for="dir in editableSettings.knowledgeBase.indexedDirectories" :key="dir" class="flex items-center justify-between p-3 bg-gray-100 dark:bg-gray-700 rounded-md text-sm group">
            <div class="flex items-center space-x-3 min-w-0">
              <input 
                type="radio" 
                name="default-save-dir" 
                :id="`radio-${dir}`"
                :value="dir"
                v-model="editableSettings.knowledgeBase.defaultSaveDirectory"
                class="form-radio h-4 w-4 text-blue-600 bg-gray-200 dark:bg-gray-800 border-gray-300 dark:border-gray-600 focus:ring-blue-500 flex-shrink-0"
              >
              <label :for="`radio-${dir}`" class="font-mono truncate cursor-pointer" :title="dir">{{ dir }}</label>
            </div>
            <div class="flex items-center space-x-2 opacity-50 group-hover:opacity-100 transition-opacity flex-shrink-0">
              <button @click="rebuildIndex(dir)" :aria-label="$t('settings.knowledge.rebuildIndex')" class="text-gray-400 hover:text-blue-500" :title="$t('settings.knowledge.rebuildIndex')">
                <RefreshCw class="w-4 h-4" />
              </button>
              <button @click="removeDirectory(dir)" :aria-label="$t('settings.knowledge.removeDirectory')" class="text-gray-400 hover:text-red-500" :title="$t('settings.knowledge.removeDirectory')">
                <X class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Online Knowledge Bases -->
    <div class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
        <h3 class="font-semibold">Online Knowledge Bases</h3>
        <div class="flex space-x-3 mt-2">
            <button @click="openOnlineKbModal(null)" class="flex items-center px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm">
                <Globe class="w-4 h-4 mr-2" />
                Add Online KB
            </button>
        </div>
        <div class="mt-4">
            <div v-if="kbStore.onlineKbs.length === 0" class="text-center text-sm text-gray-500 py-4 border border-dashed rounded-lg">
                No online knowledge bases configured.
            </div>
            <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div v-for="kb in kbStore.onlineKbs" :key="kb.id" class="flex items-center justify-between p-3 bg-gray-100 dark:bg-gray-700 rounded-md text-sm group">
                    <div class="flex items-center space-x-3 min-w-0">
                        <Globe class="w-4 h-4 text-blue-500 flex-shrink-0" />
                        <span class="font-semibold truncate" :title="kb.name">{{ kb.name }}</span>
                    </div>
                    <div class="flex items-center space-x-2 opacity-50 group-hover:opacity-100 transition-opacity flex-shrink-0">
                        <button @click="openOnlineKbModal(kb)" class="text-gray-400 hover:text-blue-500" title="Edit">
                            <Pencil class="w-4 h-4" />
                        </button>
                        <button @click="kbStore.deleteOnlineKb(kb.id)" class="text-gray-400 hover:text-red-500" title="Delete">
                            <X class="w-4 h-4" />
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <div class="pt-6 mt-4 border-t border-gray-200 dark:border-gray-700">
      <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">{{ $t('settings.knowledge.save') }}</button>
    </div>

    <OnlineKbConfig 
        v-model="isOnlineKbModalVisible"
        :kb="editingOnlineKb"
        @save="handleSaveOnlineKb"
    />
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, ref, onMounted, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '../../stores/settings';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { startIndexing, removeIndexedDirectory, clearKnowledgeBase as apiClearKB, rebuildIndex as apiRebuildIndex, openDirectoryPicker } from '../../lib/api';
import { FolderPlus, Trash2, RefreshCw, X, Globe, Pencil } from 'lucide-vue-next';
import { useToasts } from '../../composables/useToasts';
import type { Settings, OnlineKnowledgeBase } from '../../types';
import { NInputNumber, NSelect } from 'naive-ui';
import OnlineKbConfig from './OnlineKbConfig.vue';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const kbStore = useKnowledgeBaseStore();
const { success, error, info } = useToasts();

const editableSettings = reactive<Partial<Settings>>({});
const isOnlineKbModalVisible = ref(false);
const editingOnlineKb = ref<OnlineKnowledgeBase | null>(null);

const thresholdOptions = [0.3,0.4,0.5, 0.6, 0.7, 0.8, 0.9].map(v => ({ label: v.toFixed(1), value: v }));

const computedThreshold = computed({
  get() {
    const threshold = editableSettings.knowledgeBase?.scoreThreshold;
    if (typeof threshold === 'number') {
      return parseFloat(threshold.toFixed(1));
    }
    return 0.6;
  },
  set(newValue: number) {
    if (editableSettings.knowledgeBase) {
      editableSettings.knowledgeBase.scoreThreshold = newValue;
    }
  }
});

onMounted(() => {
    kbStore.fetchOnlineKbs();
});

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.knowledgeBase = JSON.parse(JSON.stringify(newSettings.knowledgeBase || { indexedDirectories: [], scriptsDirectories: [], defaultSaveDirectory: null, topK: 5, scoreThreshold: 0.6, defaultInternetSearchEngine: 'tavily' }));
  }
}, { immediate: true, deep: true });

const openOnlineKbModal = (kb: OnlineKnowledgeBase | null) => {
    editingOnlineKb.value = kb;
    isOnlineKbModalVisible.value = true;
};

const handleSaveOnlineKb = (kb: OnlineKnowledgeBase) => {
    if (editingOnlineKb.value) {
        kbStore.updateOnlineKb(kb);
    } else {
        kbStore.addOnlineKb(kb);
    }
};

const addDirectory = async () => {
  const response = await openDirectoryPicker();
  if (response?.success && response.path) {
    const selectedPath = response.path;
    if (editableSettings.knowledgeBase) {
      if (!editableSettings.knowledgeBase.indexedDirectories.includes(selectedPath)) {
        editableSettings.knowledgeBase.indexedDirectories.push(selectedPath);
        if (editableSettings.knowledgeBase.indexedDirectories.length === 1) {
          editableSettings.knowledgeBase.defaultSaveDirectory = selectedPath;
        }
        info(`Directory added. Click "Save" to apply changes and start indexing.`);
      } else {
        error('This directory is already added.');
      }
    }
  }
};

const removeDirectory = (path: string) => {
  if (editableSettings.knowledgeBase) {
    const index = editableSettings.knowledgeBase.indexedDirectories.indexOf(path);
    if (index > -1) {
      editableSettings.knowledgeBase.indexedDirectories.splice(index, 1);
      if (editableSettings.knowledgeBase.defaultSaveDirectory === path) {
        editableSettings.knowledgeBase.defaultSaveDirectory = editableSettings.knowledgeBase.indexedDirectories[0] || null;
      }
      info(`Directory marked for removal. Click "Save" to apply changes.`);
    }
  }
};

const save = async () => {
  if (!settingsStore.settings || !editableSettings.knowledgeBase) return;

  const oldDirs = new Set(settingsStore.settings.knowledgeBase.indexedDirectories);
  const newDirs = new Set(editableSettings.knowledgeBase.indexedDirectories);

  const dirsToAdd = [...newDirs].filter(dir => !oldDirs.has(dir));
  const dirsToRemove = [...oldDirs].filter(dir => !newDirs.has(dir));

  const newSettings = JSON.parse(JSON.stringify(settingsStore.settings));
  newSettings.knowledgeBase = editableSettings.knowledgeBase;
  await settingsStore.saveSettings(newSettings);
  success('Knowledge Base settings saved.');

  for (const dir of dirsToRemove) {
    await removeIndexedDirectory(dir);
    info(`Removed directory and its data from knowledge base: ${dir}`);
  }

  for (const dir of dirsToAdd) {
    await startIndexing(dir);
    info(`Indexing started for new directory: ${dir}`);
  }
};

const clearKnowledgeBase = async () => {
    await apiClearKB();
    success('Knowledge base cleared.');
};

const rebuildIndex = async (path: string) => {
    await apiRebuildIndex(path);
    success(`Rebuilding index for "${path}" started.`);
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
</style>