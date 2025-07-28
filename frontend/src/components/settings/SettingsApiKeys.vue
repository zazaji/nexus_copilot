<!-- frontend/src/components/settings/SettingsApiKeys.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">{{ $t('settings.apiKeys.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.apiKeys.description') }}</p>
    
    <div v-if="editableSettings.apiConfig && editableSettings.knowledgeBase" class="mt-6 space-y-8">
      <!-- Model Assignments -->
      <div>
        <h3 class="font-semibold text-base mb-4">{{ $t('settings.apiKeys.modelAssignmentsTitle') }}</h3>
        <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4">
            <!-- General & Agent Assignments -->
            <div class="space-y-4">
              <div v-for="assignment in mainAssignments" :key="assignment.key" class="grid grid-cols-2 items-center gap-4">
                <label :for="`assignment-${assignment.key}`" class="block text-sm font-medium">{{ $t(assignment.label) }}</label>
                <select :id="`assignment-${assignment.key}`" v-model="assignment.model.value" class="mt-1 block w-full input-style">
                  <option :value="null">{{ $t('common.none') }}</option>
                  <option v-for="opt in getModelOptionsFor(assignment.capability)" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </div>

            <!-- Creation & Other Assignments -->
            <div class="space-y-4">
               <div v-for="assignment in otherAssignments" :key="assignment.key" class="grid grid-cols-2 items-center gap-4">
                <label :for="`assignment-${assignment.key}`" class="block text-sm font-medium">{{ $t(assignment.label) }}</label>
                <select :id="`assignment-${assignment.key}`" v-model="assignment.model.value" class="mt-1 block w-full input-style">
                  <option :value="null">{{ $t('common.none') }}</option>
                  <option v-for="opt in getModelOptionsFor(assignment.capability)" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
                </select>
              </div>
            </div>
          </div>
          <div class="pt-6 mt-4 border-t border-gray-200 dark:border-gray-700">
            <button @click="saveBasicSettings" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">{{ $t('settings.apiKeys.save') }}</button>
          </div>
        </div>
      </div>

      <!-- API Providers List -->
      <div class="pt-6 border-t border-gray-200 dark:border-gray-700">
        <div class="flex justify-between items-center">
          <h3 class="font-semibold text-base">{{ $t('settings.apiKeys.providers') }}</h3>
          <button @click="openProviderModal(null)" class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm">
            {{ $t('settings.apiKeys.addProvider') }}
          </button>
        </div>
        <div class="mt-4">
          <div v-if="settingsStore.settings?.apiConfig.providers.length === 0" class="p-4 text-center text-sm text-gray-500 border border-dashed border-gray-200 dark:border-gray-700 rounded-lg">
            {{ $t('settings.apiKeys.noProviders') }}
          </div>
          <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div v-for="provider in settingsStore.settings?.apiConfig.providers" :key="provider.id" class="p-4 flex justify-between items-center border border-gray-200 dark:border-gray-700 rounded-lg">
              <div>
                <p class="font-semibold">{{ provider.name }}</p>
                <p class="text-sm text-gray-500 font-mono">{{ provider.baseUrl }}</p>
              </div>
              <div class="flex items-center space-x-2 flex-shrink-0">
                <button @click="openProviderModal(provider)" class="p-2 text-gray-500 hover:text-blue-600"><Pencil class="w-4 h-4" /></button>
                <button @click="deleteProvider(provider.id)" class="p-2 text-gray-500 hover:text-red-600"><Trash2 class="w-4 h-4" /></button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <ApiProviderConfig 
      v-model="isModalVisible"
      :provider="editingProvider"
    />
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '../../stores/settings';
import { useToasts } from '../../composables/useToasts';
import type { Settings, ApiProvider, ModelEndpoint, ModelCapability } from '../../types';
import ApiProviderConfig from './ApiProviderConfig.vue';
import { Pencil, Trash2 } from 'lucide-vue-next';

type AssignmentKey = keyof Settings['apiConfig']['assignments'];

const { t } = useI18n();
const settingsStore = useSettingsStore();
const { success } = useToasts();

