<!-- frontend/src/components/settings/SettingsExecutionEnv.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">{{ $t('settings.execution.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.execution.description') }}</p>
    
    <div v-if="editableSettings.execution" class="mt-6 space-y-6">
      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">{{ $t('settings.execution.pythonConfig') }}</h3>
        <div class="mt-4 space-y-4">
          <div>
            <label for="python-path" class="block text-sm font-medium">{{ $t('settings.execution.pythonPath') }}</label>
            <input 
              type="text" 
              id="python-path" 
              v-model="editableSettings.execution.pythonPath" 
              class="mt-1 block w-full max-w-md input-style"
              placeholder="e.g., /usr/bin/python3 or C:\Python39\python.exe"
            />
            <p class="text-xs text-gray-500 mt-1">{{ $t('settings.execution.pythonPathDesc') }}</p>
          </div>
           <div>
            <label for="node-path" class="block text-sm font-medium">Node.js Interpreter Path</label>
            <input 
              type="text" 
              id="node-path" 
              v-model="editableSettings.execution.nodePath" 
              class="mt-1 block w-full max-w-md input-style"
              placeholder="e.g., /usr/local/bin/node or C:\Program Files\nodejs\node.exe"
            />
          </div>
          <div>
            <label for="working-dir" class="block text-sm font-medium">{{ $t('settings.execution.workingDir') }}</label>
            <input 
              type="text" 
              id="working-dir" 
              v-model="editableSettings.execution.workingDirectory" 
              class="mt-1 block w-full max-w-md input-style"
              placeholder="e.g., /Users/yourname/Documents/NexusScripts"
            />
            <p class="text-xs text-gray-500 mt-1">{{ $t('settings.execution.workingDirDesc') }}</p>
          </div>
        </div>
      </div>

      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">{{ $t('settings.execution.backendManagement') }}</h3>
        <div class="mt-4 space-y-4">
            <div>
                <label for="backend-url" class="block text-sm font-medium">{{ $t('settings.execution.backendUrl') }}</label>
                <input type="text" id="backend-url" v-model="editableSettings.execution.backendUrl" class="mt-1 block w-full max-w-md input-style" placeholder="e.g., http://127.0.0.1:8008" />
            </div>
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                    <span class="font-medium">{{ $t('settings.execution.status') }}</span>
                    <span class="px-2 py-1 text-xs rounded-full" :class="backendStatusClass">{{ backendManager.status }}</span>
                </div>
                <div class="flex items-center space-x-2">
                    <button v-if="backendManager.status === 'not_installed'" @click="backendManager.installBackend()" :disabled="backendManager.isBusy" class="btn-primary">
                        <Loader2 v-if="backendManager.isBusy" class="w-4 h-4 animate-spin mr-2"/>
                        {{ $t('settings.execution.installBackend') }}
                    </button>
                    <button v-if="backendManager.status === 'stopped'" @click="backendManager.startBackend()" :disabled="backendManager.isBusy" class="btn-success">
                        <Loader2 v-if="backendManager.isBusy" class="w-4 h-4 animate-spin mr-2"/>
                        {{ $t('settings.execution.startBackend') }}
                    </button>
                    <button v-if="backendManager.status === 'running'" @click="backendManager.stopBackend()" :disabled="backendManager.isBusy" class="btn-danger">
                        <Loader2 v-if="backendManager.isBusy" class="w-4 h-4 animate-spin mr-2"/>
                        {{ $t('settings.execution.stopBackend') }}
                    </button>
                </div>
            </div>
            <div v-if="backendManager.isBusy" class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                <div class="bg-blue-600 h-2.5 rounded-full" :style="{ width: `${backendManager.progress}%` }"></div>
                <p class="text-xs text-center mt-1">{{ backendManager.progressMessage }}</p>
            </div>
            <div class="flex items-center space-x-3">
                <label for="auto-start" class="text-sm font-medium">{{ $t('settings.execution.autoStart') }}</label>
                <input type="checkbox" id="auto-start" v-model="editableSettings.execution.autoStartBackend" class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500">
            </div>
        </div>
      </div>

      <div class="pt-6">
        <button @click="save" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">{{ $t('settings.execution.save') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, onMounted, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '../../stores/settings';
import { useBackendManagerStore } from '../../stores/backendManager';
import { useToasts } from '../../composables/useToasts';
import type { Settings } from '../../types';
import { Loader2 } from 'lucide-vue-next';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const backendManager = useBackendManagerStore();
const { success } = useToasts();

const editableSettings = reactive<Partial<Settings>>({});

const backendStatusClass = computed(() => {
    switch(backendManager.status) {
        case 'running': return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
        case 'stopped': return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
        case 'installing': return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
        case 'error': return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
        default: return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
    }
});

watch(() => settingsStore.settings, (newSettings) => {
  if (newSettings) {
    editableSettings.execution = JSON.parse(JSON.stringify(newSettings.execution || {}));
  }
}, { immediate: true, deep: true });

onMounted(() => {
    backendManager.checkStatus();
});

const save = () => {
  if (settingsStore.settings && editableSettings.execution) {
    const newSettingsToSave = JSON.parse(JSON.stringify(settingsStore.settings));
    newSettingsToSave.execution = editableSettings.execution;
    settingsStore.saveSettings(newSettingsToSave);
    success(t('settings.execution.saveSuccess'));
  }
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
.btn-primary {
    @apply px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 flex items-center;
}
.btn-success {
    @apply px-4 py-2 text-sm bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:bg-gray-400 flex items-center;
}
.btn-danger {
    @apply px-4 py-2 text-sm bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:bg-gray-400 flex items-center;
}
</style>