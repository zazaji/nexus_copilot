<!-- frontend/src/components/knowledge/KnowledgeFilesView.vue -->
<template>
  <div class="flex-1 flex flex-col bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="relative">
        <Search class="w-5 h-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
        <input
          type="text"
          v-model="searchQuery"
          @input="onSearch"
          placeholder="Search your indexed files..."
          class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </div>
    <div class="flex-1 overflow-y-auto">
      <div v-if="kbStore.isSearching" class="divide-y divide-gray-200 dark:divide-gray-700">
        <KnowledgeSourceItemSkeleton v-for="i in 5" :key="i" />
      </div>
      <transition-group v-else-if="kbStore.searchResults.length > 0" name="list" tag="div" class="divide-y divide-gray-200 dark:divide-gray-700">
        <KnowledgeSourceItem v-for="item in kbStore.searchResults" :key="item.id" :item="item" />
      </transition-group>
      <div v-else class="flex flex-col items-center justify-center h-full text-center text-gray-500 p-8">
        <FileSearch class="w-16 h-16 mb-4 text-gray-400" />
        <h2 class="text-xl font-semibold">Search Your Knowledge</h2>
        <p class="mt-1">Results from your indexed files will appear here.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { Search, FileSearch } from 'lucide-vue-next';
import KnowledgeSourceItem from '../KnowledgeSourceItem.vue';
import KnowledgeSourceItemSkeleton from '../KnowledgeSourceItemSkeleton.vue';
import { debounce } from 'lodash-es';

const kbStore = useKnowledgeBaseStore();
const searchQuery = ref('');

const onSearch = debounce(() => {
  kbStore.search(searchQuery.value);
}, 300);
</script>

<style scoped>
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>