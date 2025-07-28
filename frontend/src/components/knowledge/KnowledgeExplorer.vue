<!-- frontend/src/components/knowledge/KnowledgeExplorer.vue -->
<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-800">
    <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 space-y-3">
      <div class="flex justify-between items-center">
        <h2 class="font-semibold text-lg">{{ $t('knowledge.explorerTitle') }}</h2>
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
          <div v-for="rootNode in explorerStore.fileTree" :key="rootNode.key" class="group/folder">
            <div 
              @click="toggleFolder(rootNode.key)"
              class="px-2 py-1 font-semibold text-sm flex items-center justify-between space-x-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
            >
              <div class="flex items-center space-x-2 flex-1 min-w-0" @dblclick.stop="startEditing(rootNode)">
                <component :is="collapsedFolders.has(rootNode.key) ? ChevronRight : ChevronDown" class="w-4 h-4 flex-shrink-0" />
                <Folder class="w-4 h-4 text-yellow-500" />
                <input v-if="editingNodeKey === rootNode.key" type="text" v-model="editingName" @blur="finishEditing" @keydown.enter.prevent="finishEditing" @keydown.esc="cancelEditing" class="bg-transparent focus:outline-none focus:ring-1 focus:ring-blue-500 rounded px-1 text-sm font-semibold" ref="editInputRef" />
                <span v-else class="truncate">{{ rootNode.title }}</span>
              </div>
              <div class="flex items-center opacity-0 group-hover/folder:opacity-100 transition-opacity">
                <button @click.stop="openFolderInExplorer(rootNode.key)" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" :title="$t('knowledge.openFolder')">
                  <FolderOpen class="w-4 h-4" />
                </button>
                <button @click.stop="explorerStore.createFile(rootNode)" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" :title="$t('knowledge.newNote')">
                  <FilePlus2 class="w-4 h-4" />
                </button>
              </div>
            </div>
            <div v-if="!collapsedFolders.has(rootNode.key)" class="pl-4">
              <div 
                v-for="childNode in rootNode.children" 
                :key="childNode.key"
                @click="handleFileClick(childNode, childNode.fileType)"
                @contextmenu.prevent="handleContextMenu($event, childNode)"
                class="group/file flex items-center justify-between space-x-2 px-2 py-1.5 rounded-md cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="{ 'bg-blue-50 dark:bg-blue-900/50': explorerStore.activeFile?.key === childNode.key }"
              >
                <div class="flex items-center space-x-2 truncate flex-1 min-w-0" @dblclick.stop="startEditing(childNode)">
                  <component :is="getFileIcon(childNode.fileType)" class="w-4 h-4 text-gray-500 ml-4" />
                  <input v-if="editingNodeKey === childNode.key" type="text" v-model="editingName" @blur="finishEditing" @keydown.enter.prevent="finishEditing" @keydown.esc="cancelEditing" class="bg-transparent focus:outline-none focus:ring-1 focus:ring-blue-500 rounded px-1 text-sm" ref="editInputRef" />
                  <span v-else class="text-sm truncate">{{ childNode.title }}</span>
                </div>
                <div class="flex items-center opacity-0 group-hover/file:opacity-100 transition-opacity">
                    <button @click.stop="handleDeleteFile(childNode)" class="p-1 rounded-md text-gray-400 hover:bg-red-100 hover:text-red-500 dark:hover:bg-red-900/50" :title="$t('knowledge.deleteFile')">
                        <Trash2 class="w-4 h-4" />
                    </button>
                </div>
              </div>
            </div>
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
import { ref, nextTick, computed, watch, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useKnowledgeExplorerStore } from '../../stores/knowledgeExplorer';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { FilePlus2, Folder, FileText, Loader2, Search, Trash2, ChevronDown, ChevronRight, FolderOpen, FileSignature, Presentation } from 'lucide-vue-next';
import KnowledgeSourceItem from '../KnowledgeSourceItem.vue';
import KnowledgeSourceItemSkeleton from '../KnowledgeSourceItemSkeleton.vue';
import type { FileNode, KnowledgeSource } from '../../types';
import { NDropdown, NSelect } from 'naive-ui';
import { dirname } from '@tauri-apps/api/path';
import { showInFolder } from '../../lib/api';

const { t } = useI18n();
const explorerStore = useKnowledgeExplorerStore();
const kbStore = useKnowledgeBaseStore();
const editingNodeKey = ref<string | null>(null);
const editingName = ref('');
const originalName = ref('');
const editInputRef = ref<HTMLInputElement | null>(null);
const collapsedFolders = ref(new Set<string>());

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

watch(() => explorerStore.fileTree, (newTree) => {
  if (newTree && newTree.length > 0) {
    newTree.forEach(rootNode => {
      collapsedFolders.value.add(rootNode.key);
    });
  }
}, { immediate: true, deep: true });

const showDropdown = ref(false);
const x = ref(0);
const y = ref(0);
const selectedNodeKey = ref<string | null>(null);

const getFileIcon = (fileType?: string) => {
    switch(fileType) {
        case 'pdf': return FileText;
        case 'doc': return FileSignature;
        case 'ppt': return Presentation;
        default: return FileText;
    }
};

const handleFileClick = (fileOrNode: FileNode | string, fileType?: string) => {
    const path = typeof fileOrNode === 'string' ? fileOrNode : fileOrNode.key;
    if (fileType === 'text') {
        explorerStore.selectFile(fileOrNode);
    } else {
            showInFolder(path);
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
  if (collapsedFolders.value.has(key)) {
    collapsedFolders.value.delete(key);
  } else {
    collapsedFolders.value.add(key);
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

const handleContextMenu = (e: MouseEvent, node: FileNode) => {
  e.preventDefault();
  showDropdown.value = false;
  selectedNodeKey.value = node.key;
  nextTick().then(() => {
    showDropdown.value = true;
    x.value = e.clientX;
    y.value = e.clientY;
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
  editingName.value = node.title;
  originalName.value = node.title;
  nextTick(() => {
    editInputRef.value?.focus();
  });
};

const finishEditing = () => {
  if (editingNodeKey.value && editingName.value && editingName.value !== originalName.value) {
    explorerStore.renameFile(editingNodeKey.value, editingName.value);
  }
  cancelEditing();
};

const cancelEditing = () => {
  editingNodeKey.value = null;
  editingName.value = '';
  originalName.value = '';
};
</script>