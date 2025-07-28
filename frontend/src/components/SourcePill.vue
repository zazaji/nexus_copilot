<!-- frontend/src/components/SourcePill.vue -->
<template>
  <n-popover trigger="click" placement="top-start" :style="{ maxWidth: '400px' }">
    <template #trigger>
      <button
        class="flex items-center space-x-1.5 px-2.5 py-1 bg-gray-100 dark:bg-gray-700/50 hover:bg-gray-200 dark:hover:bg-gray-600/50 rounded-full text-xs transition-colors cursor-pointer"
      >
        <component :is="isOnlineSource ? Globe : FileText" class="w-3 h-3 text-blue-500" />
        <span class="font-medium text-gray-700 dark:text-gray-300 truncate max-w-xs">{{ source.source_name }}</span>
        <span class="font-mono text-gray-500">{{ source.score.toFixed(2) }}</span>
      </button>
    </template>
    <div class="p-2">
      <header class="font-semibold text-base mb-2 flex items-center space-x-2">
        <component :is="isOnlineSource ? Globe : FileText" class="w-4 h-4 text-blue-500" />
        <span>{{ source.source_name }}</span>
      </header>
      <div class="text-sm text-gray-600 dark:text-gray-300 whitespace-pre-wrap max-h-60 overflow-y-auto font-mono bg-gray-50 dark:bg-gray-800 p-2 rounded-md">
        {{ source.content_snippet }}
      </div>
      <footer class="mt-3 flex justify-end space-x-2">
        <button 
          v-if="isHttpUrl"
          @click="openInBrowser"
          class="flex items-center space-x-1.5 px-3 py-1 text-xs bg-gray-200 dark:bg-gray-600 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500"
        >
          <ExternalLink class="w-3 h-3" />
          <span>Open in Browser</span>
        </button>
        <button 
          v-if="!isOnlineSource"
          @click="$emit('request-open-in-main', source)"
          class="px-3 py-1 text-xs bg-blue-500 text-white rounded-md hover:bg-blue-600"
        >
          Open in Knowledge Base
        </button>
      </footer>
    </div>
  </n-popover>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { KnowledgeSource } from '../types';
import { FileText, Globe, ExternalLink } from 'lucide-vue-next';
import { NPopover } from 'naive-ui';
import { openExternalLink } from '../lib/api';

const props = defineProps({
  source: {
    type: Object as PropType<KnowledgeSource>,
    required: true,
  },
});

defineEmits(['request-open-in-main']);

const isHttpUrl = computed(() => {
    const path = props.source.file_path;
    return path.startsWith('http://') || path.startsWith('https://');
});

const isOnlineSource = computed(() => {
    const path = props.source.file_path;
    return path.startsWith('online-kb://') || isHttpUrl.value || path === 'internet_search';
});

const openInBrowser = () => {
  if (isHttpUrl.value) {
    openExternalLink(props.source.file_path);
  }
};
</script>