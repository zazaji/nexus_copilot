<!-- frontend/src/components/widgets/ApiUsageWidget.vue -->
<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm h-full flex flex-col">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
      <h3 class="font-semibold">API Usage Analysis</h3>
      <div class="flex items-center space-x-1 bg-gray-100 dark:bg-gray-700 p-1 rounded-lg">
        <button 
          v-for="range in timeRanges" 
          :key="range"
          @click="dashboardStore.setActiveTimeRange(range)"
          class="px-3 py-1 text-xs rounded-md transition-colors capitalize"
          :class="dashboardStore.activeTimeRange === range ? 'bg-white dark:bg-gray-600 shadow' : 'hover:bg-gray-200 dark:hover:bg-gray-600/50'"
        >
          {{ range }}
        </button>
      </div>
    </div>
    <div v-if="!hasData" class="flex-1 flex items-center justify-center text-gray-500">
      No API usage recorded for this period.
    </div>
    <div v-else class="flex-1 p-4 grid grid-cols-2 grid-rows-2 gap-4 overflow-hidden">
      <!-- KPIs -->
      <div class="col-span-1 row-span-1 bg-gray-50 dark:bg-gray-900/50 p-4 rounded-lg flex flex-col justify-center items-center space-y-2">
        <div class="flex items-center text-gray-500 dark:text-gray-400 text-center">
          <Activity class="w-4 h-4 mr-2 flex-shrink-0"/>
          <span class="text-sm">{{ kpi.title }}</span>
        </div>
        <p class="text-4xl font-bold text-gray-800 dark:text-gray-200">{{ kpi.totalCalls }}</p>
      </div>
      <!-- Doughnut Chart -->
      <div class="col-span-1 row-span-1 relative p-2 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
        <Doughnut :data="doughnutChartData" :options="doughnutChartOptions" @click="handleChartClick" />
      </div>
      <!-- Bar Chart -->
      <div class="col-span-2 row-span-1 relative p-2 bg-gray-50 dark:bg-gray-900/50 rounded-lg">
        <Bar :data="barChartData" :options="barChartOptions" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useDashboardStore } from '../../stores/dashboard';
import { useSettingsStore } from '../../stores/settings';
import { Bar, Doughnut } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement, RadialLinearScale, PointElement, LineElement, Filler } from 'chart.js';
import { Activity } from 'lucide-vue-next';
import { getElementAtEvent } from 'vue-chartjs';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement, RadialLinearScale, PointElement, LineElement, Filler);

type TimeRange = 'day' | 'week' | 'month' | 'all';

const dashboardStore = useDashboardStore();
const settingsStore = useSettingsStore();
const timeRanges: TimeRange[] = ['day', 'week', 'month', 'all'];

const hasData = computed(() => dashboardStore.aggregatedApiStats.allModels.size > 0);

const colorPalette = [
  'hsla(210, 70%, 60%, 0.8)', // Blue
  'hsla(160, 70%, 45%, 0.8)', // Green
  'hsla(350, 70%, 60%, 0.8)', // Red
  'hsla(45, 90%, 60%, 0.8)',  // Yellow
  'hsla(260, 70%, 65%, 0.8)', // Purple
  'hsla(25, 80%, 55%, 0.8)',  // Orange
];

const kpi = computed(() => {
  const { modelTotals } = dashboardStore.aggregatedApiStats;
  const selected = dashboardStore.selectedModel;

  if (selected) {
    return {
      title: `Total Calls for ${selected}`,
      totalCalls: modelTotals[selected] || 0,
    };
  }
  
  return {
    title: 'Total API Calls',
    totalCalls: Object.values(modelTotals).reduce((sum, count) => sum + count, 0),
  };
});

const barChartData = computed(() => {
  const data = dashboardStore.apiCallStats;
  const { labels, allModels } = dashboardStore.aggregatedApiStats;
  const selected = dashboardStore.selectedModel;
  if (!data || data.length === 0) return { labels: [], datasets: [] };

  if (selected) {
    const modelIndex = Array.from(allModels).indexOf(selected);
    const datasets = [{
      label: selected,
      backgroundColor: colorPalette[modelIndex % colorPalette.length],
      data: data.map(period => {
        let total = 0;
        Object.values(period.stats).forEach(service => {
          total += service[selected] || 0;
        });
        return total;
      }),
    }];
    return { labels, datasets };
  }

  const modelArray = Array.from(allModels);
  const datasets = modelArray.map((model, index) => ({
    label: model,
    backgroundColor: colorPalette[index % colorPalette.length],
    data: data.map(period => {
      let total = 0;
      Object.values(period.stats).forEach(service => {
        total += service[model] || 0;
      });
      return total;
    }),
  }));

  return { labels, datasets };
});

const doughnutChartData = computed(() => {
  const { allModels, modelTotals } = dashboardStore.aggregatedApiStats;
  const modelArray = Array.from(allModels);
  
  return {
    labels: modelArray,
    datasets: [
      {
        backgroundColor: modelArray.map((_, index) => colorPalette[index % colorPalette.length]),
        data: modelArray.map(model => modelTotals[model] || 0),
        borderColor: settingsStore.settings?.appearance.theme === 'dark' ? '#1f2937' : '#ffffff',
        borderWidth: 2,
      },
    ],
  };
});

const isDarkMode = computed(() => {
    const theme = settingsStore.settings?.appearance.theme;
    if (theme === 'dark') return true;
    if (theme === 'system') return window.matchMedia('(prefers-color-scheme: dark)').matches;
    return false;
});

const baseChartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
  },
}));

const barChartOptions = computed(() => ({
  ...baseChartOptions.value,
  plugins: { ...baseChartOptions.value.plugins, title: { display: true, text: 'API Calls Trend', color: isDarkMode.value ? '#9ca3af' : '#6b7280' } },
  scales: {
    x: { stacked: !dashboardStore.selectedModel, ticks: { color: isDarkMode.value ? '#9ca3af' : '#6b7280' }, grid: { color: isDarkMode.value ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)' } },
    y: { stacked: !dashboardStore.selectedModel, beginAtZero: true, ticks: { color: isDarkMode.value ? '#9ca3af' : '#6b7280', precision: 0 }, grid: { color: isDarkMode.value ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)' } },
  },
}));

const doughnutChartOptions = computed(() => ({
  ...baseChartOptions.value,
  plugins: { ...baseChartOptions.value.plugins, title: { display: true, text: 'Model Distribution', color: isDarkMode.value ? '#9ca3af' : '#6b7280' } },
}));

const handleChartClick = (event: MouseEvent) => {
  const chartElement = getElementAtEvent(event.target as HTMLCanvasElement, event);
  if (chartElement.length > 0) {
    const { index } = chartElement[0];
    const modelName = doughnutChartData.value.labels[index];
    dashboardStore.selectModel(modelName);
  }
};
</script>