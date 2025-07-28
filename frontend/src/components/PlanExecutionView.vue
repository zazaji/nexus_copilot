<!-- frontend/src/components/PlanExecutionView.vue -->
<template>
  <div>
    <div v-if="!indexedPlanSteps || indexedPlanSteps.length === 0" class="pl-9 text-sm text-gray-500 dark:text-gray-400 flex items-center space-x-2">
        <Loader2 class="w-4 h-4 animate-spin" />
        <span>Agent is planning the steps...</span>
    </div>
    <div v-else class="space-y-1 pl-9">
      <PlanStepNode
        v-for="(planStep, index) in indexedPlanSteps"
        :key="index"
        :step="planStep"
        :get-step-status="getStepStatus"
        :get-step-details="getStepDetails"
        :rendered-markdown="renderedMarkdown"
      />
      <div v-if="completedStepsCount >= totalLeafSteps && totalLeafSteps > 0" class="flex items-center space-x-3 text-sm pt-2">
        <div class="flex-shrink-0">
          <Loader2 v-if="task.status === 'running'" class="w-4 h-4 text-blue-400 animate-spin" />
          <CheckCircle2 v-else-if="task.status === 'completed'" class="w-4 h-4 text-green-400" />
          <XCircle v-else-if="task.status === 'failed'" class="w-4 h-4 text-red-400" />
        </div>
        <p class="font-medium text-gray-800 dark:text-gray-200">Finalizing Report</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { AgentTask, AgentTaskStep } from '../types';
import { Loader2, CheckCircle2, XCircle } from 'lucide-vue-next';
import PlanStepNode from './PlanStepNode.vue';
import { parseMarkdown } from '../utils/markdownParser';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const renderedMarkdown = (text: string) => {
  return parseMarkdown(text);
};

const indexedPlanSteps = computed(() => {
  let counter = 1;
  const assignIndex = (nodes: any[]) => {
    nodes.forEach(node => {
      if (!node.steps || node.steps.length === 0) {
        node.step_index = counter++;
      } else {
        assignIndex(node.steps);
      }
    });
  };
  const planCopy = JSON.parse(JSON.stringify(props.task.plan || []));
  assignIndex(planCopy);
  return planCopy;
});

const totalLeafSteps = computed(() => {
  let count = 0;
  const counter = (nodes: any[]) => {
    nodes.forEach(node => {
      if (!node.steps || node.steps.length === 0) {
        count++;
      } else {
        counter(node.steps);
      }
    });
  };
  counter(props.task.plan || []);
  return count;
});

const completedStepsCount = computed(() => {
  return props.task.steps.filter(s => s.status === 'completed').length;
});

const getStepStatus = (index: number): AgentTaskStep['status'] => {
  const step = props.task.steps.find(s => s.step_index === index + 1);
  return step ? step.status : 'pending';
};

const getStepDetails = (index: number): AgentTaskStep | undefined => {
  return props.task.steps.find(s => s.step_index === index + 1);
};
</script>