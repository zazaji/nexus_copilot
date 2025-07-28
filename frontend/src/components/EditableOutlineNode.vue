<!-- frontend/src/components/EditableOutlineNode.vue -->
<template>
  <div class="space-y-2">
    <div class="flex items-center space-x-2 group">
      <GripVertical class="w-4 h-4 text-gray-400 cursor-move flex-shrink-0" />
      <input 
        type="text" 
        v-model="node.sub_goal"
        class="flex-1 bg-transparent border-b border-gray-300 dark:border-gray-600 focus:outline-none focus:border-blue-500 text-sm py-1"
      />
      <div class="flex items-center space-x-1 flex-shrink-0">
        <input
          v-if="isLeaf"
          type="number"
          v-model.number="node.word_count"
          class="w-20 text-right bg-transparent border-b border-gray-300 dark:border-gray-600 focus:outline-none focus:border-blue-500 text-xs py-1"
          placeholder="words"
        />
        <span v-else class="w-20 text-right text-xs text-gray-400 font-mono py-1">
          ({{ calculatedWordCount }})
        </span>
        <button @click="addChild" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity" title="Add Sub-section">
          <Plus class="w-4 h-4 text-green-500" />
        </button>
        <button @click="removeNode" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 opacity-0 group-hover:opacity-100 transition-opacity" title="Remove Section">
          <Trash2 class="w-4 h-4 text-red-500" />
        </button>
      </div>
    </div>
    <div v-if="node.steps && node.steps.length > 0" class="ml-6 pl-4 border-l border-gray-200 dark:border-gray-700 space-y-3">
      <EditableOutlineNode
        v-for="(child, index) in node.steps"
        :key="child.id || index"
        :node="child"
        @remove="removeChild(index)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { PropType, computed } from 'vue';
import { GripVertical, Plus, Trash2 } from 'lucide-vue-next';
import { v4 as uuidv4 } from 'uuid';

defineOptions({
  name: 'EditableOutlineNode'
});

const props = defineProps({
  node: {
    type: Object as PropType<{ sub_goal: string, steps?: any[], id?: string, word_count?: number }>,
    required: true
  }
});

const emit = defineEmits(['remove']);

const isLeaf = computed(() => !props.node.steps || props.node.steps.length === 0);

const calculatedWordCount = computed(() => {
  if (isLeaf.value) {
    return props.node.word_count || 0;
  }
  const countChildren = (nodes: any[]): number => {
    return nodes.reduce((sum, child) => {
      if (child.steps && child.steps.length > 0) {
        return sum + countChildren(child.steps);
      }
      return sum + (Number(child.word_count) || 0);
    }, 0);
  };
  const total = countChildren(props.node.steps || []);
  // This is the critical fix: DO NOT mutate the prop.
  // The parent's word_count will be updated reactively because the children's word_count changes.
  // We just return the calculated value for display.
  if (props.node) {
    props.node.word_count = total;
  }
  return total;
});

const addChild = () => {
  if (!props.node.steps) {
    props.node.steps = [];
  }
  props.node.steps.push({
    id: uuidv4(),
    sub_goal: 'New Sub-section',
    steps: [],
    word_count: 200,
  });
};

const removeNode = () => {
  emit('remove');
};

const removeChild = (index: number) => {
  if (props.node.steps) {
    props.node.steps.splice(index, 1);
  }
};
</script>