<!-- frontend/src/components/WriteModeStep.vue -->
<template>
  <div class="flex flex-col text-sm">
    <!-- Special rendering for top-level leaf nodes -->
    <template v-if="isTopLevelLeaf">
      <div class="flex items-center space-x-3 p-1 rounded-md group">
        <div class="flex-shrink-0">
          <Loader2 v-if="stepStatus === 'running' || isRefining" class="w-4 h-4 text-blue-400 animate-spin" />
          <CheckCircle2 v-else-if="stepStatus === 'completed'" class="w-4 h-4 text-green-400" />
          <XCircle v-else-if="stepStatus === 'failed'" class="w-4 h-4 text-red-400" />
          <CircleDashed v-else class="w-4 h-4 text-gray-500" />
        </div>
        <p class="font-medium text-gray-800 dark:text-gray-200 flex-1">{{ fullTitle }}</p>
        <div v-if="stepNode.word_count" class="text-xs font-mono text-gray-500 dark:text-gray-400">
          ({{ recursiveCharCount }}/{{ stepNode.word_count }})
        </div>
        <!-- Important ：直接使用 stepNode.id 和 stepNode.sub_goal 来构造标题。基于 stepNode.id 来判断层级.解决一级大纲如果没有子节点的问题。这段注释很重要，不要删除。-->
        <button 
          v-if="stepStatus === 'completed'"
          @click.stop="$emit('refine', { nodeId: stepNode.id, nodeTitle: fullTitle })"
          :disabled="isRefining"
          class="p-1 rounded-md text-gray-400 hover:bg-blue-200 dark:hover:bg-blue-800 opacity-0 group-hover:opacity-100 transition-opacity disabled:opacity-50"
          title="Refine this section"
        >
          <Wand class="w-4 h-4 text-blue-500" />
        </button>
      </div>
      <div class="mt-2 ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 pb-2 pr-2 space-y-4">
        <div v-if="contentNode">
          <div>
            <div class="flex items-center space-x-2 pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">
              <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase">Current Version</h4>
              <n-tooltip v-if="contentNode.lastPrompt" trigger="hover">
                <template #trigger>
                  <Info class="w-3 h-3 text-gray-400 cursor-pointer" />
                </template>
                Prompt: "{{ contentNode.lastPrompt }}"
              </n-tooltip>
            </div>
            <div class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedContent"></div>
          </div>
          <div v-if="contentNode.history && contentNode.history.length > 0" class="mt-4">
            <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">Refinement History ({{ contentNode.history.length }})</h4>
            <div class="space-y-3 max-h-60 overflow-y-auto">
              <div v-for="(item, index) in contentNode.history.slice().reverse()" :key="index" class="p-2 bg-gray-100 dark:bg-gray-700/50 rounded-md">
                <p class="text-xs italic text-gray-500 dark:text-gray-400"><strong>Prompt:</strong> "{{ item.prompt }}"</p>
                <div class="mt-1 prose prose-sm dark:prose-invert max-w-none border-t border-gray-200 dark:border-gray-600 pt-1" v-html="parseMarkdown(item.content)"></div>
              </div>
            </div>
          </div>
        </div>
        <div v-else-if="stepStatus === 'running'" class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
          <Loader2 class="w-4 h-4 animate-spin" />
          <span>Generating content...</span>
        </div>
        <div v-else-if="stepStatus === 'failed'" class="text-sm text-red-500 dark:text-red-400">
          Content generation failed for this section.
        </div>
        <div v-else class="text-sm text-gray-400 dark:text-gray-500">
          Content for this section has not been generated yet.
        </div>
      </div>
    </template>

    <!-- Default rendering for nested or parent nodes -->
    <template v-else>
      <div 
        @click="isExpanded = !isExpanded" 
        class="flex items-center space-x-3 cursor-pointer p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700/50 group"
      >
        <div class="flex-shrink-0">
          <Loader2 v-if="stepStatus === 'running' || isRefining" class="w-4 h-4 text-blue-400 animate-spin" />
          <CheckCircle2 v-else-if="stepStatus === 'completed'" class="w-4 h-4 text-green-400" />
          <XCircle v-else-if="stepStatus === 'failed'" class="w-4 h-4 text-red-400" />
          <CircleDashed v-else class="w-4 h-4 text-gray-500" />
        </div>
        <p class="font-medium text-gray-800 dark:text-gray-200 flex-1">{{ fullTitle }}</p>
        <div v-if="stepNode.word_count" class="text-xs font-mono text-gray-500 dark:text-gray-400">
          ({{ recursiveCharCount }}/{{ stepNode.word_count }})
        </div>
        <button 
          v-if="stepStatus === 'completed' && isLeaf"
          @click.stop="$emit('refine', { nodeId: stepNode.id, nodeTitle: fullTitle })"
          :disabled="isRefining"
          class="p-1 rounded-md text-gray-400 hover:bg-blue-200 dark:hover:bg-blue-800 opacity-0 group-hover:opacity-100 transition-opacity disabled:opacity-50"
          title="Refine this section"
        >
          <Wand class="w-4 h-4 text-blue-500" />
        </button>
        <ChevronRight class="w-4 h-4 text-gray-400 transition-transform" :class="{ 'rotate-90': isExpanded }" />
      </div>
      
      <!-- Child Steps (Recursive) -->
      <div v-if="isExpanded && !isLeaf" class="ml-4 border-l border-gray-200 dark:border-gray-700 pl-3">
        <WriteModeStep 
          v-for="(child, i) in stepNode.steps" 
          :key="i"
          :step-node="child"
          :task-steps="taskSteps"
          :task-content="taskContent"
          @refine="$emit('refine', $event)"
        />
      </div>

      <!-- Content Display for Expanded Leaf Nodes -->
      <div v-if="isExpanded && isLeaf" class="mt-2 ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 pb-2 pr-2 space-y-4">
        <div v-if="contentNode">
          <div>
            <div class="flex items-center space-x-2 pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">
              <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase">Current Version</h4>
              <n-tooltip v-if="contentNode.lastPrompt" trigger="hover">
                <template #trigger>
                  <Info class="w-3 h-3 text-gray-400 cursor-pointer" />
                </template>
                Prompt: "{{ contentNode.lastPrompt }}"
              </n-tooltip>
            </div>
            <div class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedContent"></div>
          </div>
          <div v-if="contentNode.history && contentNode.history.length > 0" class="mt-4">
            <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">Refinement History ({{ contentNode.history.length }})</h4>
            <div class="space-y-3 max-h-60 overflow-y-auto">
              <div v-for="(item, index) in contentNode.history.slice().reverse()" :key="index" class="p-2 bg-gray-100 dark:bg-gray-700/50 rounded-md">
                <p class="text-xs italic text-gray-500 dark:text-gray-400"><strong>Prompt:</strong> "{{ item.prompt }}"</p>
                <div class="mt-1 prose prose-sm dark:prose-invert max-w-none border-t border-gray-200 dark:border-gray-600 pt-1" v-html="parseMarkdown(item.content)"></div>
              </div>
            </div>
          </div>
        </div>
        <div v-else-if="stepStatus === 'running'" class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
          <Loader2 class="w-4 h-4 animate-spin" />
          <span>Generating content...</span>
        </div>
        <div v-else-if="stepStatus === 'failed'" class="text-sm text-red-500 dark:text-red-400">
          Content generation failed for this section.
        </div>
        <div v-else class="text-sm text-gray-400 dark:text-gray-500">
          Content for this section has not been generated yet.
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, PropType } from 'vue';
import type { AgentTaskStep, ContentNode, Refinement, researchOutlineNode } from '../types';
import { Loader2, CheckCircle2, XCircle, CircleDashed, ChevronRight, Wand, Info } from 'lucide-vue-next';
import { NTooltip } from 'naive-ui';
import { useAgentStore } from '../stores/agent';
import { parseMarkdown } from '../utils/markdownParser';
import { cleanRedundantTitle } from '../utils/contentProcessor';

