// frontend/src/stores/dashboard.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { DashboardWidget, DashboardStats, ApiCallStatsTimeseries } from '../types';
import { useToolsStore } from '../stores/tools';
import { getDashboardStats, getApiCallStats } from '../lib/api';
import { useSettingsStore } from './settings';
import { getClient } from '@tauri-apps/api/http';

type TimeRange = 'day' | 'week' | 'month' | 'all';

export const useDashboardStore = defineStore('dashboard', () => {
  const widgets = ref<DashboardWidget[]>([]);
  const stats = ref<DashboardStats | null>(null);
  const apiCallStats = ref<ApiCallStatsTimeseries | null>(null);
  const isLoading = ref(false);
  const isInitialized = ref(false);
  const selectedModel = ref<string | null>(null);
  const activeTimeRange = ref<TimeRange>('day');

  const toolsStore = useToolsStore();
  const settingsStore = useSettingsStore();

  const aggregatedApiStats = computed(() => {
    const data = apiCallStats.value;
    if (!data) return { labels: [], allModels: new Set<string>(), modelTotals: {}, serviceTotals: {} };

    const labels = data.map(d => d.date);
    const modelTotals: Record<string, number> = {};
    const serviceTotals: Record<string, number> = {};

    data.forEach(period => {
      Object.entries(period.stats).forEach(([service, models]) => {
        serviceTotals[service] = (serviceTotals[service] || 0) + Object.values(models).reduce((a, b) => a + b, 0);
        Object.entries(models).forEach(([model, count]) => {
          modelTotals[model] = (modelTotals[model] || 0) + count;
        });
      });
    });
    
    const allModels = new Set(Object.keys(modelTotals));

    return { labels, allModels, modelTotals, serviceTotals };
  });

  async function fetchDashboardStats() {
    isLoading.value = true;
    
    const tauriStatsPromise = getDashboardStats();
    
    const backendStatsPromise = (async () => {
      if (!settingsStore.settings) {
        await settingsStore.fetchSettings();
      }
      const backendUrl = settingsStore.settings?.execution.backendUrl;
      if (!backendUrl) return null;
      
      try {
        const client = await getClient();
        const response = await client.get(`${backendUrl}/api/v1/dashboard/stats`);
        if (response.ok) {
          return response.data as any;
        }
        return null;
      } catch (e) {
        console.error("Failed to fetch backend stats:", e);
        return null;
      }
    })();

    const [tauriStats, backendStats] = await Promise.all([tauriStatsPromise, backendStatsPromise]);

    stats.value = {
      notesCount: tauriStats?.notesCount ?? 0,
      conversationsCount: tauriStats?.conversationsCount ?? 0,
      toolsCount: tauriStats?.toolsCount ?? 0,
      dbSize: tauriStats?.dbSize ?? 0,
      vectorsCount: backendStats?.vectorsCount ?? 0,
      vectorDbSize: backendStats?.vectorDbSize ?? 0,
      backendDbSize: backendStats?.backendDbSize ?? 0,
    };

    isLoading.value = false;
  }

  async function fetchApiCallStats(timeRange: TimeRange) {
    selectedModel.value = null; // Reset selection on time range change
    const result = await getApiCallStats(timeRange);
    if (result) {
      apiCallStats.value = result;
    }
  }

  function setActiveTimeRange(timeRange: TimeRange) {
    if (activeTimeRange.value !== timeRange) {
      activeTimeRange.value = timeRange;
      fetchApiCallStats(timeRange);
    }
  }

  function initializeDashboard() {
    if (isInitialized.value) return;
    
    const savedLayout = localStorage.getItem('dashboardLayout');
    if (savedLayout) {
      widgets.value = JSON.parse(savedLayout);
    } else {
      widgets.value = [];
    }
    
    toolsStore.fetchAllTools();
    fetchDashboardStats();
    fetchApiCallStats(activeTimeRange.value);
    isInitialized.value = true;
  }

  function saveLayout(newLayout: DashboardWidget[]) {
    widgets.value = newLayout;
    localStorage.setItem('dashboardLayout', JSON.stringify(newLayout));
  }

  function selectModel(model: string | null) {
    if (selectedModel.value === model) {
      selectedModel.value = null; // Toggle off if same model is clicked again
    } else {
      selectedModel.value = model;
    }
  }

  return {
    widgets,
    stats,
    apiCallStats,
    isLoading,
    selectedModel,
    activeTimeRange,
    aggregatedApiStats,
    initializeDashboard,
    fetchDashboardStats,
    fetchApiCallStats,
    setActiveTimeRange,
    saveLayout,
    selectModel,
  };
});