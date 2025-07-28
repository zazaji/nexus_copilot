<!-- frontend/src/components/ChatSuggestions.vue -->
<template>
  <div v-if="hasContent" class=" ml-13 mt-1">
    <!--Next Steps-->
    <div class="flex flex-wrap gap-2">
      <button
        v-if="pythonCode"
        @click="$emit('execute-code', pythonCode)"
        class="px-3 py-1.5 bg-green-100 dark:bg-green-900/50 border border-green-300 dark:border-green-700 hover:bg-green-200 dark:hover:bg-green-800 rounded-lg text-sm transition-colors text-green-800 dark:text-green-300 flex items-center space-x-2"
      >
        <Play class="w-4 h-4" />
        <span>Execute Code</span>
      </button>
      <button
        v-if="hasHtmlFile"
        @click="$emit('preview-files', codeBlocks)"
        class="px-3 py-1.5 bg-indigo-100 dark:bg-indigo-900/50 border border-indigo-300 dark:border-indigo-700 hover:bg-indigo-200 dark:hover:bg-indigo-800 rounded-lg text-sm transition-colors text-indigo-800 dark:text-indigo-300 flex items-center space-x-2"
      >
        <Eye class="w-4 h-4" />
        <span>Preview</span>
      </button>
      <button
        v-for="suggestion in suggestions"
        :key="suggestion"
        @click="$emit('send', suggestion)"
        class="px-3 py-1.5 bg-gray-50 dark:bg-gray-700/50 border border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg text-sm transition-colors"
      >
        {{ suggestion }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import { Play, Eye } from 'lucide-vue-next';

interface ParsedContentPart {
  type: 'text' | 'code';
  content: string;
  language?: string;
}

const props = defineProps<{
  suggestions: string[] | undefined;
  parsedContent: ParsedContentPart[] | undefined;
}>();

defineEmits(['send', 'execute-code', 'preview-files']);

const codeBlocks = computed(() => {
    return props.parsedContent?.filter(part => part.type === 'code') || [];
});

const pythonCode = computed<string | null>(() => {
  const pythonBlock = codeBlocks.value.find(block => block.language === 'python');
  return pythonBlock ? pythonBlock.content : null;
});

const hasHtmlFile = computed(() => {
    return codeBlocks.value.some(block => {
        const firstLine = block.content.split('\n')[0];
        const match = firstLine.match(/^(?:#|<!--)\s*([a-zA-Z0-9_./-]+\.html)\s*(?:-->)?$/);
        return !!match;
    });
});

const hasContent = computed(() => {
  return !!pythonCode.value || hasHtmlFile.value || (props.suggestions && props.suggestions.length > 0);
});
</script>