defineOptions({
  name: 'WriteModeStep'
});

const props = defineProps({
  stepNode: {
    type: Object as PropType<researchOutlineNode>,
    required: true
  },
  taskSteps: {
    type: Array as PropType<AgentTaskStep[]>,
    required: true
  },
  taskContent: {
    type: Object as PropType<Record<string, ContentNode> | null | undefined>,
    default: () => ({})
  }
});

defineEmits(['refine']);

const agentStore = useAgentStore();
const isExpanded = ref(false);
//Important ：直接使用 stepNode.id 和 stepNode.sub_goal 来构造标题。基于 stepNode.id 来判断层级.解决一级大纲如果没有子节点的问题。这段注释很重要，不要删除。

const fullTitle = computed(() => `${props.stepNode.id} ${props.stepNode.sub_goal}`);
const isRefining = computed(() => agentStore.refiningNodeIds.has(props.stepNode.id));
const isLeaf = computed(() => !props.stepNode.steps || props.stepNode.steps.length === 0);
const isTopLevelLeaf = computed(() => !props.stepNode.id.includes('.') && isLeaf.value);

const countCharacters = (text: string | null | undefined) => {
  if (!text) return 0;
  return text.trim().length;
};

const getContentForNode = (nodeFullTitle: string): { current: string; history: Refinement[]; lastPrompt: string | null } | null => {
  const writeActionPrefix = `Phase 4: Write content for '${nodeFullTitle}'`;
  const refineActionPrefix = `Phase 5: Refine content for '${nodeFullTitle}'`;

  const relevantSteps = props.taskSteps
    .filter(s => 
      (s.action.startsWith(writeActionPrefix) || s.action.startsWith(refineActionPrefix)) 
      && s.status === 'completed' 
      && s.result
    )
    .sort((a, b) => a.step_index - b.step_index);

  if (relevantSteps.length === 0) return null;

  const fullHistory: Refinement[] = [];
  
  relevantSteps.forEach(step => {
    try {
      const parsedResult = JSON.parse(step.result!);
      const content = parsedResult.content;
      
      let prompt = "Initial generation";
      if (step.action.startsWith(refineActionPrefix)) {
        prompt = "Refinement";
      }

      fullHistory.push({ prompt, content, timestamp: Date.now() });
    } catch (e) {
      console.warn(`Could not parse result for step: ${step.action}`, e);
    }
  });

  if (fullHistory.length === 0) return null;

  const lastVersion = fullHistory[fullHistory.length - 1];
  const previousVersions = fullHistory.slice(0, -1);

  return {
    current: lastVersion.content,
    history: previousVersions,
    lastPrompt: lastVersion.prompt,
  };
};

