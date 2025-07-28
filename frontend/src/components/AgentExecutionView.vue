<!-- frontend/src/components/AgentExecutionView.vue -->
<template>
  <div class="bg-gray-100 dark:bg-gray-800/50 p-4 rounded-lg border border-blue-500/30">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center space-x-3">
        <div class="flex-shrink-0">
          <Loader2 v-if="task.status === 'running' || task.status === 'planning'" class="w-6 h-6 text-blue-500 animate-spin" />
          <CheckCircle2 v-else-if="task.status === 'completed'" class="w-6 h-6 text-green-500" />
          <XCircle v-else-if="task.status === 'failed'" class="w-6 h-6 text-red-500" />
          <PauseCircle v-else-if="task.status === 'awaiting_user_input'" class="w-6 h-6 text-yellow-500" />
        </div>
        <div>
          <p class="font-semibold text-gray-800 dark:text-gray-200">Agent Task ({{ displayMode }})</p>
          <p class="text-sm text-gray-600 dark:text-gray-400">Goal: {{ task.userGoal }}</p>
        </div>
      </div>
      <div class="flex items-center space-x-1 text-sm">
        <n-tooltip v-if="task.status === 'failed'" trigger="hover">
          <template #trigger>
            <button @click="resumeTask" class="p-2 rounded-md text-orange-500 hover:bg-gray-200 dark:hover:bg-gray-700">
              <RefreshCw class="w-4 h-4" />
            </button>
          </template>
          Resume Task
        </n-tooltip>
        <n-tooltip v-if="task.logFileUrl" trigger="hover">
          <template #trigger>
            <button @click="previewStore.showPreview(task.logFileUrl)" class="p-2 rounded-md text-blue-500 hover:bg-gray-200 dark:hover:bg-gray-700">
              <FileText class="w-4 h-4" />
            </button>
          </template>
          Execution Log
        </n-tooltip>
        <n-tooltip v-if="task.reportFileUrl" trigger="hover">
          <template #trigger>
            <button @click="previewStore.showPreview(task.reportFileUrl)" class="p-2 rounded-md text-green-500 hover:bg-gray-200 dark:hover:bg-gray-700">
              <FileCheck2 class="w-4 h-4" />
            </button>
          </template>
          Final Report
        </n-tooltip>
        <n-tooltip v-if="showArtifactPanel" trigger="hover">
          <template #trigger>
            <button @click="uiStore.toggleArtifactPanel(task.id)" class="p-2 rounded-md text-purple-500 hover:bg-gray-200 dark:hover:bg-gray-700">
              <component :is="isPreviewingThisTask ? PanelRightClose : PanelLeftOpen" class="w-4 h-4" />
            </button>
          </template>
          {{ isPreviewingThisTask ? 'Hide Preview' : 'Show Preview' }}
        </n-tooltip>
      </div>
    </div>

    <component 
      :is="executionComponent"
      :task="task"
      @refine-node="handleRefineNode"
    />

    <RefineModal 
      :is-visible="isRefineModalVisible"
      :node-title="refiningNodeTitle"
      @close="isRefineModalVisible = false"
      @submit="submitRefinement"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref } from 'vue';
import type { AgentTask } from '../types';
import { Loader2, CheckCircle2, XCircle, FileText, FileCheck2, RefreshCw, PanelLeftOpen, PanelRightClose, PauseCircle } from 'lucide-vue-next';
import { NTooltip } from 'naive-ui';
import { useFilePreviewStore } from '../stores/filePreview';
import { useAgentStore } from '../stores/agent';
import { useUiStore } from '../stores/ui';
import RefineModal from './RefineModal.vue';
import PlanExecutionView from './PlanExecutionView.vue';
import ExploreExecutionView from './ExploreExecutionView.vue';
import WriteExecutionView from './WriteExecutionView.vue';
import ResearchExecutionView from './ResearchExecutionView.vue';
import DebateExecutionView from './DebateExecutionView.vue';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const previewStore = useFilePreviewStore();
const agentStore = useAgentStore();
const uiStore = useUiStore();

const executionComponent = computed(() => {
  switch (props.task.mode) {
    case 'plan': return PlanExecutionView;
    case 'explore': return ExploreExecutionView;
    case 'write': return WriteExecutionView;
    case 'research': return ResearchExecutionView;
    case 'debate': return DebateExecutionView;
    default: return null;
  }
});

// --- Refine Modal State ---
const isRefineModalVisible = ref(false);
const refiningNodeId = ref('');
const refiningNodeTitle = ref('');

const handleRefineNode = (payload: { nodeId: string, nodeTitle: string }) => {
  refiningNodeId.value = payload.nodeId;
  refiningNodeTitle.value = payload.nodeTitle;
  isRefineModalVisible.value = true;
};

const submitRefinement = (payload: { prompt: string, model: string, isManual: boolean }) => {
  agentStore.refineSection(props.task.id, refiningNodeId.value, payload.prompt, payload.model, payload.isManual);
};

// --- Common Logic ---
const displayMode = computed(() => {
  const mode = props.task.mode;
  if (!mode) return 'Agent';
  return mode.charAt(0).toUpperCase() + mode.slice(1);
});

const showArtifactPanel = computed(() => {
  const mode = props.task.mode;
  return (mode === 'write' || mode === 'debate' || mode === 'plan' || mode === 'explore' || mode === 'research');
});

const isPreviewingThisTask = computed(() => {
  return uiStore.isArtifactPanelVisible && uiStore.previewedTaskId === props.task.id;
});

const resumeTask = () => {
  agentStore.restartFailedTask(props.task.id);
};
</script>