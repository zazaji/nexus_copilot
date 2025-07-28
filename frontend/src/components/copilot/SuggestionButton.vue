<!-- frontend/src/components/copilot/SuggestionButton.vue -->
<template>
  <button class="flex items-center space-x-2 p-3 bg-white/5 dark:bg-white/5 hover:bg-white/10 rounded-lg transition-colors text-left border border-white/10 hover:border-white/20">
    <component :is="iconComponent" class="w-5 h-5 text-blue-400 flex-shrink-0" />
    <span class="text-sm font-medium text-gray-200">{{ suggestion.label }}</span>
  </button>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { IntentSuggestion } from '../../types';
import * as icons from 'lucide-vue-next';

const props = defineProps({
  suggestion: {
    type: Object as PropType<IntentSuggestion>,
    required: true,
  },
});

const iconComponent = computed(() => {
  const iconName = props.suggestion.icon as keyof typeof icons;
  return icons[iconName] || icons.HelpCircle;
});
</script>