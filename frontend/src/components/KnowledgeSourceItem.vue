<!-- frontend/src/components/KnowledgeSourceItem.vue -->
<template>
  <div 
    class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
  >
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-3 min-w-0">
        <FileText class="w-5 h-5 text-blue-500 flex-shrink-0" />
        <span class="font-semibold text-gray-800 dark:text-gray-200 truncate">{{ item.source_name }}</span>
      </div>
      <div class="text-xs font-mono px-2 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded">
        {{ item.score.toFixed(2) }}
      </div>
    </div>
    <p class="mt-2 text-sm text-gray-600 dark:text-gray-400 pl-8 leading-relaxed line-clamp-2" v-html="highlightedSnippet"></p>
    <p class="mt-2 text-xs text-gray-400 dark:text-gray-500 pl-8 truncate" :title="item.file_path">{{ item.file_path }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { KnowledgeSource } from '../types';
import { FileText } from 'lucide-vue-next';

const props = defineProps({
  item: {
    type: Object as PropType<KnowledgeSource>,
    required: true,
  },
});

const highlightedSnippet = computed(() => {
  return props.item.content_snippet.replace(/\n/g, '<br>');
});
</script>