<!-- frontend/src/components/WriteExecutionView.vue -->
<template>
  <div>
    <div v-if="task.status === 'awaiting_user_input'" class="pl-9 space-y-4">
      <div class="p-3 bg-yellow-50 dark:bg-yellow-900/50 border border-yellow-200 dark:border-yellow-700 rounded-lg">
        <h4 class="font-semibold text-yellow-800 dark:text-yellow-200">Confirm Writing Plan</h4>
        <p class="text-xs text-yellow-700 dark:text-yellow-300 mt-1">Please review and edit the writing strategy and outline below. When you're satisfied, click "Confirm and Continue".</p>
      </div>
      
      <div v-if="editableElaboration" class="space-y-3 p-3 border rounded-lg dark:border-gray-700">
        <h5 class="font-semibold text-sm">Writing Strategy</h5>
        <div>
          <label class="text-xs font-medium text-gray-500 dark:text-gray-400">Summary</label>
          <input type="text" v-model="editableElaboration.summary" class="w-full bg-transparent border-b border-gray-300 dark:border-gray-600 focus:outline-none focus:border-blue-500 text-sm py-1" />
        </div>
        <div>
          <label class="text-xs font-medium text-gray-500 dark:text-gray-400">Style</label>
          <input type="text" v-model="editableElaboration.style" class="w-full bg-transparent border-b border-gray-300 dark:border-gray-600 focus:outline-none focus:border-blue-500 text-sm py-1" />
        </div>
         <div>
          <label class="text-xs font-medium text-gray-500 dark:text-gray-400">Strategy</label>
          <textarea v-model="editableElaboration.strategy" rows="2" class="w-full bg-transparent border-b border-gray-300 dark:border-gray-600 focus:outline-none focus:border-blue-500 text-sm py-1 resize-none"></textarea>
        </div>
      </div>

      <div class="space-y-3">
        <EditableOutlineNode 
          v-for="(node, index) in editablePlan"
          :key="node.id || index"
          :node="node"
          @remove="editablePlan.splice(index, 1)"
        />
      </div>
      <div class="flex justify-end space-x-3 pt-4">
        <button @click="agentStore.stopAgentTask(task.id)" class="px-4 py-2 text-sm bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500">Cancel Task</button>
        <button @click="confirmPlan" class="px-4 py-2 text-sm bg-blue-500 text-white rounded-lg hover:bg-blue-600">Confirm and Continue</button>
      </div>
    </div>
    <div v-else>
      <div v-if="!planSteps || planSteps.length === 0" class="pl-9 text-sm text-gray-500 dark:text-gray-400 flex items-center space-x-2">
          <Loader2 class="w-4 h-4 animate-spin" />
          <span>Agent is planning the steps...</span>
      </div>
      <div v-else class="pl-9 space-y-4">
        <div class="flex items-center space-x-3 text-sm">
          <component :is="getPhaseIcon('Elaboration')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Elaboration')" />
          <span class="font-medium" :class="getPhaseClass('Elaboration')">Phase 1: Elaborating Goal</span>
        </div>
        <div v-if="writeModePhase >= 1 && elaborationData" class="ml-7 pl-4 border-l border-gray-300 dark:border-gray-600 text-xs space-y-1">
          <p><strong class="text-gray-500 dark:text-gray-400">Summary:</strong> {{ elaborationData.summary }}</p>
          <p><strong class="text-gray-500 dark:text-gray-400">Style:</strong> {{ elaborationData.style }}</p>
          <p><strong class="text-gray-500 dark:text-gray-400">Strategy:</strong> {{ elaborationData.strategy }}</p>
        </div>
        <div class="flex items-center space-x-3 text-sm">
          <component :is="getPhaseIcon('Outline')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Outline')" />
          <span class="font-medium" :class="getPhaseClass('Outline')">Phase 2: Generating Outline</span>
        </div>
        <div v-if="writeModePhase >= 2 && planSteps.length > 0">
          <div class="flex items-center space-x-3 text-sm">
            <component :is="getPhaseIcon('Writing')" class="w-4 h-4 flex-shrink-0" :class="getPhaseClass('Writing')" />
            <span class="font-medium" :class="getPhaseClass('Writing')">Phase 3: Writing Content</span>
          </div>
          <div class="ml-7 mt-2 space-y-1">
            <WriteModeStep 
              v-for="(step, index) in planSteps" 
              :key="index"
              :step-node="step"
              :task-steps="task.steps"
              :task-content="task.researchContent"
              @refine="$emit('refine-node', $event)"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, PropType, watch } from 'vue';
