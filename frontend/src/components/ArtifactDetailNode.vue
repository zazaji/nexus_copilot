<!-- frontend/src/components/ArtifactDetailNode.vue -->
<template>
  <div class="flex flex-col text-sm">
    <div 
      @click="isExpanded = !isExpanded" 
      class="flex items-start space-x-3 cursor-pointer p-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700/50"
    >
      <div class="flex items-center space-x-3 flex-1 min-w-0">
        <ChevronRight v-if="hasChildren" class="w-4 h-4 text-gray-400 transition-transform flex-shrink-0 mt-1" :class="{ 'rotate-90': isExpanded }" />
        <div v-else class="w-4 flex-shrink-0"></div>
        <p class="font-semibold text-gray-800 dark:text-gray-200">{{ fullTitle }}</p>
      </div>
      <div v-if="nodeCharCount > 0" class="flex items-center space-x-1.5 text-xs font-mono text-gray-500 dark:text-gray-400 bg-gray-200 dark:bg-gray-600 px-2 py-0.5 rounded-full flex-shrink-0">
        <FileText class="w-3 h-3" />
        <span>{{ nodeCharCount }}</span>
      </div>
    </div>
    
    <div v-if="isExpanded" class="mt-2 ml-4 pl-5 border-l border-gray-300 dark:border-gray-600 space-y-4">
      <!-- Content for Leaf Node -->
      <div v-if="!hasChildren">
        <div v-if="nodeContent">
          <div>
            <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">Current Version</h4>
            <div class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedContent"></div>
          </div>
          <div v-if="nodeContent.history && nodeContent.history.length > 0" class="mt-4">
            <h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase pb-2 mb-2 border-b border-gray-200 dark:border-gray-700">Refinement History ({{ nodeContent.history.length }})</h4>
            <div class="space-y-3 max-h-60 overflow-y-auto">
              <div v-for="(item, index) in nodeContent.history.slice().reverse()" :key="index" class="p-2 bg-gray-100 dark:bg-gray-700/50 rounded-md">
                <p class="text-xs italic text-gray-500 dark:text-gray-400"><strong>Prompt:</strong> "{{ item.prompt }}"</p>
                <div class="mt-1 prose prose-sm dark:prose-invert max-w-none border-t border-gray-200 dark:border-gray-600 pt-1" v-html="parseMarkdown(item.content)"></div>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="text-xs italic text-gray-400">Content not available for this section.</div>
      </div>
      <!-- Child Nodes -->
      <ArtifactDetailNode
        v-else
        v-for="(childNode, index) in node.steps"
        :key="index"
        :node="childNode"
        :prefix="`${prefix}.${index + 1}`"
        :content-map="contentMap"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, PropType } from 'vue';
import type { ContentNode, researchOutlineNode } from '../types';
import { ChevronRight, FileText } from 'lucide-vue-next';
import { parseMarkdown } from '../utils/markdownParser';
import { cleanRedundantTitle } from '../utils/contentProcessor';

defineOptions({
  name: 'ArtifactDetailNode'
});

const props = defineProps({
  node: {
    type: Object as PropType<researchOutlineNode>,
    required: true
  },
  prefix: {
    type: String,
    required: true
  },
  contentMap: {
    type: Object as PropType<Record<string, ContentNode | null>>,
    required: true
  }
});

const hasChildren = computed(() => props.node.steps && props.node.steps.length > 0);
const isExpanded = ref(true);

const fullTitle = computed(() => `${props.prefix} ${props.node.sub_goal}`);

const nodeContent = computed(() => props.contentMap[props.node.id] || null);

const countCharacters = (text: string | null) => {
  if (!text) return 0;
  return text.trim().length;
};

const nodeCharCount = computed(() => {
  if (hasChildren.value) {
    // For parent nodes, sum up children's char counts
    const countChildren = (nodes: researchOutlineNode[]): number => {
      return nodes.reduce((sum, child) => {
        if (child.steps && child.steps.length > 0) {
          return sum + countChildren(child.steps);
        }
        return sum + countCharacters(props.contentMap[child.id]?.current || null);
      }, 0);
    };
    return countChildren(props.node.steps);
  }
  // For leaf nodes, count their own content
  return countCharacters(nodeContent.value?.current || null);
});

const renderedContent = computed(() => {
  if (!nodeContent.value?.current) return '';
  const cleaned = cleanRedundantTitle(fullTitle.value, nodeContent.value.current);
  return parseMarkdown(cleaned);
});
</script>