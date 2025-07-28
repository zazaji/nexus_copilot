<!-- frontend/src/components/knowledge/KnowledgeEditor.vue -->
<template>
  <div class="flex flex-col h-full">
    <div v-if="explorerStore.isLoadingFile" class="flex-1 flex items-center justify-center">
      <Loader2 class="w-8 h-8 animate-spin text-blue-500" />
    </div>
    <div v-else-if="!explorerStore.activeFile && !explorerStore.activeOnlineResult" class="flex-1 flex flex-col items-center justify-center text-gray-500">
      <FileSearch class="w-24 h-24 mb-4 text-gray-400" />
      <h2 class="text-xl font-semibold">{{ $t('knowledge.editor.emptyTitle') }}</h2>
      <p>{{ $t('knowledge.editor.emptyDesc') }}</p>
    </div>
    <div v-else class="flex-1 flex flex-col overflow-hidden">
      <header class="p-2 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
        <div class="flex items-center space-x-2 flex-1 min-w-0">
            <button 
                @click="$emit('toggle-explorer')" 
                class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700"
                :title="isExplorerVisible ? 'Hide Explorer' : 'Show Explorer'"
            >
                <PanelLeftClose v-if="isExplorerVisible" class="w-5 h-5 text-blue-500" />
                <PanelLeftOpen v-else class="w-5 h-5 text-blue-500" />
            </button>
            <div class="font-mono text-sm truncate">
                <template v-if="explorerStore.activeFile">
                    <input 
                        v-if="isEditingFilename"
                        ref="filenameInputRef"
                        v-model="editableFilename"
                        @blur="finishEditingFilename"
                        @keydown.enter="finishEditingFilename"
                        @keydown.esc="cancelEditingFilename"
                        class="bg-transparent border-b border-blue-500 focus:outline-none text-sm font-mono"
                    />
                    <span v-else @dblclick="startEditingFilename" :title="explorerStore.activeFile.key">
                        {{ explorerStore.activeFile.title }}
                    </span>
                </template>
                <template v-else-if="explorerStore.activeOnlineResult">
                    <span :title="explorerStore.activeOnlineResult.file_path">
                        {{ explorerStore.activeOnlineResult.source_name }}
                    </span>
                </template>
            </div>
        </div>
        <div v-if="explorerStore.activeFile" class="flex items-center space-x-2">
          <button @click="toggleMode" class="px-3 py-1 text-xs rounded-md bg-gray-200 dark:bg-gray-600">
            {{ explorerStore.editorMode === 'edit' ? $t('knowledge.editor.preview') : $t('knowledge.editor.edit') }}
          </button>
          <button 
            @click="handleSave" 
            :disabled="explorerStore.editorMode === 'preview'"
            class="px-4 py-1 text-xs rounded-md bg-blue-500 text-white hover:bg-blue-600 disabled:bg-gray-400"
          >
            {{ $t('knowledge.editor.save') }}
          </button>
        </div>
      </header>
      <main class="flex-1 flex flex-col min-h-0 bg-gray-50 dark:bg-gray-900">
        <div v-if="explorerStore.editorMode === 'preview'" class="flex-1 overflow-y-auto">
          <MdPreview 
            :modelValue="explorerStore.activeFileContent || 'Nothing to preview.'" 
            :theme="theme"
            class="p-4 prose-lg dark:prose-invert max-w-none bg-transparent"
          />
        </div>
        <MdEditor 
          v-else-if="explorerStore.activeFile"
          v-model="explorerStore.activeFileContent"
          :theme="theme"
          :preview="false"
          :toolbarsExclude="['github']"
          class="flex-1"
        />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, computed, onMounted, onUnmounted } from 'vue';
import { useKnowledgeExplorerStore } from '../../stores/knowledgeExplorer';
import { useSettingsStore } from '../../stores/settings';
import { MdEditor, MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import 'md-editor-v3/lib/style.css';
import { Loader2, FileSearch, PanelLeftClose, PanelLeftOpen } from 'lucide-vue-next';

defineProps({
  isExplorerVisible: {
    type: Boolean,
    default: true,
  }
});

defineEmits(['toggle-explorer']);

const explorerStore = useKnowledgeExplorerStore();
const settingsStore = useSettingsStore();
const isEditingFilename = ref(false);
const editableFilename = ref('');
const filenameInputRef = ref<HTMLInputElement | null>(null);

const theme = computed(() => {
  const currentTheme = settingsStore.settings?.appearance.theme;
  if (currentTheme === 'dark') return 'dark';
  if (currentTheme === 'light') return 'light';
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
});

const handleSave = () => {
  // Log Point: Check content right before calling the store action.
  console.log("[DEBUG_WIKILINK] KnowledgeEditor.vue handleSave, content:", explorerStore.activeFileContent);
  explorerStore.saveActiveFile();
};

const handleKeyDown = (event: KeyboardEvent) => {
  if ((event.metaKey || event.ctrlKey) && event.key === 's') {
    event.preventDefault();
    if (explorerStore.activeFile && explorerStore.editorMode === 'edit') {
      handleSave();
    }
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
});

watch(() => explorerStore.activeFile, (newFile) => {
  if (newFile) {
    editableFilename.value = newFile.title;
  }
});

const startEditingFilename = () => {
  if (!explorerStore.activeFile) return;
  isEditingFilename.value = true;
  nextTick(() => {
    filenameInputRef.value?.focus();
    filenameInputRef.value?.select();
  });
};

const finishEditingFilename = () => {
  if (isEditingFilename.value) {
    isEditingFilename.value = false;
    if (editableFilename.value && explorerStore.activeFile && editableFilename.value !== explorerStore.activeFile.title) {
      explorerStore.renameFile(explorerStore.activeFile.key, editableFilename.value);
    }
  }
};

const cancelEditingFilename = () => {
  isEditingFilename.value = false;
  if (explorerStore.activeFile) {
    editableFilename.value = explorerStore.activeFile.title;
  }
};

const toggleMode = () => {
  const newMode = explorerStore.editorMode === 'edit' ? 'preview' : 'edit';
  explorerStore.setEditorMode(newMode);
};
</script>