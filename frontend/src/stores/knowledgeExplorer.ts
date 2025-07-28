// frontend/src/stores/knowledgeExplorer.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { listFilesInDirectory, readFileContent, saveFileContent, createUniqueFile, renameFile as apiRenameFile, moveFile as apiMoveFile, deleteFile as apiDeleteFile } from '../lib/api';
import type { FileNode, KnowledgeSource } from '../types';
import { useSettingsStore } from './settings';
import { useKnowledgeBaseStore } from './knowledgeBase';
import { useToasts } from '../composables/useToasts';
import { dirname, join } from '@tauri-apps/api/path';

export type EditorMode = 'preview' | 'edit';
export type KnowledgeBaseTab = 'Explorer' | 'Graph';
export type SearchSource = 'local' | string; // 'local' or an online KB ID

export const useKnowledgeExplorerStore = defineStore('knowledgeExplorer', () => {
  const settingsStore = useSettingsStore();
  const kbStore = useKnowledgeBaseStore();
  const { success, error } = useToasts();
  const { t } = useI18n();

  const activeTab = ref<KnowledgeBaseTab>('Explorer');
  const fileTree = ref<FileNode[]>([]);
  const isLoadingTree = ref(false);
  const activeFile = ref<FileNode | null>(null);
  const activeFileContent = ref('');
  const isLoadingFile = ref(false);
  const editorMode = ref<EditorMode>('preview');
  const activeOnlineResult = ref<KnowledgeSource | null>(null);

  const searchQuery = ref('');
  const searchSource = ref<SearchSource>('local');

  async function search() {
    if (!searchQuery.value.trim()) {
      kbStore.searchResults = [];
      return;
    }
    if (searchSource.value === 'local') {
        console.log(`[Explorer] Searching local files for: "${searchQuery.value}"`);
        await kbStore.search(searchQuery.value);
    } else {
        console.log(`[Explorer] Searching online KB "${searchSource.value}" for: "${searchQuery.value}"`);
        await kbStore.searchOnlineKb(searchSource.value, searchQuery.value);
    }
  }

  async function loadFileTree() {
    isLoadingTree.value = true;
    const rootDirs = settingsStore.settings?.knowledgeBase.indexedDirectories || [];
    const treeData: FileNode[] = [];

    for (const dirPath of rootDirs) {
      const dirName = dirPath.split(/[/\\]/).pop() || dirPath;
      const children = await listFilesInDirectory(dirPath);
      treeData.push({
        key: dirPath,
        title: dirName,
        isLeaf: false,
        children: children || [],
      });
    }
    fileTree.value = treeData;
    isLoadingTree.value = false;
  }

  async function selectFile(fileOrPath: FileNode | string) {
    const isPathString = typeof fileOrPath === 'string';
    const path = isPathString ? fileOrPath : fileOrPath.key;
    
    if (!path) return;
    console.log(`[Explorer] Selecting local file: ${path}`);

    activeTab.value = 'Explorer';
    isLoadingFile.value = true;
    activeFileContent.value = '';
    activeOnlineResult.value = null;
    
    const content = await readFileContent(path);
    
    activeFile.value = {
      key: path,
      title: path.split(/[/\\]/).pop() || path,
      isLeaf: true,
      fileType: isPathString ? 'text' : (fileOrPath as FileNode).fileType,
    };
    
    activeFileContent.value = content || '';
    editorMode.value = 'preview';
    isLoadingFile.value = false;
  }

  function selectOnlineResult(result: KnowledgeSource) {
    console.log(`[Explorer] Selecting online result: ${result.source_name} (${result.id})`);
    activeTab.value = 'Explorer';
    activeFile.value = null;
    activeFileContent.value = result.content_snippet;
    activeOnlineResult.value = result;
    editorMode.value = 'preview';
  }

  async function saveActiveFile() {
    if (!activeFile.value) return;
    console.log(activeFile.value.key,activeFileContent.value)
    await saveFileContent(activeFile.value.key, activeFileContent.value);
    success(t('knowledge.editor.saveSuccess', { title: activeFile.value.title }));
  }

  async function createFile(node: FileNode | null) {
    let parentDir: string;

    if (node) {
      parentDir = node.isLeaf ? await dirname(node.key) : node.key;
    } else {
      parentDir = settingsStore.settings?.knowledgeBase.defaultSaveDirectory || '';
    }

    if (!parentDir) {
      error("Cannot determine directory. Please add an indexed directory in Settings first.");
      return;
    }

    const createdPath = await createUniqueFile(parentDir);
    if (createdPath) {
      success(`Created new note.`);
      await loadFileTree();
      await selectFile(createdPath);
      setEditorMode('edit');
    } else {
      error('Failed to create note.');
    }
  }

  async function renameFile(oldPath: string, newName: string) {
    if (!oldPath || !newName.trim()) return;

    const parentDir = await dirname(oldPath);
    const newPath = await join(parentDir, newName.endsWith('.md') ? newName : `${newName}.md`);

    if (oldPath === newPath) return;

    const result = await apiRenameFile(oldPath, newPath);
    if (result === null) {
      success('File renamed successfully!');
      await loadFileTree();
      await selectFile(newPath);
    }
  }

  async function moveFile(oldPath: string, newParentDir: string) {
    const result = await apiMoveFile(oldPath, newParentDir);
    if (result === null) {
      success('File moved successfully!');
      clearActiveFile();
      await loadFileTree();
    }
  }

  async function deleteFile(path: string) {
    const result = await apiDeleteFile(path);
    if (result === null) {
        success('File deleted successfully!');
        if (activeFile.value?.key === path) {
            clearActiveFile();
        }
        await loadFileTree();
    }
  }

  function setEditorMode(mode: EditorMode) {
    editorMode.value = mode;
  }
  
  function clearActiveFile() {
    activeFile.value = null;
    activeFileContent.value = '';
    activeOnlineResult.value = null;
  }

  return {
    activeTab,
    fileTree,
    isLoadingTree,
    activeFile,
    activeFileContent,
    isLoadingFile,
    editorMode,
    activeOnlineResult,
    searchQuery,
    searchSource,
    search,
    loadFileTree,
    selectFile,
    selectOnlineResult,
    saveActiveFile,
    createFile,
    renameFile,
    moveFile,
    deleteFile,
    setEditorMode,
    clearActiveFile,
  };
});