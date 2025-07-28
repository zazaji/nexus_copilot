<!-- frontend/src/components/settings/OnlineKbConfig.vue -->
<template>
  <transition name="modal-fade">
    <div v-if="isVisible" class="fixed inset-0 bg-black bg-opacity-60 z-50 flex items-center justify-center" @click.self="close">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-2xl m-4 flex flex-col max-h-[90vh]">
        <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
          <h2 class="text-lg font-semibold">{{ isEditing ? 'Edit Online KB' : 'Add Online KB' }}</h2>
          <button @click="close" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700">
            <X class="w-5 h-5" />
          </button>
        </header>
        
        <main v-if="localKb" class="flex-1 overflow-y-auto p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium">Name</label>
            <input type="text" v-model="localKb.name" placeholder="e.g., Company API" class="mt-1 block w-full input-style" />
          </div>
          <div>
            <label class="block text-sm font-medium">URL</label>
            <input type="text" v-model="localKb.url" placeholder="e.g., https://api.example.com/retrieve" class="mt-1 block w-full input-style" />
          </div>
          <div>
            <label class="block text-sm font-medium">Token (Header)</label>
            <input type="password" v-model="localKb.token" placeholder="Enter Bearer token or API key" class="mt-1 block w-full input-style" />
          </div>
        </main>

        <footer class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3 flex-shrink-0">
          <button @click="close" class="px-4 py-2 bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500">Cancel</button>
          <button @click="save" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">Save</button>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { OnlineKnowledgeBase } from '../../types';
import { X } from 'lucide-vue-next';
import { v4 as uuidv4 } from 'uuid';

const props = defineProps<{
  modelValue: boolean;
  kb: OnlineKnowledgeBase | null;
}>();

const emit = defineEmits(['update:modelValue', 'save']);

const isVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEditing = computed(() => !!props.kb);

const createDefaultKb = (): OnlineKnowledgeBase => ({
  id: uuidv4(),
  name: '',
  url: '',
  token: '',
});

const localKb = ref<OnlineKnowledgeBase>(createDefaultKb());

watch(() => props.kb, (newKb) => {
  if (newKb) {
    localKb.value = JSON.parse(JSON.stringify(newKb));
  } else {
    localKb.value = createDefaultKb();
  }
}, { immediate: true, deep: true });

const close = () => {
  isVisible.value = false;
};

const save = () => {
  emit('save', localKb.value);
  close();
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>