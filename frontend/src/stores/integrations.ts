import { defineStore } from 'pinia';
import { ref } from 'vue';
import { listIntegrations, addIntegration, deleteIntegration, getIntegrationTemplates } from '../lib/api';
import type { Integration, IntegrationTemplate } from '../types';
import { v4 as uuidv4 } from 'uuid';

export const useIntegrationsStore = defineStore('integrations', () => {
  const integrations = ref<Integration[]>([]);
  const templates = ref<IntegrationTemplate[]>([]);
  const isLoading = ref(false);

  async function fetchIntegrations() {
    isLoading.value = true;
    const result = await listIntegrations();
    if (result) {
      integrations.value = result;
    }
    isLoading.value = false;
  }

  async function fetchTemplates() {
    const result = await getIntegrationTemplates();
    if (result) {
      templates.value = result;
    }
  }

  async function addIntegrationFromTemplate(name: string, template: IntegrationTemplate) {
    const newIntegration: Integration = {
      id: uuidv4(),
      name,
      service: template.id, // Use template ID as the service identifier
      configJson: JSON.stringify({}), // Start with empty config
      isEnabled: true,
      createdAt: Date.now(),
    };

    const result = await addIntegration(newIntegration);
    if (result) {
      integrations.value.unshift(result);
    }
  }

  async function deleteIntegration(id: string) {
    await deleteIntegration(id);
    integrations.value = integrations.value.filter(i => i.id !== id);
  }

  return {
    integrations,
    templates,
    isLoading,
    fetchIntegrations,
    fetchTemplates,
    addIntegrationFromTemplate,
    deleteIntegration,
  };
});