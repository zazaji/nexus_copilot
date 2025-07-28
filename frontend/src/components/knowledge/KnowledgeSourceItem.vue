<!-- frontend/src/components/knowledge/KnowledgeSourceItem.vue -->
<template>
  <div 
    class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
  >
    <div class="flex items-start justify-between space-x-3">
      <div class="flex items-center space-x-3 min-w-0 flex-1">
        <component :is="isOnlineSource ? Globe : FileText" class="w-5 h-5 text-blue-500 flex-shrink-0" />
        <div class="min-w-0">
            <span class="font-semibold text-gray-800 dark:text-gray-200 truncate block" :title="item.source_name">{{ item.source_name }}</span>
            <p class="text-xs text-gray-400 dark:text-gray-500 truncate" :title="item.file_path">{{ item.file_path }}</p>
        </div>
      </div>
      <div class="flex items-center space-x-2 flex-shrink-0">
        <div class="text-xs font-mono px-2 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded">
          {{ item.score.toFixed(2) }}
        </div>
        <button @click.stop="isExpanded = !isExpanded" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600">
            <ChevronDown v-if="!isExpanded" class="w-4 h-4 text-gray-500" />
            <ChevronUp v-else class="w-4 h-4 text-gray-500" />
        </button>
      </div>
    </div>
    <p 
      class="mt-2 text-sm text-gray-600 dark:text-gray-400 pl-8 leading-relaxed cursor-pointer" 
      :class="{ 'line-clamp-2': !isExpanded }"
      @click="isExpanded = !isExpanded"
      v-html="highlightedSnippet"
    ></p>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref } from 'vue';
import type { KnowledgeSource } from '../../types';
import { FileText, ChevronDown, ChevronUp, Globe } from 'lucide-vue-next';

const props = defineProps({
  item: {
    type: Object as PropType<KnowledgeSource>,
    required: true,
  },
});

const isExpanded = ref(false);

const isOnlineSource = computed(() => props.item.file_path.startsWith('online-kb://'));

const highlightedSnippet = computed(() => {
  return props.item.content_snippet.replace(/\n/g, '<br>');
});
</script>