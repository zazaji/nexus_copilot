<!-- frontend/src/components/knowledge/GraphDetailPanel.vue -->
<template>
  <div class="p-4 h-full flex flex-col overflow-hidden">
    <h3 class="font-semibold text-lg mb-4 border-b pb-3 border-gray-200 dark:border-gray-700 flex-shrink-0">
      {{ selectedNote ? selectedNote.title : 'Node Details' }}
    </h3>
    <div v-if="!nodeId" class="flex-1 flex items-center justify-center text-center text-gray-500">
      <p>Select a node in the graph to see its details and connections.</p>
    </div>
    <div v-else-if="isLoading" class="flex-1 flex items-center justify-center">
      <Loader2 class="w-6 h-6 animate-spin text-blue-500" />
    </div>
    <div v-else-if="selectedNote" class="flex-1 overflow-y-auto space-y-6">
      <div>
        <h4 class="font-semibold text-sm mb-2 text-gray-500 dark:text-gray-400">Content Preview</h4>
        <p class="text-sm bg-gray-50 dark:bg-gray-700/50 p-3 rounded-md max-h-40 overflow-y-auto">
          {{ selectedNote.content.substring(0, 500) }}{{ selectedNote.content.length > 500 ? '...' : '' }}
        </p>
      </div>
      <div>
        <h4 class="font-semibold text-sm mb-2 text-gray-500 dark:text-gray-400">Backlinks ({{ selectedNote.backlinks.length }})</h4>
        <ul class="space-y-1 text-sm">
          <li v-for="link in selectedNote.backlinks" :key="link.note_id">
            <a @click.prevent="$emit('navigate', link.note_id)" href="#" class="text-blue-500 hover:underline">{{ link.note_title }}</a>
          </li>
        </ul>
      </div>
      <div>
        <h4 class="font-semibold text-sm mb-2 text-gray-500 dark:text-gray-400">Outgoing Links ({{ selectedNote.outgoing_links.length }})</h4>
        <ul class="space-y-1 text-sm">
          <li v-for="link in selectedNote.outgoing_links" :key="link.note_id">
            <a @click.prevent="$emit('navigate', link.note_id)" href="#" class="text-blue-500 hover:underline">{{ link.note_title }}</a>
          </li>
        </ul>
      </div>
    </div>
    <div v-else class="flex-1 flex items-center justify-center text-red-500">
      <p>Could not load details for the selected node.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { getNoteDetails } from '../../lib/api';
import type { KnowledgeNote } from '../../types';
import { Loader2 } from 'lucide-vue-next';

const props = defineProps<{
  nodeId: string | null;
}>();

defineEmits(['navigate']);

const selectedNote = ref<KnowledgeNote | null>(null);
const isLoading = ref(false);

watch(() => props.nodeId, async (newId) => {
  if (newId) {
    isLoading.value = true;
    selectedNote.value = null;
    const note = await getNoteDetails(newId);
    selectedNote.value = note;
    isLoading.value = false;
  } else {
    selectedNote.value = null;
  }
}, { immediate: true });
</script>