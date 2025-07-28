<!-- frontend/src/components/widgets/TopModelsWidget.vue -->
<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm flex flex-col h-full">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <h3 class="font-semibold">Top 10 Models</h3>
    </div>
    <div class="flex-1 p-4 space-y-3 overflow-y-auto">
      <div v-if="topModels.length === 0" class="flex items-center justify-center h-full text-gray-500 text-sm">
        No model usage data for this period.
      </div>
      <div v-for="(model, index) in topModels" :key="model.name" class="space-y-1">
        <div class="flex justify-between items-center text-sm">
          <div class="flex items-center space-x-2 min-w-0">
            <span class="font-mono text-xs w-5 text-center text-gray-400">{{ index + 1 }}.</span>
            <p class="font-medium text-gray-700 dark:text-gray-300 truncate" :title="model.name">{{ model.name }}</p>
          </div>
          <p class="font-semibold font-mono">{{ model.count }}</p>
        </div>
        <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5 ml-1">
          <div class="bg-blue-500 h-1.5 rounded-full" :style="{ width: `${model.percentage}%` }"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useDashboardStore } from '../../stores/dashboard';

const dashboardStore = useDashboardStore();

const topModels = computed(() => {
  const { modelTotals } = dashboardStore.aggregatedApiStats;

  const sortedModels = Object.entries(modelTotals)
    .map(([name, count]) => ({ name, count }))
    .sort((a, b) => b.count - a.count);

  const totalCalls = sortedModels.reduce((sum, model) => sum + model.count, 0);
  if (totalCalls === 0) return [];

  return sortedModels
    .slice(0, 10)
    .map(model => ({
      ...model,
      percentage: (model.count / totalCalls) * 100,
    }));
});
</script>