import type { AgentTask } from '../types';
import { useAgentStore } from '../stores/agent';
import { Loader2, CheckCircle2, XCircle, CircleDashed, PauseCircle } from 'lucide-vue-next';
import EditableOutlineNode from './EditableOutlineNode.vue';
import WriteModeStep from './WriteModeStep.vue';

const props = defineProps({
  task: {
    type: Object as PropType<AgentTask>,
    required: true,
  },
});

defineEmits(['refine-node']);

const agentStore = useAgentStore();

const planSteps = computed(() => props.task.plan || []);
const editablePlan = ref<any[]>([]);
const editableElaboration = ref<any | null>(null);

const elaborationData = computed(() => {
  if (!props.task || !props.task.steps) return null;
  
  // Find the LAST completed step for elaboration, as there might be intermediate 'running' steps.
  const completedElaborationSteps = props.task.steps.filter(s => 
    s.action === 'Phase 1: Generate Elaboration' && s.status === 'completed'
  );

  const step = completedElaborationSteps[completedElaborationSteps.length - 1];

  if (step && step.result) {
    try {
      return JSON.parse(step.result).elaboration;
    } catch (e) {
      console.error("Failed to parse elaboration from step result:", e);
      return null;
    }
  }
  return null;
});

watch(() => props.task, (newTask) => {
  if (newTask && newTask.status === 'awaiting_user_input') {
    if (newTask.plan) {
      editablePlan.value = JSON.parse(JSON.stringify(newTask.plan));
    }
    if (elaborationData.value) {
      editableElaboration.value = JSON.parse(JSON.stringify(elaborationData.value));
    }
  }
}, { immediate: true, deep: true });

const confirmPlan = () => {
  agentStore.resumeWriteTaskWithPlan(props.task.id, {
    elaboration: editableElaboration.value,
    plan: editablePlan.value
  });
};

const writeModePhase = computed<number>(() => {
  if (!props.task || !props.task.steps) return 0;
  const completedActions = new Set(props.task.steps.filter(s => s.status === 'completed').map(s => s.action));
  if (props.task.steps.some(s => s.action.startsWith('Phase 5'))) return 4;
  if (props.task.steps.some(s => s.action.startsWith('Phase 4'))) return 3;
  if (completedActions.has('Phase 2: Generate Outline')) return 2;
  if (completedActions.has('Phase 1: Generate Elaboration')) return 1;
  return 0;
});

const getPhaseIcon = (phaseName: 'Elaboration' | 'Outline' | 'Writing') => {
  const phaseMap: Record<string, number> = { 'Elaboration': 1, 'Outline': 2, 'Writing': 3 };
  const phaseNumber = phaseMap[phaseName];
  if (props.task.status === 'awaiting_user_input' && phaseName === 'Outline') return PauseCircle;
  if ((props.task.status === 'running' || props.task.status === 'planning') && writeModePhase.value === phaseNumber - 1) return Loader2;
  if (writeModePhase.value >= phaseNumber) return CheckCircle2;
  if (props.task.status === 'failed') return XCircle;
  return CircleDashed;
};

const getPhaseClass = (phaseName: 'Elaboration' | 'Outline' | 'Writing') => {
  const phaseMap: Record<string, number> = { 'Elaboration': 1, 'Outline': 2, 'Writing': 3 };
  const phaseNumber = phaseMap[phaseName];
  if (props.task.status === 'awaiting_user_input' && phaseName === 'Outline') return 'text-yellow-500';
  if ((props.task.status === 'running' || props.task.status === 'planning') && writeModePhase.value === phaseNumber - 1) return 'text-blue-500 animate-spin';
  if (writeModePhase.value >= phaseNumber) return 'text-green-500';
  if (props.task.status === 'failed' && writeModePhase.value < phaseNumber) return 'text-red-500';
  return 'text-gray-500';
};
</script>