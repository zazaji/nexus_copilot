<!-- frontend/src/components/settings/SettingsBackupRestore.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">{{ $t('settings.backup.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.backup.description') }}</p>
    
    <div class="mt-6 space-y-6">
      <!-- Export Section -->
      <div class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
        <h3 class="font-semibold">{{ $t('settings.backup.exportTitle') }}</h3>
        <p class="text-sm text-gray-500 mt-1 mb-4">
          {{ $t('settings.backup.exportDesc') }}
        </p>
        <button @click="handleExport" :disabled="isProcessing" class="flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400">
          <Loader2 v-if="isProcessing" class="w-4 h-4 mr-2 animate-spin" />
          <Download v-else class="w-4 h-4 mr-2" />
          {{ $t('settings.backup.exportBtn') }}
        </button>
      </div>

      <!-- Import Section -->
      <div class="p-4 border border-red-300 dark:border-red-700 rounded-lg">
        <h3 class="font-semibold text-red-600 dark:text-red-400">{{ $t('settings.backup.restoreTitle') }}</h3>
        <p class="text-sm text-gray-500 mt-1 mb-4">
          <span class="font-bold text-red-500">Warning:</span> {{ $t('settings.backup.restoreDesc') }}
        </p>
        <button @click="handleImport" :disabled="isProcessing" class="flex items-center px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:bg-gray-400">
          <Loader2 v-if="isProcessing" class="w-4 h-4 mr-2 animate-spin" />
          <Upload v-else class="w-4 h-4 mr-2" />
          {{ $t('settings.backup.restoreBtn') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { save as saveDialog, open as openDialog } from '@tauri-apps/api/dialog';
import { exportData, importData } from '../../lib/api';
import { useToasts } from '../../composables/useToasts';
import { Download, Upload, Loader2 } from 'lucide-vue-next';

const { t } = useI18n();
const { success, error } = useToasts();
const isProcessing = ref(false);

const handleExport = async () => {
  isProcessing.value = true;
  try {
    const filePath = await saveDialog({
      title: 'Export Nexus Data',
      defaultPath: `nexus_backup_${new Date().toISOString().split('T')[0]}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });

    if (filePath) {
      const result = await exportData(filePath);
      if (result === null) {
        success(t('settings.backup.exportSuccess'));
      }
    }
  } catch (e) {
    error(t('settings.backup.exportError'));
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};

const handleImport = async () => {
  const confirmed = confirm(t('settings.backup.confirmRestore'));

  if (!confirmed) return;

  isProcessing.value = true;
  try {
    const selectedPath = await openDialog({
      title: 'Restore Nexus Data',
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });

    if (typeof selectedPath === 'string' && selectedPath) {
      const result = await importData(selectedPath);
      if (result === null) {
        success(t('settings.backup.restoreSuccess'));
      }
    }
  } catch (e) {
    error(t('settings.backup.restoreError'));
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};
</script>