<!-- frontend/src/components/JsonViewer.vue -->
<template>
  <div class="json-viewer bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md mt-1 text-xs">
    <template v-if="parseError">
      <pre class="whitespace-pre-wrap font-mono text-gray-600 dark:text-gray-300">{{ source }}</pre>
    </template>
    <template v-else>
      <JsonNode :data="parsedData" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, VNode } from 'vue';

const props = defineProps<{
  source: string;
}>();

const parseError = ref<Error | null>(null);

const parsedData = computed(() => {
  try {
    let jsonString = props.source.trim();
    const match = /```json\s*([\s\S]*?)\s*```/.exec(jsonString);
    if (match && match[1]) {
      jsonString = match[1];
    }
    const data = JSON.parse(jsonString);
    parseError.value = null;
    return data;
  } catch (e) {
    parseError.value = e as Error;
    return null;
  }
});

// --- Recursive Node Component (defined locally) ---
const JsonNode = (props: { data: any, level?: number }) => {
  const { data, level = 0 } = props;
  const isExpanded = ref(level < 2); // Auto-expand first 2 levels

  if (typeof data === 'object' && data !== null) {
    const isArray = Array.isArray(data);
    const entries = Object.entries(data);
    const bracketOpen = isArray ? '[' : '{';
    const bracketClose = isArray ? ']' : '}';
    const nodeType = isArray ? 'array' : 'object';

    return h('div', { class: `json-${nodeType}` }, [
      h('span', { 
        class: 'cursor-pointer',
        onClick: () => isExpanded.value = !isExpanded.value 
      }, `${bracketOpen}${!isExpanded.value ? '...' + bracketClose : ''}`),
      
      isExpanded.value && h('div', { class: 'ml-4' }, 
        entries.map(([key, value]) => 
          h('div', { key, class: 'json-entry' }, [
            !isArray && h('span', { class: 'font-semibold text-gray-500 dark:text-gray-400' }, `"${key}": `),
            h(JsonNode, { data: value, level: level + 1 })
          ])
        )
      ),
      
      isExpanded.value && h('span', {}, bracketClose)
    ]);
  } else {
    const type = typeof data;
    let displayValue = JSON.stringify(data);
    let valueClass = 'text-green-600 dark:text-green-400'; // Default for string
    if (type === 'number') valueClass = 'text-blue-600 dark:text-blue-400';
    if (type === 'boolean') valueClass = 'text-purple-600 dark:text-purple-400';
    if (data === null) {
      valueClass = 'text-gray-500';
      displayValue = 'null';
    }
    
    return h('span', { class: `json-value ${valueClass}` }, displayValue);
  }
};
</script>