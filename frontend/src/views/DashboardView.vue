<!-- frontend/src/views/DashboardView.vue -->
<template>
  <div class="flex-1 flex flex-col bg-gray-100 dark:bg-gray-900 overflow-y-auto">
    <!-- Header -->
    <div class="p-6 flex justify-between items-center">
      <h1 class="text-2xl font-bold text-gray-800 dark:text-gray-200">{{ $t('dashboard.title') }}</h1>
      <button @click="refreshData" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400">
        <RefreshCw class="w-5 h-5" :class="{ 'animate-spin': dashboardStore.isLoading }" />
      </button>
    </div>

    <div class="flex-1 px-6 pb-6 space-y-6">
      <!-- Top Row: Stats Cards -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard 
          v-for="stat in stats" 
          :key="stat.title"
          :icon="stat.icon"
          :title="$t(stat.title)"
          :value="stat.value"
        />
      </div>

      <!-- Main Dashboard Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
        <!-- Left Column: Main Chart -->
        <div class="lg:col-span-8">
          <ApiUsageWidget />
        </div>
        
        <!-- Right Column: Side Widgets -->
        <div class="lg:col-span-4 flex flex-col space-y-6">
          <!-- Combined Storage Usage Card -->
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-4 flex flex-col h-full">
            <h3 class="font-semibold mb-4 px-2">Storage Overview</h3>
            <div class="space-y-4">
              <div v-for="storage in storageStats" :key="storage.title" class="flex items-center space-x-3 px-2">
                <div class="p-2 bg-gray-100 dark:bg-gray-700 rounded-lg">
                  <component :is="storage.icon" class="w-5 h-5 text-gray-500 dark:text-gray-400" />
                </div>
                <div class="flex-1">
                  <p class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ storage.title }}</p>
                </div>
                <p class="text-sm font-mono font-semibold">{{ storage.value }}</p>
              </div>
            </div>
          </div>

          <TopModelsWidget />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useDashboardStore } from '../stores/dashboard';
import StatCard from '../components/widgets/StatCard.vue';
import ApiUsageWidget from '../components/widgets/ApiUsageWidget.vue';
import TopModelsWidget from '../components/widgets/TopModelsWidget.vue';
import { RefreshCw, FileText, Database, MessageSquare, Wrench, HardDrive, DatabaseZap, Server } from 'lucide-vue-next';

const dashboardStore = useDashboardStore();

const refreshData = () => {
  dashboardStore.fetchDashboardStats();
  dashboardStore.fetchApiCallStats('day');
};

onMounted(async () => {
  await dashboardStore.initializeDashboard();
});

const formatBytes = (bytes: number, decimals = 2) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
};

const stats = computed(() => [
  {
    title: 'dashboard.stats.notes',
    value: dashboardStore.stats?.notesCount ?? 0,
    icon: FileText,
  },
  {
    title: 'dashboard.stats.vectors',
    value: dashboardStore.stats?.vectorsCount ?? 0,
    icon: Database,
  },
  {
    title: 'dashboard.stats.conversations',
    value: dashboardStore.stats?.conversationsCount ?? 0,
    icon: MessageSquare,
  },
  {
    title: 'dashboard.stats.tools',
    value: dashboardStore.stats?.toolsCount ?? 0,
    icon: Wrench,
  },
]);

const storageStats = computed(() => [
  {
    title: 'Tauri DB Size',
    icon: HardDrive,
    value: formatBytes(dashboardStore.stats?.dbSize ?? 0),
  },
  {
    title: 'Backend DB Size',
    icon: Server,
    value: formatBytes(dashboardStore.stats?.backendDbSize ?? 0),
  },
  {
    title: 'Vector DB Size',
    icon: DatabaseZap,
    value: formatBytes(dashboardStore.stats?.vectorDbSize ?? 0),
  },
]);
</script>