// frontend/src/stores/settings.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Settings, ApiProvider } from '../types';
import { getUserSettings, updateUserSettings } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import { v4 as uuidv4 } from 'uuid';
import { useKnowledgeBaseStore } from './knowledgeBase';

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings | null>(null);
  const isLoading = ref(false);
  const { error, success } = useToasts();

  async function fetchSettings() {
    if (settings.value) return;
    isLoading.value = true;
    try {
      const result = await getUserSettings();
      if (result) {
        const kbStore = useKnowledgeBaseStore();
        await kbStore.fetchOnlineKbs();
        result.apiConfig.onlineKbs = kbStore.onlineKbs;
        
        settings.value = result;
      } else {
        error("Could not load settings from backend.");
      }
    } finally {
      isLoading.value = false;
    }
  }

  async function saveSettings(newSettings: Settings) {
    isLoading.value = true;
    try {
      const settingsToSave = JSON.parse(JSON.stringify(newSettings));
      if (settingsToSave.apiConfig.onlineKbs) {
        delete settingsToSave.apiConfig.onlineKbs;
      }

      await updateUserSettings(settingsToSave);
      
      // Directly update the store's state with the successfully saved settings.
      // This avoids the null state and ensures smooth reactivity.
      const kbStore = useKnowledgeBaseStore();
      await kbStore.fetchOnlineKbs(); // Ensure online KBs are fresh
      newSettings.apiConfig.onlineKbs = kbStore.onlineKbs;
      settings.value = newSettings;
      
      success("Settings saved successfully.");
    } catch (e) {
      error("Failed to save settings.");
    }
    finally {
      isLoading.value = false;
    }
  }

  async function saveApiProvider(providerToSave: ApiProvider) {
    if (!settings.value) {
      error("Settings not loaded.");
      return;
    }
    const newSettings = JSON.parse(JSON.stringify(settings.value)) as Settings;
    const index = newSettings.apiConfig.providers.findIndex(p => p.id === providerToSave.id);

    if (index > -1) {
      newSettings.apiConfig.providers[index] = providerToSave;
    } else {
      newSettings.apiConfig.providers.push({ ...providerToSave, id: uuidv4() });
    }
    
    await saveSettings(newSettings);
  }

  async function deleteApiProvider(providerId: string) {
    if (!settings.value) {
      error("Settings not loaded.");
      return;
    }
    const newSettings = JSON.parse(JSON.stringify(settings.value)) as Settings;
    const providerName = newSettings.apiConfig.providers.find(p => p.id === providerId)?.name || 'Provider';
    newSettings.apiConfig.providers = newSettings.apiConfig.providers.filter(p => p.id !== providerId);

    // Also clean up any model assignments that used this provider
    Object.keys(newSettings.apiConfig.assignments).forEach(key => {
        const assignmentKey = key as keyof typeof newSettings.apiConfig.assignments;
        const assignment = newSettings.apiConfig.assignments[assignmentKey];
        if (assignment && assignment.providerId === providerId) {
            newSettings.apiConfig.assignments[assignmentKey] = null;
        }
    });

    await saveSettings(newSettings);
    success(`API Provider "${providerName}" deleted.`);
  }

  return { settings, isLoading, fetchSettings, saveSettings, saveApiProvider, deleteApiProvider };
});