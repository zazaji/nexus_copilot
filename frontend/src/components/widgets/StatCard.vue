<!-- frontend/src/components/widgets/StatCard.vue -->
<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 flex items-start space-x-4">
    <div class="p-3 bg-blue-100 dark:bg-blue-900/50 rounded-lg flex-shrink-0">
      <component :is="icon" class="w-6 h-6 text-blue-500" />
    </div>
    <div>
      <p class="text-sm font-medium text-gray-500 dark:text-gray-400">{{ title }}</p>
      <p class="text-3xl font-bold">{{ value }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FunctionalComponent } from 'vue';

defineProps<{
  icon: FunctionalComponent;
  title: string;
  value: number | string;
}>();
</script>