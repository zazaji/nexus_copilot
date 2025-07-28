<template>
  <div>
    <h2 class="text-lg font-semibold">{{ $t('settings.integrations.title') }}</h2>
    <p class="text-sm text-gray-500 mt-1">{{ $t('settings.integrations.description') }}</p>

    <div class="mt-6 space-y-4">
      <div class="flex justify-between items-center">
        <h3 class="font-semibold">{{ $t('settings.integrations.configured') }}</h3>
        <button @click="isModalVisible = true" class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm flex items-center space-x-2">
          <Plus class="w-4 h-4" />
          <span>{{ $t('settings.integrations.add') }}</span>
        </button>
      </div>
      <div class="border border-gray-200 dark:border-gray-700 rounded-lg">
        <div v-if="integrationsStore.integrations.length === 0" class="p-4 text-center text-sm text-gray-500">
          {{ $t('settings.integrations.noIntegrations') }}
        </div>
        <ul v-else class="divide-y divide-gray-200 dark:divide-gray-700">
          <li v-for="integration in integrationsStore.integrations" :key="integration.id" class="p-4 flex justify-between items-center group">
            <div>
              <p class="font-semibold">{{ integration.name }}</p>
              <p class="text-sm text-gray-500 font-mono flex items-center space-x-2">
                <span>{{ getWebhookUrl(integration.id) }}</span>
                <button @click="copyToClipboard(getWebhookUrl(integration.id))" class="p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 opacity-0 group-hover:opacity-100 transition-opacity">
                  <Copy class="w-3 h-3" />
                </button>
              </p>
            </div>
            <div class="flex items-center space-x-2">
              <button @click="integrationsStore.deleteIntegration(integration.id)" class="p-2 text-gray-500 hover:text-red-600"><Trash2 class="w-4 h-4" /></button>
            </div>
          </li>
        </ul>
      </div>
    </div>

    <!-- Add Integration Modal -->
    <n-modal v-model:show="isModalVisible" preset="card" title="Add New Integration" class="max-w-lg">
      <div class="space-y-4">
        <p>Select a template to create a new integration.</p>
        <div v-for="template in integrationsStore.templates" :key="template.id"
             @click="selectTemplate(template)"
             class="p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 cursor-pointer">
          <h4 class="font-semibold">{{ template.name }}</h4>
          <p class="text-sm text-gray-500">{{ template.description }}</p>
        </div>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useIntegrationsStore } from '../../stores/integrations';
import { useSettingsStore } from '../../stores/settings';
import { useToasts } from '../../composables/useToasts';
import { Plus, Trash2, Copy } from 'lucide-vue-next';
import { NModal } from 'naive-ui';
import type { IntegrationTemplate } from '../../types';

const integrationsStore = useIntegrationsStore();
const settingsStore = useSettingsStore();
const { success } = useToasts();
const isModalVisible = ref(false);

onMounted(() => {
  integrationsStore.fetchIntegrations();
  integrationsStore.fetchTemplates();
});

const getWebhookUrl = (integrationId: string): string => {
  const backendUrl = settingsStore.settings?.execution.backendUrl || 'http://127.0.0.1:8008';
  return `${backendUrl}/api/v1/integrations/webhooks/${integrationId}`;
};

const selectTemplate = (template: IntegrationTemplate) => {
  const name = prompt(`Enter a name for this integration (e.g., 'Notes from ${template.name}'):`);
  if (name) {
    integrationsStore.addIntegrationFromTemplate(name, template);
    isModalVisible.value = false;
  }
};

const copyToClipboard = async (text: string) => {
  await navigator.clipboard.writeText(text);
  success('Webhook URL copied to clipboard!');
};
</script>