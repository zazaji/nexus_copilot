<!-- frontend/src/components/ResearchExecutionView.vue -->
<template>
  <div class="pl-9">
    <div v-if="!outline || outline.length === 0" class="text-sm text-gray-500 dark:text-gray-400 flex items-center space-x-2">
      <Loader2 class="w-4 h-4 animate-spin" />
      <span>AI正在精心规划大纲...</span>
    </div>
    <div v-else class="grid grid-cols-12 gap-4 h-full">
      <div class="col-span-5 border-r border-gray-200 dark:border-gray-700 pr-4 overflow-y-auto">
        <h3 class="font-semibold text-gray-700 dark:text-gray-300 mb-2">写作大纲</h3>
        <div class="space-y-1">
          <ResearchOutlineNode 
            v-for="node in outline"
            :key="node.id"
            :node="node"
            :active-node-id="activeNodeId"
            :expanded-nodes="expandedNodes"
            @select-node="handleSelectNode"
            @toggle-expand="handleToggleExpand"
            @generate-node="handleGenerateNode"
            @refine-node="$emit('refine-node', $event)"
          />
        </div>
      </div>
      <div class="col-span-7 overflow-y-auto">
        <div v-if="activeNodeId">
          <div v-if="activeContentNode" class="space-y-4">
            <div class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedContent"></div>
            <div v-if="activeContentNode.history && activeContentNode.history.length > 0">
              <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">Refinement History ({{ activeContentNode.history.length }})</h4>
              <div class="space-y-3 max-h-60 overflow-y-auto">
                <div v-for="(item, index) in activeContentNode.history.slice().reverse()" :key="index" class="p-2 bg-gray-100 dark:bg-gray-700/50 rounded-md">
                  <p class="text-xs italic text-gray-500 dark:text-gray-400"><strong>Prompt:</strong> "{{ item.prompt }}"</p>
                  <div class="mt-1 prose prose-xs dark:prose-invert max-w-none border-t border-gray-200 dark:border-gray-600 pt-1" v-html="parseMarkdown(item.content)"></div>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-full text-gray-400">
            <div class="text-center">
              <p>选择一个章节开始创作</p>
              <button @click="handleGenerateNode(activeNodeId)" class="mt-2 px-3 py-1 text-xs bg-blue-500 text-white rounded-md hover:bg-blue-600">
                生成本节内容
              </button>
            </div>
          </div>
        </div>
        <div v-else class="flex items-center justify-center h-full text-gray-400">
          <p>请从左侧大纲中选择一个章节</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, PropType, watch } from 'vue';
import type { AgentTask, researchOutlineNode as OutlineNode, ContentNode } from '../types';
import { Loader2 } from 'lucide-vue-next';
import ResearchOutlineNode from './ResearchOutlineNode.vue';
import { parseMarkdown } from '../utils/markdownParser';
import { cleanRedundantTitle } from '../utils/contentProcessor';
import { useAgentStore } from '../stores/agent';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

defineEmits(['refine-node']);

const agentStore = useAgentStore();
const activeNodeId = ref<string | null>(null);
const expandedNodes = ref(new Set<string>());

const outline = computed<OutlineNode[]>(() => props.task.plan as OutlineNode[] || []);
const contentMap = computed<Record<string, ContentNode>>(() => props.task.researchContent || {});

const findNodeById = (nodes: OutlineNode[], id: string): OutlineNode | null => {
  for (const node of nodes) {
    if (node.id === id) return node;
    if (node.steps) {
      const found = findNodeById(node.steps, id);
      if (found) return found;
    }
  }
  return null;
};

const activeContentNode = computed(() => {
  if (!activeNodeId.value) return null;
  return contentMap.value[activeNodeId.value] || null;
});

const renderedContent = computed(() => {
  if (!activeContentNode.value) return '';
  const activeNode = findNodeById(outline.value, activeNodeId.value!);
  const outlineTitle = activeNode ? `${activeNode.id} ${activeNode.sub_goal}` : undefined;
  const cleanedText = cleanRedundantTitle(outlineTitle, activeContentNode.value.current);
  return parseMarkdown(cleanedText);
});

const handleSelectNode = (nodeId: string) => {
  activeNodeId.value = nodeId;
  const parts = nodeId.split('.');
  for (let i = 1; i < parts.length; i++) {
    expandedNodes.value.add(parts.slice(0, i).join('.'));
  }
};

const handleToggleExpand = (nodeId: string) => {
  if (expandedNodes.value.has(nodeId)) {
    expandedNodes.value.delete(nodeId);
  } else {
    expandedNodes.value.add(nodeId);
  }
};

const handleGenerateNode = (nodeId: string) => {
  agentStore.generateResearchNodeContent(props.task.id, nodeId);
};

watch(() => props.task.plan, (newPlan) => {
  if (props.task.mode === 'research' && newPlan && !activeNodeId.value) {
    const findFirstLeaf = (nodes: OutlineNode[]): OutlineNode | null => {
      for (const node of nodes) {
        if (!node.steps || node.steps.length === 0) return node;
        const leaf = findFirstLeaf(node.steps);
        if (leaf) return leaf;
      }
      return null;
    };
    const firstLeaf = findFirstLeaf(newPlan as OutlineNode[]);
    if (firstLeaf) handleSelectNode(firstLeaf.id);
  }
}, { immediate: true });
</script>