<!-- frontend/src/components/DebateArtifactDetail.vue -->
<template>
  <div class="space-y-6 prose prose-sm dark:prose-invert max-w-none">
    <!-- Personas -->
    <div v-if="personas" class="not-prose grid grid-cols-1 md:grid-cols-3 gap-4">
      <div v-for="role in ['pro', 'con', 'judge']" :key="role" class="p-3 rounded-lg" :class="personaCardClass(role)">
        <h4 class="font-bold capitalize text-base mt-0 mb-2">{{ role }}</h4>
        <p class="text-xs mt-1 mb-0"><strong class="font-semibold">Style:</strong> {{ personas[role]?.style }}</p>
        <p class="text-xs mt-1 mb-0"><strong class="font-semibold">Framework:</strong> {{ personas[role]?.framework }}</p>
      </div>
    </div>

    <!-- Rounds -->
    <div v-if="rounds && rounds.length > 0" class="space-y-4">
      <div v-for="(round, index) in rounds" :key="index" class="p-4 bg-gray-100 dark:bg-gray-700/30 rounded-lg">
        <h3 class="font-bold text-lg mt-0 mb-3">Round {{ round.round }}: {{ round.rules }}</h3>
        <div class="space-y-3">
          <div>
            <h4 class="font-semibold mt-0 mb-2 border-b border-gray-300 dark:border-gray-600 pb-1">Pro Argument</h4>
            <div v-if="round.pro_argument" v-html="renderedMarkdown(round.pro_argument)"></div>
          </div>
          <div>
            <h4 class="font-semibold mt-0 mb-2 border-b border-gray-300 dark:border-gray-600 pb-1">Con Argument</h4>
            <div v-if="round.con_argument" v-html="renderedMarkdown(round.con_argument)"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { AgentTask } from '../types';
import { parseMarkdown } from '../utils/markdownParser';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const plan = computed(() => props.task.plan || {});
const personas = computed(() => plan.value.personas);
const rounds = computed(() => plan.value.rounds || []);

const personaCardClass = (role: string) => {
  switch (role) {
    case 'pro': return 'bg-blue-100/50 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300 border border-blue-200 dark:border-blue-800';
    case 'con': return 'bg-red-100/50 dark:bg-red-900/30 text-red-800 dark:text-red-300 border border-red-200 dark:border-red-800';
    case 'judge': return 'bg-yellow-100/50 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800';
    default: return 'bg-gray-100 dark:bg-gray-700';
  }
};

const renderedMarkdown = (text: string) => {
  return parseMarkdown(text || '');
};
</script>