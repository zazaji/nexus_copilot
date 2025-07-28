// frontend/src/stores/knowledgeBase.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { KnowledgeNote, KnowledgeGraphData, KnowledgeSource, OnlineKnowledgeBase } from '../types';
import { 
    openDirectoryPicker, 
    startIndexing, 
    removeIndexedDirectory, 
    getKnowledgeGraphData, 
    onIndexingProgress, 
    searchLocalKb,
    searchOnlineKb as apiSearchOnlineKb,
    listOnlineKbs,
    addOnlineKb as apiAddOnlineKb,
    updateOnlineKb as apiUpdateOnlineKb,
    deleteOnlineKb as apiDeleteOnlineKb,
} from '../lib/api';
import { useSettingsStore } from './settings';
import { useToasts } from '../composables/useToasts';

export const useKnowledgeBaseStore = defineStore('knowledgeBase', () => {
  const settingsStore = useSettingsStore();
  const { success, error, info } = useToasts();
  
  const notes = ref<KnowledgeNote[]>([]);
  const graphData = ref<KnowledgeGraphData | null>(null);
  const onlineKbs = ref<OnlineKnowledgeBase[]>([]);

  const isIndexing = ref(false);
  const indexingProgress = ref<{ file: string; progress: number } | null>(null);

  const searchResults = ref<KnowledgeSource[]>([]);
  const isSearching = ref(false);

  async function search(query: string) {
    if (!query.trim()) {
      searchResults.value = [];
      return;
    }
    isSearching.value = true;
    const settings = settingsStore.settings?.knowledgeBase;
    if (!settings) {
        error("Knowledge base settings not loaded.");
        isSearching.value = false;
        return;
    }
    const results = await searchLocalKb(query, settings.topK, settings.scoreThreshold);
    searchResults.value = results || [];
    isSearching.value = false;
  }

  async function searchOnlineKb(kbId: string, query: string) {
    if (!query.trim()) {
        searchResults.value = [];
        return;
    }
    isSearching.value = true;
    const settings = settingsStore.settings?.knowledgeBase;
    if (!settings) {
        error("Knowledge base settings not loaded.");
        isSearching.value = false;
        return;
    }
    const results = await apiSearchOnlineKb(kbId, query, settings.topK, settings.scoreThreshold);
    searchResults.value = results || [];
    isSearching.value = false;
  }

  async function addDirectory() {
    const response = await openDirectoryPicker();
    if (response?.success && response.path) {
      const selectedPath = response.path;
      if (settingsStore.settings) {
        if (!settingsStore.settings.knowledgeBase.indexedDirectories.includes(selectedPath)) {
          isIndexing.value = true;
          indexingProgress.value = { file: 'Starting...', progress: 0 };
          await startIndexing(selectedPath);
          
          const updatedSettings = JSON.parse(JSON.stringify(settingsStore.settings));
          updatedSettings.knowledgeBase.indexedDirectories.push(selectedPath);
          await settingsStore.saveSettings(updatedSettings);
          success(`Started indexing ${selectedPath}`);
        } else {
          info(`${selectedPath} is already indexed.`);
        }
      }
    } else if (response?.error) {
      error(`Could not pick directory: ${response.error}`);
    }
  }

  async function removeDirectory(path: string) {
      await removeIndexedDirectory(path);
      await settingsStore.fetchSettings();
      success(`Directory removed and data deleted.`);
  }


  async function fetchGraphData() {
    const data = await getKnowledgeGraphData();
    if (data) {
      graphData.value = data;
    }
  }

  async function fetchOnlineKbs() {
    const kbs = await listOnlineKbs();
    if (kbs) {
        onlineKbs.value = kbs;
    }
  }

  async function addOnlineKb(kb: OnlineKnowledgeBase) {
    await apiAddOnlineKb(kb);
    await fetchOnlineKbs();
    success("Online knowledge base added.");
  }

  async function updateOnlineKb(kb: OnlineKnowledgeBase) {
    await apiUpdateOnlineKb(kb);
    await fetchOnlineKbs();
    success("Online knowledge base updated.");
  }

  async function deleteOnlineKb(id: string) {
        await apiDeleteOnlineKb(id);
        await fetchOnlineKbs();
        success("Online knowledge base deleted.");
  }

  onIndexingProgress((payload) => {
    isIndexing.value = true;
    indexingProgress.value = payload;
    if (payload.progress >= 100) {
      setTimeout(() => {
        isIndexing.value = false;
        indexingProgress.value = null;
        success('Indexing finished!');
      }, 2000);
    }
  });

  return {
    notes,
    graphData,
    onlineKbs,
    isIndexing,
    indexingProgress,
    searchResults,
    isSearching,
    search,
    searchOnlineKb,
    addDirectory,
    removeDirectory,
    fetchGraphData,
    fetchOnlineKbs,
    addOnlineKb,
    updateOnlineKb,
    deleteOnlineKb,
  };
});