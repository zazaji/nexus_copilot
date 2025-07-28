<!-- frontend/src/views/ToolsView.vue -->
<template>
  <div class="flex-1 flex flex-col p-6 bg-gray-50 dark:bg-gray-900/50 overflow-hidden">
    <header class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold">{{ $t('tools.title') }}</h1>
    </header>
    <div class="flex-1 flex bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden">
      <!-- Left: Tool List -->
      <aside class="w-1/3 border-r border-gray-200 dark:border-gray-700 flex flex-col">
        <div class="p-4 border-b border-gray-200 dark:border-gray-700">
          <input 
            type="text" 
            v-model="searchQuery"
            :placeholder="$t('knowledge.searchPlaceholder')"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="flex-1 overflow-y-auto">
          <div v-if="toolsStore.isLoading" class="p-4 text-center text-gray-500">{{ $t('tools.loading') }}</div>
          <div v-else-if="filteredTools.length === 0" class="p-4 text-center text-gray-500">{{ $t('tools.notFound') }}</div>
          <ul v-else>
            <li v-for="tool in filteredTools" :key="tool.id">
              <button 
                @click="handleSelectTool(tool.id)"
                class="w-full text-left p-4 hover:bg-gray-100 dark:hover:bg-gray-700/50"
                :class="{ 'bg-blue-50 dark:bg-blue-900/50': toolsStore.selectedToolId === tool.id }"
              >
                <div class="flex items-center space-x-2">
                    <component :is="getRuntimeIcon(tool)" class="w-4 h-4 text-gray-500" />
                    <p class="font-semibold">{{ tool.name }}</p>
                </div>
                <p class="text-sm text-gray-500 truncate mt-1 pl-6">{{ tool.description }}</p>
              </button>
            </li>
          </ul>
        </div>
      </aside>
      <!-- Right: Execution Panel -->
      <main class="w-2/3">
        <ToolExecutionPanel :tool="toolsStore.selectedTool" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useToolsStore, UnifiedTool } from '../stores/tools';
import ToolExecutionPanel from '../components/tools/ToolExecutionPanel.vue';
import { Terminal, Code, Atom } from 'lucide-vue-next';

const toolsStore = useToolsStore();
const searchQuery = ref('');

onMounted(() => {
  toolsStore.fetchAllTools();
});

const handleSelectTool = (id: string) => {
  toolsStore.selectTool(id);
};

const getRuntimeIcon = (tool: UnifiedTool) => {
    if (tool.type === 'configured') {
        switch(tool.runtime) {
            case 'python': return Code;
            case 'node': return Atom;
            case 'shell': return Terminal;
            default: return Terminal;
        }
    }
    return Code; // Default for dynamic tools (which are python)
};

const filteredTools = computed(() => {
  if (!searchQuery.value) {
    return toolsStore.allTools;
  }
  return toolsStore.allTools.filter(tool => 
    tool.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    tool.description.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});
</script>