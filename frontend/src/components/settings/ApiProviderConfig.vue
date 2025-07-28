<!-- frontend/src/components/settings/ApiProviderConfig.vue -->
<template>
  <transition name="modal-fade">
    <div v-if="isVisible" class="fixed inset-0 bg-black bg-opacity-60 z-50 flex items-center justify-center" @click.self="close">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-3xl m-4 flex flex-col max-h-[90vh]">
        <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
          <h2 class="text-lg font-semibold">{{ isEditing ? 'Edit API Provider' : 'Add New API Provider' }}</h2>
          <button @click="close" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700">
            <X class="w-5 h-5" />
          </button>
        </header>
        
        <main v-if="localProvider" class="flex-1 overflow-y-auto p-6 space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium">Provider Name</label>
              <input type="text" v-model="localProvider.name" placeholder="e.g., My OpenAI Key" class="mt-1 block w-full input-style" />
            </div>
            <div>
              <label class="block text-sm font-medium">Base URL</label>
              <input type="text" v-model="localProvider.baseUrl" class="mt-1 block w-full input-style" />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium">API Key(s)</label>
            <textarea 
              v-model="localProvider.apiKey" 
              rows="3"
              class="mt-1 block w-full input-style font-mono"
              placeholder="Enter one or more API keys, separated by commas"
            ></textarea>
            <p class="text-xs text-gray-500 mt-1">For multiple keys, separate them with a comma. A random key will be used for each request.</p>
          </div>
          
          <div>
            <div class="flex justify-between items-center">
              <label class="block text-sm font-medium">Models</label>
              <button @click="addModel" class="px-3 py-1 text-xs bg-gray-200 dark:bg-gray-600 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500">Add Model</button>
            </div>
            <div class="mt-2 space-y-3 border border-gray-200 dark:border-gray-700 rounded-lg p-3">
              <div v-for="(model, index) in localProvider.models" :key="index" class="grid grid-cols-12 gap-3 items-start">
                <input type="text" v-model="model.name" placeholder="Model Name (e.g., gpt-4-turbo)" class="input-style col-span-12 sm:col-span-4">
                <div class="col-span-12 sm:col-span-2">
                  <n-input-number v-model:value="model.maxTokens" :show-button="false" placeholder="Max Tokens" class="w-full">
                     <template #prefix>
                      <span class="text-xs text-gray-400"></span>
                    </template>
                  </n-input-number>
                </div>
                <div class="col-span-12 sm:col-span-5">
                  <div class="grid grid-cols-2 sm:grid-cols-3 gap-x-4 gap-y-2">
                    <label v-for="cap in allCapabilities" :key="cap.value" class="flex items-center space-x-2 text-sm">
                      <input type="checkbox" :value="cap.value" v-model="model.capabilities" class="form-checkbox h-4 w-4 text-blue-600 rounded">
                      <span>{{ cap.label }}</span>
                    </label>
                  </div>
                </div>
                <div class="col-span-12 sm:col-span-1 flex justify-end items-center">
                  <button @click="removeModel(index)" class="text-red-500 hover:text-red-700 text-xs">Remove</button>
                </div>
              </div>
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium">Proxy (Optional)</label>
            <input 
              type="text" 
              v-model="localProvider.proxy" 
              class="mt-1 block w-full input-style"
              placeholder="e.g., http://127.0.0.1:7890 or socks5://127.0.0.1:1080"
            />
          </div>
        </main>

        <footer class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3 flex-shrink-0">
          <button @click="close" class="px-4 py-2 bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500">Cancel</button>
          <button @click="save" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">Save Provider</button>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { ApiProvider, ModelCapability } from '../../types';
import { X } from 'lucide-vue-next';
import { v4 as uuidv4 } from 'uuid';
import { useSettingsStore } from '../../stores/settings';
import { NInputNumber } from 'naive-ui';

const props = defineProps<{
  modelValue: boolean;
  provider: ApiProvider | null;
}>();

const emit = defineEmits(['update:modelValue']);
const settingsStore = useSettingsStore();

const isVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEditing = computed(() => !!props.provider);

const allCapabilities: { label: string, value: ModelCapability }[] = [
  { label: 'Chat', value: 'chat' },
  { label: 'Vision', value: 'vision' },
  { label: 'Embedding', value: 'embedding' },
  { label: 'Image Gen', value: 'image_gen' },
  { label: 'Video Gen', value: 'video_gen' },
  { label: 'TTS', value: 'tts' },
];

const createDefaultProvider = (): ApiProvider => ({
  id: uuidv4(),
  name: '',
  baseUrl: '',
  apiKey: '',
  models: [{ name: '', capabilities: ['chat'], maxTokens: null }],
  proxy: undefined,
});

const localProvider = ref<ApiProvider>(createDefaultProvider());

watch(() => props.provider, (newProvider) => {
  if (newProvider) {
    localProvider.value = JSON.parse(JSON.stringify(newProvider));
  } else {
    localProvider.value = createDefaultProvider();
  }
}, { immediate: true, deep: true });

const addModel = () => {
  localProvider.value.models.push({ name: '', capabilities: ['chat'], maxTokens: null });
};

const removeModel = (index: number) => {
  localProvider.value.models.splice(index, 1);
};

const close = () => {
  isVisible.value = false;
};

const save = () => {
  if (localProvider.value.proxy === '') {
    localProvider.value.proxy = undefined;
  }
  // Filter out empty model names
  localProvider.value.models = localProvider.value.models.filter(m => m.name.trim() !== '');
  settingsStore.saveApiProvider(localProvider.value);
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