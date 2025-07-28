<!-- frontend/src/components/settings/SettingsAbout.vue -->
<template>
  <div>
    <div class="flex flex-col items-center text-center">
      <div class="p-4 bg-gray-100 dark:bg-gray-700 rounded-full mb-4">
        <Sparkles class="w-16 h-16 text-blue-500" />
      </div>
      <h2 class="text-2xl font-bold">{{ appInfo?.name || 'Nexus' }}</h2>
      <p class="text-lg text-gray-500 dark:text-gray-400">Version {{ appInfo?.version || 'N/A' }}</p>
      <p class="mt-4 max-w-md">
        Nexus is a local-first AI native application designed to augment your workflow and knowledge management.
      </p>
      <div class="mt-6 space-x-4">
        <a href="https://github.com/zazaji/nexus_copilot" target="_blank" class="text-blue-500 hover:underline">GitHub Repository</a>
        <a href="https://github.com/zazaji/nexus_copilot/issues" target="_blank" class="text-blue-500 hover:underline">Report an Issue</a>
      </div>
      <p class="mt-8 text-xs text-gray-400">© {{ new Date().getFullYear() }} Ouyang.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getAppInfo } from '../../lib/api';
import { Sparkles } from 'lucide-vue-next';

interface AppInfo {
  name: string;
  version: string;
}

const appInfo = ref<AppInfo | null>(null);

onMounted(async () => {
  appInfo.value = await getAppInfo();
});
</script>