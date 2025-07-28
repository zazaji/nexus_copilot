<template>
  <div class="flex-1 flex flex-col p-6 bg-gray-50 dark:bg-gray-900/50 overflow-hidden">
    <h1 class="text-2xl font-bold mb-4">{{ $t('settings.title') }}</h1>
    <div class="flex-1 flex bg-white dark:bg-gray-800 rounded-lg shadow-sm overflow-hidden">
      <aside class="w-56 border-r border-gray-200 dark:border-gray-700 p-4">
        <nav class="space-y-1">
          <button
            v-for="tab in tabs"
            :key="tab.name"
            @click="activeTab = tab.component"
            :aria-label="tab.name"
            :class="[
              'w-full flex items-center space-x-3 px-3 py-2 text-sm font-medium rounded-md text-left',
              activeTab === tab.component
                ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300'
                : 'text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-700/50',
            ]"
          >
            <component :is="tab.icon" class="w-5 h-5" />
            <span>{{ $t(tab.label) }}</span>
          </button>
        </nav>
      </aside>
      <main class="flex-1 p-6 overflow-y-auto">
        <div v-if="settingsStore.isLoading" class="flex items-center justify-center h-full">
          <Loader2 class="w-8 h-8 animate-spin text-blue-500" />
        </div>
        <div v-else-if="settingsStore.settings">
          <component :is="activeTab" />
        </div>
        <div v-else class="text-center text-gray-500">
          Could not load settings. Please try restarting the application.
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { shallowRef } from 'vue';
import { useSettingsStore } from '../stores/settings';
import { KeyRound, Database, Code, Loader2, Archive, TerminalSquare, Info, Settings, ArrowRightLeft } from 'lucide-vue-next';
import SettingsGeneral from '../components/settings/SettingsGeneral.vue';
import SettingsApiKeys from '../components/settings/SettingsApiKeys.vue';
import SettingsKnowledgeBase from '../components/settings/SettingsKnowledgeBase.vue';
import SettingsScripts from '../components/settings/SettingsScripts.vue';
import SettingsBackupRestore from '../components/settings/SettingsBackupRestore.vue';
import SettingsExecutionEnv from '../components/settings/SettingsExecutionEnv.vue';
import SettingsAbout from '../components/settings/SettingsAbout.vue';
import SettingsConverter from '../components/settings/SettingsConverter.vue';

const settingsStore = useSettingsStore();

const tabs = [
  { name: 'General', label: 'settings.tabs.general', icon: Settings, component: SettingsGeneral },
  { name: 'API & Models', label: 'settings.tabs.apiKeys', icon: KeyRound, component: SettingsApiKeys },
  { name: 'Knowledge Base', label: 'settings.tabs.knowledgeBase', icon: Database, component: SettingsKnowledgeBase },
  { name: 'Actions & Integrations', label: 'settings.tabs.scriptsAndTools', icon: Code, component: SettingsScripts },
  { name: 'Execution', label: 'settings.tabs.execution', icon: TerminalSquare, component: SettingsExecutionEnv },
  { name: 'Converter', label: 'settings.tabs.converter', icon: ArrowRightLeft, component: SettingsConverter },
  { name: 'Backup & Restore', label: 'settings.tabs.backupRestore', icon: Archive, component: SettingsBackupRestore },
  { name: 'About', label: 'settings.tabs.about', icon: Info, component: SettingsAbout },
];
const activeTab = shallowRef(tabs[0].component);
</script>