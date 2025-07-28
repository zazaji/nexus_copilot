<!-- frontend/src/components/PlanStepNode.vue -->
<template>
  <div class="flex flex-col text-sm">
    <div @click="toggleExpansion" class="flex items-center space-x-3 cursor-pointer p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700/50">
      <div class="flex-shrink-0 w-4 h-4 flex items-center justify-center">
        <ChevronRight v-if="hasChildren" class="w-4 h-4 text-gray-400 transition-transform" :class="{ 'rotate-90': isExpanded }" />
      </div>
      <div class="flex-shrink-0">
        <Loader2 v-if="status === 'running'" class="w-4 h-4 text-blue-400 animate-spin" />
        <CheckCircle2 v-else-if="status === 'completed'" class="w-4 h-4 text-green-400" />
        <XCircle v-else-if="status === 'failed'" class="w-4 h-4 text-red-400" />
        <CircleDashed v-else class="w-4 h-4 text-gray-500" />
      </div>
      <p class="font-medium text-gray-800 dark:text-gray-200 flex-1">{{ step.sub_goal }}</p>
    </div>
    
    <!-- Child Steps -->
    <div v-if="isExpanded && hasChildren" class="mt-1 ml-4 pl-4 border-l border-gray-300 dark:border-gray-600 space-y-1">
      <PlanStepNode
        v-for="(childStep, index) in step.steps"
        :key="index"
        :step="childStep"
        :get-step-status="getStepStatus"
        :get-step-details="getStepDetails"
        :rendered-markdown="renderedMarkdown"
      />
    </div>
    
    <!-- Step Details for Leaf Nodes -->
    <div v-if="isExpanded && !hasChildren && details" class="mt-2 ml-11 pl-4 border-l border-gray-300 dark:border-gray-600 space-y-2 pb-2">
      <div v-if="details.thought" class="bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Thought</p>
        <p class="text-gray-600 dark:text-gray-300 text-xs mt-1 italic">"{{ details.thought }}"</p>
      </div>
      <div v-if="details.action && details.status !== 'pending'" class="bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Action</p>
        <p class="font-mono text-xs text-blue-600 dark:text-blue-400 mt-1">{{ details.action }}</p>
      </div>
      <div v-if="details.observation" class="border-l-4 border-gray-300 dark:border-gray-600 pl-3 py-1">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Observation</p>
        <div v-if="parsedObservation" class="bg-gray-200/50 dark:bg-gray-700/50 p-2 rounded-md mt-1">
          <p class="text-xs font-semibold text-gray-500 dark:text-gray-400">Inner Thought</p>
          <p class="text-gray-600 dark:text-gray-300 text-xs mt-1 italic">"{{ parsedObservation.thought }}"</p>
        </div>
        <pre v-else class="whitespace-pre-wrap font-mono text-xs text-gray-600 dark:text-gray-300 mt-1">{{ details.observation }}</pre>
      </div>
      <div v-if="details.result" class="border-l-4 border-green-400 dark:border-green-600 pl-3 py-1">
        <p class="text-xs font-semibold text-green-600 dark:text-green-400">Result</p>
        <div class="prose prose-sm dark:prose-invert max-w-none mt-1" v-html="renderedMarkdown(details.result || '')"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, PropType } from 'vue';
import type { AgentTaskStep } from '../types';
import { Loader2, CheckCircle2, XCircle, CircleDashed, ChevronRight } from 'lucide-vue-next';

defineOptions({
  name: 'PlanStepNode'
});

const props = defineProps({
  step: {
    type: Object as PropType<{ sub_goal: string, steps?: any[], step_index?: number }>,
    required: true
  },
  getStepStatus: {
    type: Function as PropType<(index: number) => AgentTaskStep['status']>,
    required: true
  },
  getStepDetails: {
    type: Function as PropType<(index: number) => AgentTaskStep | undefined>,
    required: true
  },
  renderedMarkdown: {
    type: Function as PropType<(text: string) => string>,
    required: true
  }
});

const isExpanded = ref(false);

const hasChildren = computed(() => props.step.steps && props.step.steps.length > 0);

const status = computed(() => {
  if (props.step.step_index !== undefined) {
    return props.getStepStatus(props.step.step_index - 1);
  }
  return 'pending'; // Default for parent nodes
});

const details = computed(() => {
  if (props.step.step_index !== undefined) {
    return props.getStepDetails(props.step.step_index - 1);
  }
  return undefined;
});

const parsedObservation = computed(() => {
  const observation = details.value?.observation;
  if (!observation) return null;

  let jsonString = observation.trim();
  const match = /```json\s*([\s\S]*?)\s*```/.exec(jsonString);
  if (match && match[1]) {
    jsonString = match[1];
  }

  try {
    const parsed = JSON.parse(jsonString);
    if (parsed && typeof parsed === 'object' && 'thought' in parsed) {
      return parsed;
    }
    return null;
  } catch (e) {
    return null;
  }
});

const toggleExpansion = () => {
  isExpanded.value = !isExpanded.value;
};
</script>