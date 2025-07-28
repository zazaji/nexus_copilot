<!-- frontend/src/components/knowledge/EditorToolbar.vue -->
<template>
  <div class="flex items-center space-x-1 p-2 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
    <button v-for="button in buttons" :key="button.label" @click="button.action" :title="button.label" class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600">
      <component :is="button.icon" class="w-5 h-5" />
    </button>
  </div>
</template>

<script setup lang="ts">
import {
  toggleMark,
  wrapIn,
  setBlockType,
} from '@milkdown/prose/commands';
import { redo, undo } from '@milkdown/prose/history';
import {
  Bold, Italic, Strikethrough, Code, Heading1, Heading2, Heading3, List, ListOrdered, Quote, Undo, Redo
} from 'lucide-vue-next';
import { computed } from 'vue';
import { useInstance } from '@milkdown/vue';
import { gfm } from '@milkdown/preset-gfm';
import { callCommand } from '@milkdown/utils';

const [loading, editor] = useInstance();

// Use the official utility to call commands
const call = <T>(command: T, payload?: any) => {
  if (loading.value) return;
  return editor.value.action(callCommand(command, payload));
};

const buttons = computed(() => [
  { label: 'Bold', icon: Bold, action: () => call(gfm.commands.toggleStrong.key) },
  { label: 'Italic', icon: Italic, action: () => call(gfm.commands.toggleEmphasis.key) },
  { label: 'Strikethrough', icon: Strikethrough, action: () => call(gfm.commands.toggleStrikethrough.key) },
  { label: 'Code', icon: Code, action: () => call(gfm.commands.toggleInlineCode.key) },
  { label: 'H1', icon: Heading1, action: () => call(gfm.commands.turnIntoHeading.key, 1) },
  { label: 'H2', icon: Heading2, action: () => call(gfm.commands.turnIntoHeading.key, 2) },
  { label: 'H3', icon: Heading3, action: () => call(gfm.commands.turnIntoHeading.key, 3) },
  { label: 'Bullet List', icon: List, action: () => call(gfm.commands.wrapInBulletList.key) },
  { label: 'Ordered List', icon: ListOrdered, action: () => call(gfm.commands.wrapInOrderedList.key) },
  { label: 'Quote', icon: Quote, action: () => call(gfm.commands.wrapInBlockquote.key) },
  { label: 'Undo', icon: Undo, action: () => call(undo) },
  { label: 'Redo', icon: Redo, action: () => call(redo) },
]);
</script>