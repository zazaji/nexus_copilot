// frontend/src/stores/ui.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useUiStore = defineStore('ui', () => {
  const isSidebarCollapsed = ref(false);
  const isArtifactPanelVisible = ref(true);
  const previewedTaskId = ref<string | null>(null);

  function toggleSidebar() {
    isSidebarCollapsed.value = !isSidebarCollapsed.value;
  }

  function toggleArtifactPanel(taskId: string) {
    if (isArtifactPanelVisible.value && previewedTaskId.value === taskId) {
      // If the panel is open and showing the clicked task, close it.
      isArtifactPanelVisible.value = false;
      previewedTaskId.value = null;
    } else {
      // Otherwise, show the panel and set it to the clicked task.
      isArtifactPanelVisible.value = true;
      previewedTaskId.value = taskId;
    }
  }

  function showArtifactPanelForTask(taskId: string) {
    isArtifactPanelVisible.value = true;
    previewedTaskId.value = taskId;
  }

  function hideArtifactPanel() {
    isArtifactPanelVisible.value = false;
    previewedTaskId.value = null;
  }

  return {
    isSidebarCollapsed,
    isArtifactPanelVisible,
    previewedTaskId,
    toggleSidebar,
    toggleArtifactPanel,
    showArtifactPanelForTask,
    hideArtifactPanel,
  };
});