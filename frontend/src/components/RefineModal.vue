<!-- frontend/src/components/RefineModal.vue -->
<template>
  <n-modal
    :show="isVisible"
    preset="card"
    :title="`Refine: ${nodeTitle}`"
    class="max-w-2xl"
    @update:show="$emit('close')"
  >
    <div class="space-y-4">
      <div>
        <label for="refine-prompt" class="block text-sm font-medium mb-1">Refinement Instructions</label>
        <textarea
          id="refine-prompt"
          v-model="prompt"
          rows="4"
          class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="e.g., Make this section more concise and add a real-world example."
        ></textarea>
      </div>
      <div>
        <label class="block text-sm font-medium mb-1">Model for Refinement</label>
        <select 
          v-model="selectedModel"
          class="w-full p-2 border border-gray-300 dark:border-gray-600 rounded-md bg-transparent focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option v-for="model in chatStore.availableModels" :key="model.value" :value="model.value">
            {{ model.label }}
          </option>
        </select>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end space-x-3">
        <button @click="$emit('close')" class="px-4 py-2 bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500">Cancel</button>
        <button @click="handleSubmit" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600" :disabled="!prompt.trim() || !selectedModel">Submit</button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { NModal } from 'naive-ui';
import { useChatStore } from '../stores/chat';

const props = defineProps<{
  isVisible: boolean;
  nodeTitle: string;
}>();

const emit = defineEmits(['close', 'submit']);

const chatStore = useChatStore();
const prompt = ref('');
const selectedModel = ref<string | null>(chatStore.getDefaultModel('chat'));

watch(() => props.isVisible, (newValue) => {
  if (newValue) {
    prompt.value = '';
    if (!selectedModel.value) {
      selectedModel.value = chatStore.getDefaultModel('chat');
    }
  }
});

const handleSubmit = () => {
  if (prompt.value.trim() && selectedModel.value) {
    emit('submit', { prompt: prompt.value, model: selectedModel.value });
    emit('close');
  }
};
</script>