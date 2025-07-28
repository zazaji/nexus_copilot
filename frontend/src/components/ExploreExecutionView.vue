<!-- frontend/src/components/ExploreExecutionView.vue -->
<template>
  <div>
    <div v-if="task.steps.length === 0 && task.status !== 'failed'" class="pl-9 text-sm text-gray-500 dark:text-gray-400 flex items-center space-x-2">
        <Loader2 class="w-4 h-4 animate-spin" />
        <span>Agent is thinking about the first step...</span>
    </div>
    <div v-else class="space-y-1 pl-9">
      <div v-for="(step, index) in task.steps" :key="step.id" class="flex flex-col text-sm">
        <div @click="toggleStep(index)" class="flex items-center space-x-3 cursor-pointer p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700/50">
            <div class="flex-shrink-0">
                <Loader2 v-if="step.status === 'running'" class="w-4 h-4 text-blue-400 animate-spin" />
                <CheckCircle2 v-else-if="step.status === 'completed'" class="w-4 h-4 text-green-400" />
                <XCircle v-else-if="step.status === 'failed'" class="w-4 h-4 text-red-400" />
                <CircleDashed v-else class="w-4 h-4 text-gray-500" />
            </div>
            <p class="font-medium text-gray-800 dark:text-gray-200 flex-1">Step {{ step.step_index }}: {{ step.action }}</p>
            <ChevronRight class="w-4 h-4 text-gray-400 transition-transform" :class="{ 'rotate-90': expandedSteps.has(index) }" />
        </div>
        <div v-if="expandedSteps.has(index)" class="mt-2 ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 space-y-2 pb-2">
          <div v-if="step.thought" class="bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md">
            <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Thought</p>
            <p class="text-gray-600 dark:text-gray-300 text-xs mt-1 italic">"{{ step.thought }}"</p>
          </div>
          <div v-if="step.action && step.status !== 'pending'" class="bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md">
            <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Action</p>
            <p class="font-mono text-xs text-blue-600 dark:text-blue-400 mt-1">{{ step.action }}</p>
          </div>
          <div v-if="step.observation" class="border-l-4 border-gray-300 dark:border-gray-600 pl-3 py-1">
              <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Observation</p>
              <JsonViewer :source="step.observation" />
          </div>
          <div v-if="step.result" class="border-l-4 border-green-400 dark:border-green-600 pl-3 py-1">
              <p class="text-xs font-semibold text-green-600 dark:text-green-400">Result</p>
              <div class="prose prose-sm dark:prose-invert max-w-none mt-1" v-html="renderedMarkdown(step.result || '')"></div>
          </div>
        </div>
      </div>
      <div v-if="task.status === 'running' || task.status === 'planning'" class="text-sm text-gray-500 dark:text-gray-400 flex items-center space-x-2 pt-2">
          <Loader2 class="w-4 h-4 animate-spin" />
          <span>Agent is thinking about the next step...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, PropType } from 'vue';
import type { AgentTask } from '../types';
import { Loader2, CheckCircle2, XCircle, CircleDashed, ChevronRight } from 'lucide-vue-next';
import JsonViewer from './JsonViewer.vue';
import { parseMarkdown } from '../utils/markdownParser';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const expandedSteps = ref(new Set<number>());

const toggleStep = (index: number) => {
    if (expandedSteps.value.has(index)) {
        expandedSteps.value.delete(index);
    } else {
        expandedSteps.value.add(index);
    }
};

const renderedMarkdown = (text: string) => {
  return parseMarkdown(text);
};
</script>