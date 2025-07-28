<!-- frontend/src/components/ExploreArtifactDetail.vue -->
<template>
  <div class="space-y-4">
    <div v-for="(step, index) in task.steps" :key="step.id" class="p-4 bg-gray-50 dark:bg-gray-700/30 rounded-lg">
      <h3 class="font-bold text-lg mb-3 text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-600 pb-2">
        Step {{ step.step_index }}: {{ step.action }}
      </h3>
      <div class="space-y-3 text-sm">
        <div v-if="step.thought">
          <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Thought</p>
          <p class="text-gray-600 dark:text-gray-300 mt-1 italic">"{{ step.thought }}"</p>
        </div>
        <div v-if="step.action && step.status !== 'pending'">
          <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Action</p>
          <p class="font-mono text-xs text-blue-600 dark:text-blue-400 mt-1">{{ step.action }}</p>
        </div>
        <div v-if="step.observation">
          <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Observation</p>
          <JsonViewer :source="step.observation" />
        </div>
        <div v-if="step.result">
          <p class="text-xs font-semibold text-green-600 dark:text-green-400">Result</p>
          <div class="prose prose-sm dark:prose-invert max-w-none mt-1" v-html="renderedMarkdown(step.result)"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PropType } from 'vue';
import type { AgentTask } from '../types';
import { parseMarkdown } from '../utils/markdownParser';
import JsonViewer from './JsonViewer.vue';

defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const renderedMarkdown = (text: string) => {
  return parseMarkdown(text || '');
};
</script>