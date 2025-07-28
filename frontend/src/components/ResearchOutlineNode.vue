<!-- frontend/src/components/ResearchOutlineNode.vue -->
<template>
  <div>
    <div 
      @click="$emit('selectNode', node.id)"
      class="flex items-center space-x-2 p-1.5 rounded-md cursor-pointer group"
      :class="isActive ? 'bg-blue-100 dark:bg-blue-900/50' : 'hover:bg-gray-200 dark:hover:bg-gray-700/50'"
    >
      <button @click.stop="$emit('toggleExpand', node.id)" v-if="!isLeaf" class="p-0.5 rounded-sm hover:bg-gray-300 dark:hover:bg-gray-600">
        <ChevronRight class="w-3 h-3 text-gray-500 transition-transform" :class="{ 'rotate-90': isExpanded }" />
      </button>
      <div v-else class="w-4"></div> <!-- Placeholder for alignment -->

      <component :is="nodeIcon" class="w-4 h-4 flex-shrink-0" :class="nodeIconClass" />
      <span class="text-sm flex-1" :class="isActive ? 'font-semibold text-blue-800 dark:text-blue-300' : 'text-gray-700 dark:text-gray-300'">
        {{ node.id }} {{ node.sub_goal }}
      </span>
      <button 
        v-if="isLeaf && node.status === 'completed'" 
        @click.stop="$emit('refineNode', { nodeId: node.id, nodeTitle: `${node.id} ${node.sub_goal}` })"
        :disabled="isRefining"
        class="p-1 rounded-md text-gray-400 hover:bg-blue-200 dark:hover:bg-blue-800 opacity-0 group-hover:opacity-100 transition-opacity disabled:opacity-50"
        title="Refine this section"
      >
        <Loader2 v-if="isRefining" class="w-4 h-4 text-blue-500 animate-spin" />
        <Wand v-else class="w-4 h-4 text-blue-500" />
      </button>
      <button 
        v-if="isLeaf && node.status === 'pending'" 
        @click.stop="$emit('generateNode', node.id)"
        class="p-1 rounded-md text-gray-400 hover:bg-blue-200 dark:hover:bg-blue-800 opacity-0 group-hover:opacity-100 transition-opacity"
        title="生成本节内容"
      >
        <Wand class="w-4 h-4 text-blue-500" />
      </button>
    </div>
    <div v-if="isExpanded && !isLeaf" class="ml-4 pl-3 border-l border-gray-200 dark:border-gray-700 mt-1 space-y-1">
      <ResearchOutlineNode
        v-for="child in node.steps"
        :key="child.id"
        :node="child"
        :active-node-id="activeNodeId"
        :expanded-nodes="expandedNodes"
        @select-node="$emit('selectNode', $event)"
        @toggle-expand="$emit('toggleExpand', $event)"
        @generate-node="$emit('generateNode', $event)"
        @refine-node="$emit('refineNode', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { researchOutlineNode as OutlineNode } from '../types';
import { useAgentStore } from '../stores/agent';
import { FileText, CheckCircle2, Loader2, Wand, ChevronRight } from 'lucide-vue-next';

defineOptions({
  name: 'ResearchOutlineNode'
});

const props = defineProps({
  node: {
    type: Object as PropType<OutlineNode>,
    required: true
  },
  activeNodeId: {
    type: String as PropType<string | null>,
    default: null
  },
  expandedNodes: {
    type: Set as PropType<Set<string>>,
    required: true,
  }
});

defineEmits(['selectNode', 'generateNode', 'toggleExpand', 'refineNode']);

const agentStore = useAgentStore();

const isLeaf = computed(() => !props.node.steps || props.node.steps.length === 0);
const isActive = computed(() => props.node.id === props.activeNodeId);
const isExpanded = computed(() => props.expandedNodes.has(props.node.id));
const isRefining = computed(() => agentStore.refiningNodeIds.has(props.node.id));

const nodeIcon = computed(() => {
  if (props.node.status === 'writing') return Loader2;
  if (props.node.status === 'completed') return CheckCircle2;
  return FileText;
});

const nodeIconClass = computed(() => {
  if (props.node.status === 'writing') return 'text-blue-500 animate-spin';
  if (props.node.status === 'completed') return 'text-green-500';
  return 'text-gray-400';
});
</script>