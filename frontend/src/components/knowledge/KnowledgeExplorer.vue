<!-- frontend/src/components/knowledge/KnowledgeExplorer.vue -->
<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-800">
    <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 space-y-3">
      <div class="flex justify-between items-center">
        <h2 class="font-semibold text-lg">{{ $t('knowledge.explorerTitle') }}</h2>
        <n-tooltip trigger="hover">
          <template #trigger>
            <button 
              @click="refreshTree" 
              :disabled="explorerStore.isLoadingTree"
              class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': explorerStore.isLoadingTree }" />
            </button>
          </template>
          Refresh File Tree
        </n-tooltip>
      </div>
      <div class="space-y-2">
        <n-select
            v-model:value="explorerStore.searchSource"
            :options="searchSourceOptions"
            size="small"
        />
        <div class="relative">
            <input
            type="text"
            v-model="explorerStore.searchQuery"
            @keydown.enter="triggerSearch"
            :placeholder="$t('knowledge.searchPlaceholder')"
            class="w-full pl-4 pr-10 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button @click="triggerSearch" class="absolute right-2 top-1/2 -translate-y-1/2 p-2 text-gray-400 hover:text-blue-500 rounded-full">
                <Search class="w-5 h-5" />
            </button>
        </div>
      </div>
    </header>
    <div class="flex-1 overflow-y-auto p-2" @click="showDropdown = false">
      <!-- Search Results View -->
      <div v-if="explorerStore.searchQuery">
        <div v-if="kbStore.isSearching" class="divide-y divide-gray-200 dark:divide-gray-700">
          <KnowledgeSourceItemSkeleton v-for="i in 5" :key="i" />
        </div>
        <transition-group v-else-if="kbStore.searchResults.length > 0" name="list" tag="div" class="divide-y divide-gray-200 dark:divide-gray-700">
          <div 
            v-for="item in kbStore.searchResults" 
            :key="item.id" 
            @click="handleSearchResultClick(item)"
            class="cursor-pointer"
            :class="{ 'bg-blue-50 dark:bg-blue-900/50': isResultActive(item) }"
          >
            <KnowledgeSourceItem :item="item" />
          </div>
        </transition-group>
        <div v-else class="text-center text-gray-500 pt-10">{{ $t('knowledge.noResults') }}</div>
      </div>
      <!-- File Tree View -->
      <div v-else>
        <div v-if="explorerStore.isLoadingTree">
          <Loader2 class="w-6 h-6 animate-spin mx-auto mt-10" />
        </div>
        <div v-else>
          <div v-for="rootNode in explorerStore.fileTree" :key="rootNode.key">
            <FileTreeNode
              :node="rootNode"
              :level="0"
              :active-file-key="explorerStore.activeFile?.key"
              :expanded-keys="expandedKeys"
              :editing-key="editingNodeKey"
              @select-file="handleFileClick"
              @toggle-expand="toggleFolder"
              @context-menu="handleContextMenu"
              @create-file="explorerStore.createFile"
              @delete-file="handleDeleteFile"
              @start-rename="startEditing"
              @finish-rename="finishEditing"
              @open-in-explorer="openFolderInExplorer"
            />
          </div>
        </div>
      </div>
    </div>
    <n-dropdown
      placement="bottom-start"
      trigger="manual"
      :x="x"
      :y="y"
      :options="dropdownOptions"
      :show="showDropdown"
      :on-clickoutside="() => showDropdown = false"
      @select="handleDropdownSelect"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useKnowledgeExplorerStore } from '../../stores/knowledgeExplorer';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { Loader2, Search, RefreshCw } from 'lucide-vue-next';
import KnowledgeSourceItem from '../KnowledgeSourceItem.vue';
import KnowledgeSourceItemSkeleton from '../KnowledgeSourceItemSkeleton.vue';
import FileTreeNode from './FileTreeNode.vue';
import type { FileNode, KnowledgeSource } from '../../types';
import { NDropdown, NSelect, NTooltip } from 'naive-ui';
import { dirname } from '@tauri-apps/api/path';
import { showInFolder } from '../../lib/api';

const { t } = useI18n();
const explorerStore = useKnowledgeExplorerStore();
const kbStore = useKnowledgeBaseStore();
const editingNodeKey = ref<string | null>(null);
const expandedKeys = ref(new Set<string>());

onMounted(() => {
    kbStore.fetchOnlineKbs();
});

const searchSourceOptions = computed(() => {
    const options = [{ label: 'Local Files', value: 'local' }];
    kbStore.onlineKbs.forEach(kb => {
        options.push({ label: kb.name, value: kb.id });
    });
    return options;
});

const showDropdown = ref(false);
const x = ref(0);
const y = ref(0);
const selectedNodeKey = ref<string | null>(null);

const refreshTree = () => {
  explorerStore.loadFileTree();
};

const handleFileClick = (fileNode: FileNode) => {
    if (fileNode.fileType === 'text') {
        explorerStore.selectFile(fileNode);
    } else if (fileNode.key) {
        showInFolder(fileNode.key);
    }
};

const handleSearchResultClick = (item: KnowledgeSource) => {
    console.log(`[Explorer] Clicked search result:`, item);
    if (item.file_path.startsWith('online-kb://')) {
        explorerStore.selectOnlineResult(item);
    } else {
        explorerStore.selectFile(item.file_path);
    }
};

const isResultActive = (item: KnowledgeSource) => {
    if (item.file_path.startsWith('online-kb://')) {
        return explorerStore.activeOnlineResult?.id === item.id;
    } else {
        return explorerStore.activeFile?.key === item.file_path;
    }
};

const toggleFolder = (key: string) => {
  if (expandedKeys.value.has(key)) {
    expandedKeys.value.delete(key);
  } else {
    expandedKeys.value.add(key);
  }
};

const openFolderInExplorer = (folderPath: string) => {
  showInFolder(folderPath);
};

const dropdownOptions = computed(() => {
  if (!selectedNodeKey.value) return [];
  
  const moveOptions = explorerStore.fileTree
    .filter(rootNode => {
      const parentDir = dirname(selectedNodeKey.value || '');
      return rootNode.key !== parentDir;
    })
    .map(rootNode => ({
      label: `Move to "${rootNode.title}"`,
      key: `move-to::${rootNode.key}`
    }));

  return moveOptions;
});

const handleContextMenu = (payload: { event: MouseEvent, node: FileNode }) => {
  payload.event.preventDefault();
  showDropdown.value = false;
  selectedNodeKey.value = payload.node.key;
  nextTick().then(() => {
    showDropdown.value = true;
    x.value = payload.event.clientX;
    y.value = payload.event.clientY;
  });
};

const handleDropdownSelect = (key: string) => {
  showDropdown.value = false;
  if (!selectedNodeKey.value) return;

  if (key.startsWith('move-to::')) {
    const newParentDir = key.substring('move-to::'.length);
    explorerStore.moveFile(selectedNodeKey.value, newParentDir);
  }
};

const handleDeleteFile = (node: FileNode) => {
    explorerStore.deleteFile(node.key);
};

const triggerSearch = () => {
  explorerStore.search();
};

const startEditing = (node: FileNode) => {
  editingNodeKey.value = node.key;
};

const finishEditing = (payload: { oldKey: string, newName: string }) => {
  if (payload.newName && payload.newName !== payload.oldKey.split(/[/\\]/).pop()) {
    explorerStore.renameFile(payload.oldKey, payload.newName);
  }
  editingNodeKey.value = null;
};
</script>