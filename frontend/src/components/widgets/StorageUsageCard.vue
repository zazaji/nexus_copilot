<!-- frontend/src/components/widgets/StorageUsageCard.vue -->
<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-4 flex flex-col h-full">
    <div class="flex items-start justify-between">
      <div class="flex items-start space-x-4">
        <div class="p-3 bg-gray-100 dark:bg-gray-700 rounded-lg flex-shrink-0">
          <component :is="icon" class="w-6 h-6 text-gray-500 dark:text-gray-400" />
        </div>
        <div>
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">{{ title }}</p>
          <p class="text-2xl font-bold">{{ formattedSize }}</p>
        </div>
      </div>
      <span class="text-xs font-semibold px-2 py-1 rounded-full" :class="status.colorClass">{{ status.label }}</span>
    </div>
    <div class="mt-4 flex-1">
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5">
        <div class="h-2.5 rounded-full" :class="status.colorClass" :style="{ width: `${progress}%` }"></div>
      </div>
    </div>
    <div v-if="status.suggestion" class="mt-3 text-xs text-gray-500 dark:text-gray-400 flex items-center space-x-2">
      <AlertTriangle class="w-4 h-4" :class="status.textColorClass" />
      <span>{{ status.suggestion }} <router-link v-if="status.link" :to="status.link" class="text-blue-500 hover:underline">{{ $t('dashboard.storage.actionLink') }}</router-link></span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, FunctionalComponent } from 'vue';
import { useI18n } from 'vue-i18n';
import { AlertTriangle } from 'lucide-vue-next';

const props = defineProps<{
  icon: FunctionalComponent;
  title: string;
  sizeBytes: number;
}>();

const { t } = useI18n();

const KB_IN_BYTES = 1024;
const MB_IN_BYTES = KB_IN_BYTES * 1024;
const GB_IN_BYTES = MB_IN_BYTES * 1024;

const thresholds = computed(() => {
  if (props.title === t('dashboard.storage.database')) {
    return {
      WARNING_THRESHOLD_MB: 20,
      DANGER_THRESHOLD_MB: 200,
      PROGRESS_CAP_MB: 250,
    };
  }
  // Default to Vector Store Size thresholds
  return {
    WARNING_THRESHOLD_MB: 200,
    DANGER_THRESHOLD_MB: 2048, // 2 GB
    PROGRESS_CAP_MB: 2560, // 2.5 GB
  };
});

const formattedSize = computed(() => {
  if (props.sizeBytes > GB_IN_BYTES) {
    return `${(props.sizeBytes / GB_IN_BYTES).toFixed(2)} GB`;
  }
  if (props.sizeBytes > MB_IN_BYTES) {
    return `${(props.sizeBytes / MB_IN_BYTES).toFixed(2)} MB`;
  }
  if (props.sizeBytes > KB_IN_BYTES) {
    return `${(props.sizeBytes / KB_IN_BYTES).toFixed(2)} KB`;
  }
  return `${props.sizeBytes} Bytes`;
});

const sizeInMb = computed(() => props.sizeBytes / MB_IN_BYTES);

const status = computed(() => {
  const { WARNING_THRESHOLD_MB, DANGER_THRESHOLD_MB } = thresholds.value;
  if (sizeInMb.value > DANGER_THRESHOLD_MB) {
    return {
      label: t('dashboard.storage.statusCritical'),
      colorClass: 'bg-red-500',
      textColorClass: 'text-red-500',
      suggestion: t('dashboard.storage.criticalText'),
      link: '/settings'
    };
  }
  if (sizeInMb.value > WARNING_THRESHOLD_MB) {
    return {
      label: t('dashboard.storage.statusWarning'),
      colorClass: 'bg-yellow-500',
      textColorClass: 'text-yellow-500',
      suggestion: t('dashboard.storage.warningText'),
      link: '/settings'
    };
  }
  return {
    label: t('dashboard.storage.statusNormal'),
    colorClass: 'bg-green-500',
    textColorClass: 'text-green-500',
    suggestion: null,
    link: null
  };
});

const progress = computed(() => {
  const { PROGRESS_CAP_MB } = thresholds.value;
  const percentage = (sizeInMb.value / PROGRESS_CAP_MB) * 100;
  return Math.min(percentage, 100);
});
</script>