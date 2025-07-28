// frontend/src/stores/backendManager.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useSettingsStore } from './settings';
import { 
    checkBackendStatus, 
    installBackendService, 
    startBackendService, 
    stopBackendService,
    onBackendInstallProgress 
} from '../lib/api';
import type { UnlistenFn } from '@tauri-apps/api/event';

export type BackendStatus = 'unknown' | 'not_installed' | 'installing' | 'stopped' | 'running' | 'error';

export const useBackendManagerStore = defineStore('backendManager', () => {
    const status = ref<BackendStatus>('unknown');
    const isBusy = ref(false);
    const progress = ref(0);
    const progressMessage = ref('');
    const settingsStore = useSettingsStore();
    let unlistenProgress: UnlistenFn | null = null;

    async function listenToProgress() {
        if (unlistenProgress) unlistenProgress();
        unlistenProgress = await onBackendInstallProgress((payload) => {
            progress.value = payload.progress;
            progressMessage.value = payload.message;
        });
    }

    async function checkStatus() {
        if (!settingsStore.settings) await settingsStore.fetchSettings();
        const pythonPath = settingsStore.settings?.execution.pythonPath || 'python';
        const result = await checkBackendStatus(pythonPath);
        if (result) {
            status.value = result.status;
        }
    }

    async function installBackend() {
        isBusy.value = true;
        status.value = 'installing';
        progress.value = 0;
        progressMessage.value = 'Starting installation...';
        await listenToProgress();
        await installBackendService();
        isBusy.value = false;
        await checkStatus();
        if (unlistenProgress) {
            unlistenProgress();
            unlistenProgress = null;
        }
    }

    async function startBackend() {
        isBusy.value = true;
        const pythonPath = settingsStore.settings?.execution.pythonPath || 'python';
        await startBackendService(pythonPath);
        isBusy.value = false;
        await checkStatus();
    }

    async function stopBackend() {
        isBusy.value = true;
        await stopBackendService();
        isBusy.value = false;
        await checkStatus();
    }

    return {
        status,
        isBusy,
        progress,
        progressMessage,
        checkStatus,
        installBackend,
        startBackend,
        stopBackend,
    };
});