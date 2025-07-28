<!-- frontend/src/components/CodeBlock.vue -->
<template>
  <div class="code-block relative group bg-gray-900 dark:bg-black/50 rounded-lg my-2">
    <div class="flex justify-between items-center px-4 py-2 bg-gray-800/50 dark:bg-black/20 rounded-t-lg">
      <span class="text-xs font-sans text-gray-400">{{ language }}</span>
      <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <button 
          v-if="filename"
          @click="writeFile" 
          class="p-1.5 text-gray-300 hover:bg-gray-700 rounded-md" 
          title="Save File to Workspace"
        >
          <FileDown class="w-4 h-4" />
        </button>
        <button 
          @click="runCode" 
          :disabled="isExecuting || language !== 'python'" 
          class="p-1.5 text-gray-300 hover:bg-gray-700 rounded-md disabled:text-gray-500 disabled:cursor-not-allowed" 
          title="Run Python Code"
        >
          <Loader2 v-if="isExecuting" class="w-4 h-4 animate-spin" />
          <Play v-else class="w-4 h-4" />
        </button>
        <button @click="copyCode" class="p-1.5 text-gray-300 hover:bg-gray-700 rounded-md" title="Copy Code">
          <Copy class="w-4 h-4" />
        </button>
      </div>
    </div>
    <pre class="p-4 overflow-x-auto"><code ref="codeEl" :class="`language-${language}`" v-text="code"></code></pre>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';
import { Play, Copy, Loader2, FileDown } from 'lucide-vue-next';
import { useToasts } from '../composables/useToasts';

const props = defineProps<{
  code: string;
  language: string;
  isExecuting: boolean;
}>();

const emit = defineEmits(['execute', 'write-file']);

const { success, error } = useToasts();
const codeEl = ref<HTMLElement | null>(null);

const highlight = () => {
  if (codeEl.value) {
    hljs.highlightElement(codeEl.value);
  }
};

onMounted(highlight);
watch(() => props.code, highlight);

const filename = computed(() => {
  const firstLine = props.code.split('\n')[0];
  const match = firstLine.match(/^(?:#|<!--)\s*([a-zA-Z0-9_./-]+\.[a-zA-Z0-9]+)\s*(?:-->)?$/);
  return match ? match[1] : null;
});

const copyCode = async () => {
  try {
    await navigator.clipboard.writeText(props.code);
    success('Code copied to clipboard!');
  } catch (err) {
    error('Failed to copy code.');
  }
};

const runCode = () => {
  if (props.language === 'python') {
    emit('execute', props.code);
  } else {
    error('Execution is only supported for Python code blocks at the moment.');
  }
};

const writeFile = () => {
  if (filename.value) {
    emit('write-file', { filename: filename.value, content: props.code });
  }
};
</script>

<style scoped>
/* Override default pre/code styles for better integration */
pre {
  margin: 0;
  padding: 0;
}
code {
  font-family: 'Fira Code', 'Courier New', monospace;
  font-size: 0.875rem;
}
</style>