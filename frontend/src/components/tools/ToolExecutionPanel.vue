<!-- frontend/src/components/tools/ToolExecutionPanel.vue -->
<template>
  <div class="p-6 h-full flex flex-col">
    <div v-if="!tool" class="flex-1 flex items-center justify-center text-gray-500">
      <p>Select a tool from the list to get started.</p>
    </div>
    <div v-else class="flex-1 flex flex-col overflow-y-auto">
      <div class="flex items-center justify-between mb-2">
        <h2 class="text-xl font-bold">{{ tool.name }}</h2>
        <span class="text-xs font-mono px-2 py-1 rounded-full" :class="tool.type === 'dynamic' ? 'bg-purple-100 dark:bg-purple-900/50 text-purple-700 dark:text-purple-300' : 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'">
          {{ $t(`tools.${tool.type}`) }}
        </span>
      </div>
      <div class="flex items-center space-x-2 mb-4">
        <p class="text-sm text-gray-500">{{ tool.description }}</p>
        <button @click="previewCode" class="text-blue-500 hover:underline text-sm flex items-center space-x-1">
            <Code2 class="w-4 h-4" />
            <span>Preview Code</span>
        </button>
      </div>
      <p class="text-xs font-mono text-gray-400 mb-6" :title="tool.scriptPath">{{ tool.scriptPath }}</p>

      <div class="space-y-4">
        <div v-if="tool.type === 'configured' && tool.parameters.length > 0" class="space-y-3">
          <div v-for="param in tool.parameters" :key="param.name">
            <label :for="`${tool.id}-${param.name}`" class="block text-sm font-medium text-gray-700 dark:text-gray-300">{{ param.label }} <span v-if="param.required" class="text-red-500">*</span></label>
            <input v-if="param.paramType === 'text'" type="text" :id="`${tool.id}-${param.name}`" v-model="formInputs[param.name]" class="mt-1 input-style">
            <textarea v-if="param.paramType === 'textarea'" :id="`${tool.id}-${param.name}`" v-model="formInputs[param.name]" rows="3" class="mt-1 input-style"></textarea>
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Input (stdin)</label>
          <textarea v-model="formInputs['stdin']" rows="5" class="mt-1 input-style font-mono"></textarea>
        </div>

        <button @click="run" class="w-full justify-center px-4 py-2 text-sm rounded-md bg-blue-600 text-white hover:bg-blue-700 flex items-center space-x-2" :disabled="isRunning">
          <Loader2 v-if="isRunning" class="w-4 h-4 animate-spin" />
          <Play v-else class="w-4 h-4" />
          <span>{{ $t('tools.run') }}</span>
        </button>

        <div v-if="result" class="mt-4 p-4 bg-gray-100 dark:bg-gray-900 rounded-lg">
          <h4 class="font-semibold mb-2">{{ $t('tools.output') }}</h4>
          <pre class="text-sm whitespace-pre-wrap font-mono max-h-80 overflow-y-auto">{{ result }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, PropType, onUnmounted } from 'vue';
import { executeTool, onToolOutput, onToolComplete, onToolError } from '../../lib/api';
import { useFilePreviewStore } from '../../stores/filePreview';
import { Play, Loader2, Code2 } from 'lucide-vue-next';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { UnifiedTool } from '../../stores/tools';

const props = defineProps({
  tool: {
    type: Object as PropType<UnifiedTool | null>,
    default: null,
  },
});

const filePreviewStore = useFilePreviewStore();
const formInputs = ref<Record<string, any>>({});
const result = ref<string | null>(null);
const isRunning = ref(false);
const runningTaskId = ref<string | null>(null);

let unlistenOutput: UnlistenFn | null = null;
let unlistenComplete: UnlistenFn | null = null;
let unlistenError: UnlistenFn | null = null;

const initializeForm = (tool: UnifiedTool | null) => {
  const newForm: Record<string, any> = { stdin: '' };
  if (tool && tool.type === 'configured') {
    tool.parameters.forEach(param => {
      newForm[param.name] = param.defaultValue || '';
    });
  }
  formInputs.value = newForm;
  result.value = null;
  isRunning.value = false;
};

watch(() => props.tool, (newTool) => {
  initializeForm(newTool);
}, { immediate: true });

const previewCode = () => {
    if (props.tool) {
        filePreviewStore.showPreview(props.tool.scriptPath);
    }
};

const run = async () => {
  if (!props.tool) return;
  isRunning.value = true;
  result.value = '';
  
  const params = { ...formInputs.value };
  const taskId = await executeTool(props.tool.id, params);
  if (taskId) {
    runningTaskId.value = taskId;
    listenToEvents(taskId);
  } else {
    result.value = 'Failed to start tool execution.';
    isRunning.value = false;
  }
};

const stopListening = () => {
    unlistenOutput?.();
    unlistenComplete?.();
    unlistenError?.();
    unlistenOutput = null;
    unlistenComplete = null;
    unlistenError = null;
};

const listenToEvents = async (taskId: string) => {
  stopListening();

  unlistenOutput = await onToolOutput((event) => {
    if (event.taskId === taskId) {
      result.value += event.chunk;
    }
  });
  unlistenComplete = await onToolComplete((event) => {
    if (event.taskId === taskId) {
      isRunning.value = false;
      runningTaskId.value = null;
      stopListening();
    }
  });
  unlistenError = await onToolError((event) => {
    if (event.taskId === taskId) {
      result.value += `\nERROR: ${event.payload}`;
      isRunning.value = false;
      runningTaskId.value = null;
      stopListening();
    }
  });
};

onUnmounted(() => {
    stopListening();
});
</script>

<style scoped>
.input-style {
  @apply mt-1 block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
</style>