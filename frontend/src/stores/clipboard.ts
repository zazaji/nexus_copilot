// frontend/src/stores/clipboard.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ClipboardItem } from '../types';
import { getClipboardHistory, clearClipboardHistory, deleteClipboardItem, deleteClipboardItems, onClipboardItemAdded, toggleClipboardItemPinned } from '../lib/api';
import type { UnlistenFn } from '@tauri-apps/api/event';

const MAX_HISTORY_LENGTH = 200;
type FilterType = 'All' | 'Text' | 'Image' | 'Files';

export const useClipboardStore = defineStore('clipboard', () => {
  const history = ref<ClipboardItem[]>([]);
  const activeFilter = ref<FilterType>('All');
  const isInitialized = ref(false);
  let unlisten: UnlistenFn | null = null;

  const setupListener = async () => {
    if (unlisten) {
      unlisten();
    }
    unlisten = await onClipboardItemAdded((item) => {
      addHistoryItemFromEvent(item);
    });
  };
  setupListener();

  const filteredHistory = computed(() => {
    if (activeFilter.value === 'All') {
      return history.value;
    }
    const filter = activeFilter.value.toLowerCase() as 'text' | 'image' | 'files';
    return history.value.filter(item => item.type === filter);
  });

  async function initialize() {
    if (isInitialized.value) return;
    const persistedHistory = await getClipboardHistory();
    if (persistedHistory) {
        history.value = persistedHistory as ClipboardItem[];
    }
    isInitialized.value = true;
  }

  function setFilter(filter: FilterType) {
    activeFilter.value = filter;
  }

  function addHistoryItemFromEvent(item: ClipboardItem) {
    if (history.value.length > 0 && history.value[0].content === item.content) {
      return;
    }

    history.value.unshift(item);
    if (history.value.length > MAX_HISTORY_LENGTH) {
      const removedItem = history.value.pop();
      if (removedItem) {
        deleteClipboardItem(removedItem.id);
      }
    }
  }

  async function togglePin(id: string) {
    const item = history.value.find(item => item.id === id);
    if (item) {
      item.isPinned = !item.isPinned;
      await toggleClipboardItemPinned(id, item.isPinned);
      // Re-sort to reflect the change immediately
      history.value.sort((a, b) => {
        if (a.isPinned !== b.isPinned) {
          return a.isPinned ? -1 : 1;
        }
        return b.timestamp - a.timestamp;
      });
    }
  }

  async function removeItem(id: string) {
    const index = history.value.findIndex(item => item.id === id);
    if (index > -1) {
      history.value.splice(index, 1);
      await deleteClipboardItem(id);
    }
  }

  async function removeItems(ids: string[]) {
    const idSet = new Set(ids);
    history.value = history.value.filter(item => !idSet.has(item.id));
    await deleteClipboardItems(ids);
  }

  async function clearHistory() {
    await clearClipboardHistory();
    // After clearing, only pinned items remain in the DB. Refetch to update UI.
    const persistedHistory = await getClipboardHistory();
    history.value = persistedHistory || [];
  }

  return {
    history,
    activeFilter,
    filteredHistory,
    initialize,
    setFilter,
    addHistoryItemFromEvent,
    togglePin,
    clearHistory,
    removeItem,
    removeItems,
  };
});