const contentNode = computed(() => getContentForNode(fullTitle.value));

const recursiveCharCount = computed(() => {
  const countNodeChars = (node: researchOutlineNode): number => {
    if (node.steps && node.steps.length > 0) {
      return node.steps.reduce((sum, child) => sum + countNodeChars(child), 0);
    } else {
      const nodeFullTitle = `${node.id} ${node.sub_goal}`;
      const content = getContentForNode(nodeFullTitle)?.current;
      return countCharacters(content);
    }
  };
  return countNodeChars(props.stepNode);
});

const renderedContent = computed(() => {
  if (!contentNode.value?.current) return '';
  const cleaned = cleanRedundantTitle(fullTitle.value, contentNode.value.current);
  return parseMarkdown(cleaned);
});

const getRecursiveStatus = (node: researchOutlineNode): AgentTaskStep['status'] => {
  const nodeFullTitle = `${node.id} ${node.sub_goal}`;

  if (!node.steps || node.steps.length === 0) {
    const writeAction = `Phase 4: Write content for '${nodeFullTitle}'`;
    const refineAction = `Phase 5: Refine content for '${nodeFullTitle}'`;
    
    const hasCompletedContent = props.taskSteps.some(s => 
      (s.action.startsWith(writeAction) || s.action.startsWith(refineAction)) 
      && s.status === 'completed' 
      && s.result
    );

    if (hasCompletedContent) return 'completed';
    
    const isRunning = props.taskSteps.some(s => (s.action.startsWith(writeAction) || s.action.startsWith(refineAction)) && s.status === 'running');
    if (isRunning) return 'running';
    
    const hasFailed = props.taskSteps.some(s => (s.action.startsWith(writeAction) || s.action.startsWith(refineAction)) && s.status === 'failed');
    if (hasFailed) return 'failed';

    return 'pending';
  }

  const childStatuses = node.steps.map((child) => 
    getRecursiveStatus(child)
  );

  if (childStatuses.some(s => s === 'running')) return 'running';
  if (childStatuses.some(s => s === 'failed')) return 'failed';
  if (childStatuses.every(s => s === 'completed')) return 'completed';
  return 'pending';
};

const stepStatus = computed(() => getRecursiveStatus(props.stepNode));
</script>