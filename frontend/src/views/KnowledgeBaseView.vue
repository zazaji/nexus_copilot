<!-- frontend/src/views/KnowledgeBaseView.vue -->
<template>
  <div class="flex-1 flex flex-col p-6 bg-gray-50 dark:bg-gray-900/50 overflow-hidden">
    <header class="flex justify-between items-center mb-4">
      <div class="flex items-center space-x-4">
        <h1 class="text-2xl font-bold">{{ $t('knowledge.title') }}</h1>
      </div>
      <div class="flex items-center space-x-4">
        <div class="flex border-b border-gray-200 dark:border-gray-700">
          <button
            v-for="tab in tabs"
            :key="tab.name"
            @click="explorerStore.activeTab = tab.name"
            :class="[
              'px-4 py-2 text-sm font-medium',
              explorerStore.activeTab === tab.name
                ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300',
            ]"
          >
            {{ $t(tab.label) }}
          </button>
        </div>
        <button @click="rebuildGraph" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700" title="Rebuild Knowledge Graph">
          <RefreshCw class="w-5 h-5" :class="{ 'animate-spin': isRebuilding }" />
        </button>
      </div>
    </header>

    <div class="flex-1 flex overflow-hidden bg-white dark:bg-gray-800 rounded-lg shadow-sm">
      <!-- Explorer Sidebar -->
      <KnowledgeExplorer
        v-if="isExplorerVisible"
        class="w-[300px] border-r border-gray-200 dark:border-gray-700 flex-shrink-0"
      />

      <!-- Main Content Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <template v-if="explorerStore.activeTab === 'Explorer'">
          <KnowledgeEditor
            :is-explorer-visible="isExplorerVisible"
            @toggle-explorer="isExplorerVisible = !isExplorerVisible"
          />
        </template>
        <template v-else-if="explorerStore.activeTab === 'Graph'">
          <KnowledgeGraph />
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import KnowledgeExplorer from '../components/knowledge/KnowledgeExplorer.vue';
import KnowledgeEditor from '../components/knowledge/KnowledgeEditor.vue';
import KnowledgeGraph from '../components/knowledge/KnowledgeGraph.vue';
import { useKnowledgeExplorerStore } from '../stores/knowledgeExplorer';
import { useKnowledgeBaseStore } from '../stores/knowledgeBase';
import { rebuildKnowledgeGraph } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import { RefreshCw } from 'lucide-vue-next';

const explorerStore = useKnowledgeExplorerStore();
const kbStore = useKnowledgeBaseStore();
const { success } = useToasts();
const isExplorerVisible = ref(true);
const isRebuilding = ref(false);

const tabs = [
  { name: 'Explorer', label: 'knowledge.explorer' },
  { name: 'Graph', label: 'knowledge.graph' },
];

const rebuildGraph = async () => {
  isRebuilding.value = true;
  await rebuildKnowledgeGraph();
  await kbStore.fetchGraphData(); // Refresh graph data after rebuilding
  success('Knowledge graph rebuild complete!');
  isRebuilding.value = false;
};

onMounted(() => {
  if (explorerStore.fileTree.length === 0) {
    explorerStore.loadFileTree();
  }
});
</script>