<!-- frontend/src/components/ArtifactPreviewPanel.vue -->
<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700">
    <header class="p-2 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 flex items-center justify-between group">
      <n-tabs v-model:value="activeTab" type="line" animated class="flex-1">
        <n-tab v-if="showDetailsTab" name="details">
          <div class="flex items-center space-x-2">
            <span>Details</span>
            <span v-if="detailsCharCount > 0" class="text-xs font-mono text-gray-400">({{ detailsCharCount }})</span>
          </div>
        </n-tab>
        <n-tab v-if="showReportTab" name="report">
           <div class="flex items-center space-x-2">
            <span>Report</span>
            <span v-if="reportCharCount > 0" class="text-xs font-mono text-gray-400">({{ reportCharCount }})</span>
          </div>
        </n-tab>
      </n-tabs>
      <div class="flex items-center space-x-1 pl-2">
        <n-tooltip trigger="hover">
          <template #trigger>
            <button @click="copyContent" class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700">
              <Copy class="w-4 h-4" />
            </button>
          </template>
          Copy Markdown
        </n-tooltip>
        <n-tooltip trigger="hover">
          <template #trigger>
            <button @click="uiStore.hideArtifactPanel()" class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700">
              <PanelRightClose class="w-4 h-4" />
            </button>
          </template>
          Hide Preview
        </n-tooltip>
      </div>
    </header>
    <main class="flex-1 flex flex-col overflow-hidden">
      <div class="flex-1 overflow-y-auto p-6">
        <div v-if="isLoading" class="flex items-center justify-center h-full text-gray-500">
          <Loader2 class="w-6 h-6 animate-spin" />
          <span class="ml-2">正在生成最终报告...</span>
        </div>
        <div v-else-if="!task" class="text-center text-gray-400">
          <p>No task selected for preview.</p>
        </div>
        <template v-else>
          <div v-if="activeTab === 'report' && showReportTab" class="prose dark:prose-invert max-w-none" v-html="renderedReport"></div>
          <div v-if="activeTab === 'details' && showDetailsTab">
            <div v-if="task.mode === 'research'" class="space-y-2">
              <ArtifactDetailNode 
                v-for="(node, index) in task.plan"
                :key="index"
                :node="node"
                :prefix="`${index + 1}`"
                :content-map="task.researchContent || {}"
              />
            </div>
            <div v-else-if="task.mode === 'write'" class="space-y-1">
               <WriteModeStep 
                v-for="(step, index) in task.plan" 
                :key="index"
                :step-node="step"
                :task-steps="task.steps"
                :task-content="task.researchContent"
                :prefix="`${index + 1}`"
              />
            </div>
            <DebateArtifactDetail v-else-if="task.mode === 'debate'" :task="task" />
            <ExploreArtifactDetail v-else-if="task.mode === 'explore' || task.mode === 'plan'" :task="task" />
          </div>
        </template>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref, watch } from 'vue';
import type { AgentTask, researchOutlineNode } from '../types';
import { parseMarkdown } from '../utils/markdownParser';
import { Loader2, PanelRightClose, Copy } from 'lucide-vue-next';
import { useUiStore } from '../stores/ui';
import { useToasts } from '../composables/useToasts';
import { NTooltip, NTabs, NTab } from 'naive-ui';
import ArtifactDetailNode from './ArtifactDetailNode.vue';
import DebateArtifactDetail from './DebateArtifactDetail.vue';
import ExploreArtifactDetail from './ExploreArtifactDetail.vue';
import WriteModeStep from './WriteModeStep.vue';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask | null>,
    default: null
  }
});

const uiStore = useUiStore();
const { success, error } = useToasts();
const activeTab = ref('details');

const showDetailsTab = computed(() => {
  if (!props.task) return false;
  const mode = props.task.mode;
  if ((mode === 'research' || mode === 'write') && props.task.plan && (props.task.plan as any[]).length > 0) {
    return true;
  }
  if (mode === 'debate' && props.task.plan && (props.task.plan as any).rounds) {
    return true;
  }
  if ((mode === 'explore' || mode === 'plan') && props.task.steps && props.task.steps.length > 0) {
    return true;
  }
  return false;
});

const showReportTab = computed(() => {
  return props.task && !!props.task.finalReport;
});

watch(() => props.task, (newTask) => {
  if (!newTask) return;
  
  if (newTask.status === 'completed' && showReportTab.value) {
    activeTab.value = 'report';
  } else if (showDetailsTab.value) {
    activeTab.value = 'details';
  } else if (showReportTab.value) {
    activeTab.value = 'report';
  }
}, { immediate: true, deep: true });

const countCharacters = (text: string | null | undefined) => {
  if (!text) return 0;
  return text.trim().length;
};

const isLoading = computed(() => {
  if (!props.task) return false;
  return (props.task.status === 'running' || props.task.status === 'planning') && !props.task.finalReport;
});

const reportContent = computed(() => {
  return props.task?.finalReport || '';
});

const detailsContent = computed(() => {
  if (!props.task || !showDetailsTab.value) return '';
  
  if (props.task.mode === 'write' || props.task.mode === 'research') {
    const contentMap = props.task.researchContent || {};
    let fullText = '';
    
    const buildText = (nodes: researchOutlineNode[], prefix = '') => {
      nodes.forEach((node, index) => {
        const currentPrefix = prefix ? `${prefix}.${index + 1}` : `${index + 1}`;
        fullText += `# ${currentPrefix} ${node.sub_goal}\n\n`;
        if (node.steps && node.steps.length > 0) {
          buildText(node.steps, currentPrefix);
        } else {
          const content = contentMap[node.id]?.current || '';
          fullText += `${content}\n\n`;
        }
      });
    };
    
    if (props.task.plan) {
      buildText(props.task.plan as researchOutlineNode[]);
    }
    return fullText;
  }
  
  return props.task.finalReport || 'Details view content is not available for copying.';
});

const reportCharCount = computed(() => countCharacters(reportContent.value));
const detailsCharCount = computed(() => countCharacters(detailsContent.value));

const renderedReport = computed(() => {
  return parseMarkdown(reportContent.value);
});

const copyContent = async () => {
  let contentToCopy = '';
  if (activeTab.value === 'report' || !showDetailsTab.value) {
    contentToCopy = reportContent.value;
  } else if (activeTab.value === 'details') {
    contentToCopy = detailsContent.value;
  }

  if (!contentToCopy) {
    error('No content to copy.');
    return;
  }
  try {
    await navigator.clipboard.writeText(contentToCopy);
    success('Markdown content copied to clipboard!');
  } catch (err) {
    console.error('Failed to copy content:', err);
    error('Failed to copy content.');
  }
};
</script>