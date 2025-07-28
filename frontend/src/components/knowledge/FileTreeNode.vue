<!-- frontend/src/components/knowledge/FileTreeNode.vue -->
<template>
  <div class="group/item">
    <div
      @click.stop="handleSelect"
      @contextmenu.prevent="emit('context-menu', { event: $event, node })"
      class="flex items-center space-x-2 px-2 py-1.5 rounded-md cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
      :class="{ 'bg-blue-50 dark:bg-blue-900/50': isActive }"
      :style="{ paddingLeft: `${level * 1.25 + 0.5}rem` }"
    >
      <!-- Chevron for Folders -->
      <div class="w-4 h-4 flex-shrink-0 flex items-center justify-center">
        <ChevronRight
          v-if="!node.isLeaf"
          @click.stop="emit('toggle-expand', node.key)"
          class="w-4 h-4 text-gray-400 transition-transform"
          :class="{ 'rotate-90': isExpanded }"
        />
      </div>

      <!-- Icon -->
      <component :is="icon" class="w-4 h-4 flex-shrink-0" :class="iconClass" />

      <!-- Title / Input -->
      <div class="flex-1 min-w-0" @dblclick.stop="startEditing">
        <input
          v-if="isEditing"
          ref="editInputRef"
          v-model="editableName"
          @blur="finishEditing"
          @keydown.enter.prevent="finishEditing"
          @keydown.esc="cancelEditing"
          class="bg-transparent focus:outline-none focus:ring-1 focus:ring-blue-500 rounded px-1 text-sm w-full"
        />
        <span v-else class="text-sm truncate" :title="node.title">{{ node.title }}</span>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center opacity-0 group-hover/item:opacity-100 transition-opacity flex-shrink-0">
        <button v-if="!node.isLeaf" @click.stop="emit('open-in-explorer', node.key)" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" :title="$t('knowledge.openFolder')">
          <FolderOpen class="w-4 h-4" />
        </button>
        <button v-if="!node.isLeaf" @click.stop="emit('create-file', node)" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" :title="$t('knowledge.newNote')">
          <FilePlus2 class="w-4 h-4" />
        </button>
        <button v-if="level > 0" @click.stop="emit('delete-file', node)" class="p-1 rounded-md text-gray-400 hover:bg-red-100 hover:text-red-500 dark:hover:bg-red-900/50" :title="$t('knowledge.deleteFile')">
          <Trash2 class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Recursive Children -->
    <div v-if="isExpanded && !node.isLeaf">
      <FileTreeNode
        v-for="child in node.children"
        :key="child.key"
        :node="child"
        :level="level + 1"
        :active-file-key="activeFileKey"
        :expanded-keys="expandedKeys"
        :editing-key="editingKey"
        @select-file="emit('select-file', $event)"
        @toggle-expand="emit('toggle-expand', $event)"
        @context-menu="emit('context-menu', $event)"
        @create-file="emit('create-file', $event)"
        @delete-file="emit('delete-file', $event)"
        @start-rename="emit('start-rename', $event)"
        @finish-rename="emit('finish-rename', $event)"
        @open-in-explorer="emit('open-in-explorer', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, PropType, watch, nextTick } from 'vue';
import type { FileNode } from '../../types';
import { Folder, FileText, ChevronRight, FolderOpen, FilePlus2, Trash2, FileSignature, Presentation } from 'lucide-vue-next';

const props = defineProps({
  node: { type: Object as PropType<FileNode>, required: true },
  level: { type: Number, default: 0 },
  activeFileKey: { type: String as PropType<string | null>, default: null },
  expandedKeys: { type: Set as PropType<Set<string>>, required: true },
  editingKey: { type: String as PropType<string | null>, default: null },
});

const emit = defineEmits([
  'select-file', 'toggle-expand', 'context-menu', 'create-file', 'delete-file', 'start-rename', 'finish-rename', 'open-in-explorer'
]);

const editInputRef = ref<HTMLInputElement | null>(null);
const editableName = ref('');

const isExpanded = computed(() => props.expandedKeys.has(props.node.key));
const isActive = computed(() => props.activeFileKey === props.node.key);
const isEditing = computed(() => props.editingKey === props.node.key);

const icon = computed(() => {
  if (!props.node.isLeaf) return Folder;
  switch (props.node.fileType) {
    case 'pdf': return FileText;
    case 'doc': return FileSignature;
    case 'ppt': return Presentation;
    default: return FileText;
  }
});

const iconClass = computed(() => {
  if (!props.node.isLeaf) return 'text-yellow-500';
  return 'text-gray-500';
});

const handleSelect = () => {
  if (props.node.isLeaf) {
    emit('select-file', props.node);
  } else {
    emit('toggle-expand', props.node.key);
  }
};

const startEditing = () => {
  if (props.node.isLeaf) {
    editableName.value = props.node.title;
    emit('start-rename', props.node);
  }
};

const finishEditing = () => {
  emit('finish-rename', { oldKey: props.node.key, newName: editableName.value });
};

const cancelEditing = () => {
  emit('finish-rename', { oldKey: props.node.key, newName: props.node.title });
};

watch(isEditing, (newValue) => {
  if (newValue) {
    nextTick(() => {
      editInputRef.value?.focus();
      editInputRef.value?.select();
    });
  }
});
</script>