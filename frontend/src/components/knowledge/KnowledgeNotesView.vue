<!-- frontend/src/components/knowledge/KnowledgeNotesView.vue -->
<template>
  <div class="flex-1 flex flex-col bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
      <h2 class="font-semibold">All Notes</h2>
      <button @click="explorerStore.createNoteInRoot()" class="flex items-center px-3 py-1.5 bg-blue-500 text-white rounded-lg hover:bg-blue-600 text-sm">
        <Plus class="w-4 h-4 mr-1" />
        New Note
      </button>
    </div>
    <div class="flex-1 overflow-y-auto">
      <div v-if="kbStore.notes.length > 0" class="divide-y divide-gray-200 dark:divide-gray-700">
        <div v-for="note in kbStore.notes" :key="note.id" class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50">
          <p class="font-medium">{{ note.title }}</p>
          <p class="text-xs text-gray-500">Updated: {{ new Date(note.updated_at).toLocaleDateString() }}</p>
        </div>
      </div>
      <div v-else class="flex flex-col items-center justify-center h-full text-center text-gray-500 p-8">
        <Notebook class="w-16 h-16 mb-4 text-gray-400" />
        <h2 class="text-xl font-semibold">Create Your First Note</h2>
        <p class="mt-1">Notes are automatically linked to build your knowledge graph.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { useKnowledgeExplorerStore } from '../../stores/knowledgeExplorer';
import { Plus, Notebook } from 'lucide-vue-next';

const kbStore = useKnowledgeBaseStore();
const explorerStore = useKnowledgeExplorerStore();

onMounted(() => {
  kbStore.fetchNotes();
});
</script>