const editableSettings = reactive<Partial<Settings>>({});
const isModalVisible = ref(false);
const editingProvider = ref<ApiProvider | null>(null);

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.apiConfig = JSON.parse(JSON.stringify(newSettings.apiConfig || { providers: [], assignments: {}, keys: {} }));
    editableSettings.knowledgeBase = JSON.parse(JSON.stringify(newSettings.knowledgeBase || {}));
  }
}, { immediate: true, deep: true });

const getModelOptionsFor = (capability: ModelCapability) => {
  const options: { value: string, label: string }[] = [];
  settingsStore.settings?.apiConfig?.providers.forEach(p => {
    p.models.forEach(m => {
      if (m.capabilities.includes(capability)) {
        options.push({
          value: `${p.id}::${m.name}`,
          label: `${p.name} - ${m.name}`
        });
      }
    });
  });
  return options;
};

const createIdentifier = (endpoint: ModelEndpoint | null | undefined) => {
  return endpoint ? `${endpoint.providerId}::${endpoint.modelName}` : null;
};

const parseIdentifier = (identifier: string | null): ModelEndpoint | null => {
  if (!identifier) return null;
  const [providerId, modelName] = identifier.split('::');
  return { providerId, modelName };
};

const createAssignmentComputed = (key: AssignmentKey) => {
  return computed({
    get: () => createIdentifier(editableSettings.apiConfig?.assignments[key]),
    set: (value) => {
      if (editableSettings.apiConfig) {
        editableSettings.apiConfig.assignments[key] = parseIdentifier(value);
      }
    }
  });
};

const mainAssignments = [
  { key: 'chat', label: 'settings.apiKeys.assignments_chat', capability: 'chat', model: createAssignmentComputed('chat') },
  { key: 'suggestion', label: 'settings.apiKeys.assignments_suggestion', capability: 'chat', model: createAssignmentComputed('suggestion') },
  { key: 'vision', label: 'settings.apiKeys.assignments_vision', capability: 'vision', model: createAssignmentComputed('vision') },
  { key: 'embedding', label: 'settings.apiKeys.assignments_embedding', capability: 'embedding', model: createAssignmentComputed('embedding') },
  { key: 'plan', label: 'settings.apiKeys.assignments_plan', capability: 'chat', model: createAssignmentComputed('plan') },
  { key: 'write', label: 'settings.apiKeys.assignments_write', capability: 'chat', model: createAssignmentComputed('write') },
  { key: 'refine', label: 'settings.apiKeys.assignments_refine', capability: 'chat', model: createAssignmentComputed('refine') },
];

const otherAssignments = [
  { key: 'imageGen', label: 'settings.apiKeys.assignments_imageGen', capability: 'image_gen', model: createAssignmentComputed('imageGen') },
  { key: 'videoGen', label: 'settings.apiKeys.assignments_videoGen', capability: 'video_gen', model: createAssignmentComputed('videoGen') },
  { key: 'tts', label: 'settings.apiKeys.assignments_tts', capability: 'tts', model: createAssignmentComputed('tts') },
  { key: 'debatePro', label: 'settings.apiKeys.assignments_debatePro', capability: 'chat', model: createAssignmentComputed('debatePro') },
  { key: 'debateCon', label: 'settings.apiKeys.assignments_debateCon', capability: 'chat', model: createAssignmentComputed('debateCon') },
  { key: 'debateJudge', label: 'settings.apiKeys.assignments_debateJudge', capability: 'chat', model: createAssignmentComputed('debateJudge') },
];

const openProviderModal = (provider: ApiProvider | null) => {
  editingProvider.value = provider;
  isModalVisible.value = true;
};

const deleteProvider = (providerId: string) => {
  if (confirm('Are you sure you want to delete this API provider?')) {
    settingsStore.deleteApiProvider(providerId);
  }
};

const saveBasicSettings = () => {
  if (settingsStore.settings && editableSettings.apiConfig) {
    const newSettings = JSON.parse(JSON.stringify(settingsStore.settings)) as Settings;
    
    newSettings.apiConfig.assignments = editableSettings.apiConfig.assignments;
    newSettings.apiConfig.keys = editableSettings.apiConfig.keys;

    settingsStore.saveSettings(newSettings);
    success(t('settings.apiKeys.saveSuccess'));
  }
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
</style>