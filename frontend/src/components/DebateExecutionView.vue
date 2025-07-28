<!-- frontend/src/components/DebateExecutionView.vue -->
<template>
  <div class="pl-9 space-y-4">
    <!-- Phase 1: Personas -->
    <div class="flex items-center space-x-3 text-sm">
      <component :is="getPhaseIcon('Personas')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Personas')" />
      <span class="font-medium" :class="getPhaseClass('Personas')">Phase 1: Generating Personas</span>
    </div>
    <div v-if="personas" class="ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 grid grid-cols-1 md:grid-cols-3 gap-4">
      <div v-for="role in ['pro', 'con', 'judge']" :key="role" class="p-3 rounded-lg" :class="personaCardClass(role)">
        <h4 class="font-bold capitalize text-base mt-0 mb-2">{{ role }}</h4>
        <p class="text-xs mt-1 mb-0"><strong class="font-semibold">Style:</strong> {{ personas[role]?.style }}</p>
        <p class="text-xs mt-1 mb-0"><strong class="font-semibold">Framework:</strong> {{ personas[role]?.framework }}</p>
      </div>
    </div>

    <!-- Phase 2: Debate Rounds -->
    <div class="flex items-center space-x-3 text-sm">
      <component :is="getPhaseIcon('Rounds')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Rounds')" />
      <span class="font-medium" :class="getPhaseClass('Rounds')">Phase 2: Debate Rounds</span>
    </div>
    <div v-if="rounds && rounds.length > 0" class="ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 space-y-3">
      <div v-for="(round, index) in rounds" :key="index">
        <div @click="toggleRound(index)" class="flex items-center space-x-3 cursor-pointer p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700/50">
          <ChevronRight class="w-4 h-4 text-gray-400 transition-transform" :class="{ 'rotate-90': expandedRounds.has(index) }" />
          <p class="font-semibold text-gray-800 dark:text-gray-200">Round {{ round.round }}: {{ round.rules }}</p>
        </div>
        <div v-if="expandedRounds.has(index)" class="mt-2 ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 space-y-3 pb-2">
          <!-- Pro Argument -->
          <div class="p-3 bg-blue-100/50 dark:bg-blue-900/30 rounded-md border border-blue-200 dark:border-blue-800">
            <div class="flex items-center space-x-2 mb-2 text-blue-800 dark:text-blue-300">
              <Check class="w-4 h-4" />
              <h5 class="font-semibold">Pro Argument</h5>
            </div>
            <div v-if="round.pro_argument" class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedMarkdown(round.pro_argument)"></div>
            <div v-else class="flex items-center space-x-2 text-sm text-gray-500"><Loader2 class="w-3 h-3 animate-spin" /><span>Thinking...</span></div>
          </div>
          <!-- Con Argument -->
          <div class="p-3 bg-red-100/50 dark:bg-red-900/30 rounded-md border border-red-200 dark:border-red-800">
            <div class="flex items-center space-x-2 mb-2 text-red-800 dark:text-red-300">
              <X class="w-4 h-4" />
              <h5 class="font-semibold">Con Argument</h5>
            </div>
            <div v-if="round.con_argument" class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedMarkdown(round.con_argument)"></div>
            <div v-else-if="round.pro_argument" class="flex items-center space-x-2 text-sm text-gray-500"><Loader2 class="w-3 h-3 animate-spin" /><span>Thinking...</span></div>
            <div v-else class="text-sm text-gray-400 italic">Waiting for Pro argument...</div>
          </div>
           <!-- Judge's Evaluation -->
          <div class="p-3 bg-yellow-100/50 dark:bg-yellow-900/30 rounded-md border border-yellow-200 dark:border-yellow-800">
            <div class="flex items-center space-x-2 mb-2 text-yellow-800 dark:text-yellow-300">
              <Gavel class="w-4 h-4" />
              <h5 class="font-semibold">Judge's Evaluation</h5>
            </div>
            <div v-if="round.evaluation" class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedMarkdown(round.evaluation.justification)"></div>
            <div v-else-if="round.con_argument" class="flex items-center space-x-2 text-sm text-gray-500"><Loader2 class="w-3 h-3 animate-spin" /><span>Evaluating...</span></div>
            <div v-else class="text-sm text-gray-400 italic">Waiting for arguments...</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Phase 3: Verdict -->
    <div class="flex items-center space-x-3 text-sm">
      <component :is="getPhaseIcon('Verdict')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Verdict')" />
      <span class="font-medium" :class="getPhaseClass('Verdict')">Phase 3: Final Verdict</span>
    </div>
    <div v-if="verdict" class="ml-7 pl-4 border-l border-gray-300 dark:border-gray-600">
      <div class="p-4 bg-yellow-100/50 dark:bg-yellow-900/30 rounded-lg border border-yellow-300 dark:border-yellow-700">
        <h3 class="font-bold text-2xl mb-3 text-yellow-800 dark:text-yellow-200 flex items-center space-x-2">
          <Gavel class="w-6 h-6" />
          <span>Final Verdict</span>
        </h3>
        <div class="prose prose-sm dark:prose-invert max-w-none" v-html="renderedMarkdown(verdict.justification)"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref } from 'vue';
import type { AgentTask } from '../types';
import { Loader2, Check, X, Gavel, CircleDashed, CheckCircle2, XCircle, ChevronRight } from 'lucide-vue-next';
import { parseMarkdown } from '../utils/markdownParser';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

const expandedRounds = ref(new Set<number>());

const plan = computed(() => props.task.plan || {});
const personas = computed(() => plan.value.personas);
const rounds = computed(() => plan.value.rounds || []);
const verdict = computed(() => plan.value.verdict);

const toggleRound = (index: number) => {
  if (expandedRounds.value.has(index)) {
    expandedRounds.value.delete(index);
  } else {
    expandedRounds.value.add(index);
  }
};

const debatePhase = computed(() => {
  if (verdict.value) return 3; // Verdict
  if (rounds.value.length > 0) return 2; // Rounds
  if (personas.value) return 1; // Personas
  return 0; // Planning
});

const getPhaseIcon = (phaseName: 'Personas' | 'Rounds' | 'Verdict') => {
  const phaseMap: Record<string, number> = { 'Personas': 1, 'Rounds': 2, 'Verdict': 3 };
  const phaseNumber = phaseMap[phaseName];
  if ((props.task.status === 'running' || props.task.status === 'planning') && debatePhase.value === phaseNumber - 1) return Loader2;
  if (debatePhase.value >= phaseNumber) return CheckCircle2;
  if (props.task.status === 'failed') return XCircle;
  return CircleDashed;
};

const getPhaseClass = (phaseName: 'Personas' | 'Rounds' | 'Verdict') => {
  const phaseMap: Record<string, number> = { 'Personas': 1, 'Rounds': 2, 'Verdict': 3 };
  const phaseNumber = phaseMap[phaseName];
  if ((props.task.status === 'running' || props.task.status === 'planning') && debatePhase.value === phaseNumber - 1) return 'text-blue-500 animate-spin';
  if (debatePhase.value >= phaseNumber) return 'text-green-500';
  if (props.task.status === 'failed' && debatePhase.value < phaseNumber) return 'text-red-500';
  return 'text-gray-500';
};

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