<!-- frontend/src/views/ClipboardHistoryView.vue -->
<template>
  <div class="flex-1 flex flex-col p-6 bg-gray-50 dark:bg-gray-900/50 overflow-hidden">
    <header class="flex justify-between items-center mb-4">
      <div>
        <h1 class="text-2xl font-bold">{{ $t('clipboard.title') }}</h1>
        <div class="flex items-center space-x-2 mt-2">
          <button 
            v-for="filter in filters" :key="filter"
            @click="clipboardStore.setFilter(filter)"
            class="px-3 py-1 text-xs rounded-full"
            :class="clipboardStore.activeFilter === filter ? 'bg-blue-500 text-white' : 'bg-gray-200 dark:bg-gray-700'"
          >
            {{ filter }}
          </button>
        </div>
      </div>
      <div class="flex items-center space-x-3">
        <button 
          @click="deleteCurrentPage" 
          class="px-4 py-2 text-sm bg-yellow-500 text-white rounded-lg hover:bg-yellow-600 disabled:bg-gray-400"
          :disabled="paginatedHistory.length === 0"
        >
          {{ $t('clipboard.deletePage') }}
        </button>
        <button 
          @click="clearAll" 
          class="px-4 py-2 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600 disabled:bg-gray-400"
          :disabled="clipboardStore.history.length === 0"
        >
          {{ $t('clipboard.clearAll') }}
        </button>
      </div>
    </header>
    <div class="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow-sm flex flex-col">
      <div class="flex-1 overflow-y-auto">
        <div v-if="paginatedHistory.length > 0" class="divide-y divide-gray-200 dark:divide-gray-700">
          <ClipboardItem v-for="item in paginatedHistory" :key="item.id" :item="item" />
        </div>
        <div v-else class="flex flex-col items-center justify-center h-full text-center text-gray-500 p-8">
          <ClipboardPaste class="w-16 h-16 mb-4 text-gray-400" />
          <h2 class="text-xl font-semibold">{{ $t('clipboard.empty') }}</h2>
          <p class="mt-1">{{ $t('clipboard.emptyDesc') }}</p>
        </div>
      </div>
      <div v-if="totalPages > 1" class="p-2 border-t border-gray-200 dark:border-gray-700 flex justify-center items-center space-x-4">
        <button @click="prevPage" :disabled="currentPage === 1" class="px-3 py-1 rounded disabled:opacity-50 hover:bg-gray-100 dark:hover:bg-gray-700">{{ $t('clipboard.previous') }}</button>
        <span class="text-sm">{{ $t('clipboard.page', { currentPage, totalPages }) }}</span>
        <button @click="nextPage" :disabled="currentPage === totalPages" class="px-3 py-1 rounded disabled:opacity-50 hover:bg-gray-100 dark:hover:bg-gray-700">{{ $t('clipboard.next') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useClipboardStore } from '../stores/clipboard';
import ClipboardItem from '../components/ClipboardItem.vue';
import { ClipboardPaste } from 'lucide-vue-next';

const clipboardStore = useClipboardStore();
const filters: ('All' | 'Text' | 'Image')[] = ['All', 'Text', 'Image'];

const currentPage = ref(1);
const itemsPerPage = 10;

const totalPages = computed(() => {
  return Math.ceil(clipboardStore.filteredHistory.length / itemsPerPage);
});

const paginatedHistory = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return clipboardStore.filteredHistory.slice(start, end);
});

watch(() => clipboardStore.activeFilter, () => {
  currentPage.value = 1;
});

watch(totalPages, (newTotal) => {
    if (currentPage.value > newTotal) {
        currentPage.value = newTotal || 1;
    }
});

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

const deleteCurrentPage = () => {
  const idsToDelete = paginatedHistory.value.map(item => item.id);
  clipboardStore.removeItems(idsToDelete);
};

const clearAll = () => {
  clipboardStore.clearHistory();
};
</script>