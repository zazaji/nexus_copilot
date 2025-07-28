// frontend/src/stores/tools.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { listConfiguredTools, saveConfiguredTool, deleteConfiguredTool as apiDeleteTool, listTools } from '../lib/api';
import type { ConfiguredTool, DynamicTool } from '../types';
import { useToasts } from '../composables/useToasts';

export type UnifiedTool = (DynamicTool & { type: 'dynamic' }) | (ConfiguredTool & { type: 'configured' });

export const useToolsStore = defineStore('tools', () => {
  const { success } = useToasts();
  const { t } = useI18n();
  const allTools = ref<UnifiedTool[]>([]);
  const isLoading = ref(false);
  const selectedToolId = ref<string | null>(null);

  const selectedTool = computed(() => {
    if (!selectedToolId.value) return null;
    return allTools.value.find(t => t.id === selectedToolId.value) || null;
  });

  function selectTool(id: string | null) {
    selectedToolId.value = id;
  }

  async function fetchAllTools() {
    isLoading.value = true;
    const [dynamicTools, configuredTools] = await Promise.all([
      listTools(),
      listConfiguredTools()
    ]);

    const unified: UnifiedTool[] = [];

    if (dynamicTools) {
      unified.push(...dynamicTools.map(t => ({ ...t, type: 'dynamic' as const, scriptPath: t.script_path })));
    }
    if (configuredTools) {
      unified.push(...configuredTools.map(t => ({ ...t, type: 'configured' as const })));
    }
    
    allTools.value = unified.sort((a, b) => a.name.localeCompare(b.name));
    
    isLoading.value = false;
  }

  async function saveTool(tool: ConfiguredTool) {
    await saveConfiguredTool(tool);
    success(t('tools.saveSuccess', { name: tool.name }));
    await fetchAllTools();
  }

  async function deleteTool(id: string) {
      await apiDeleteTool(id);
      success(t('tools.deleteSuccess'));
      await fetchAllTools();
      if (selectedToolId.value === id) {
        selectedToolId.value = null;
    }
  }

  return {
    allTools,
    isLoading,
    selectedToolId,
    selectedTool,
    selectTool,
    fetchAllTools,
    saveTool,
    deleteTool,
  };
});