<!-- frontend/src/components/settings/SettingsConverter.vue -->
<template>
  <div>
    <h2 class="text-lg font-semibold">Batch Convert to Markdown</h2>
    <p class="text-sm text-gray-500 mt-1">Convert a folder of documents (PDF, DOCX, PPTX, etc.) to Markdown files.</p>
    <div class="mt-6 p-4 border border-gray-200 dark:border-gray-700 rounded-lg space-y-4">
        <div>
            <label class="block text-sm font-medium">Input Folder</label>
            <div class="flex items-center space-x-2 mt-1">
                <input type="text" readonly :value="converter.inputDir" class="input-style flex-1 bg-gray-100 dark:bg-gray-800" placeholder="Select a source folder...">
                <button @click="converter.selectInputDirectory()" class="btn-secondary">Choose...</button>
            </div>
        </div>
        <div>
            <label class="block text-sm font-medium">Output Folder</label>
            <div class="flex items-center space-x-2 mt-1">
                <input type="text" readonly :value="converter.outputDir" class="input-style flex-1 bg-gray-100 dark:bg-gray-800" placeholder="Select a destination folder...">
                <button @click="converter.selectOutputDirectory()" class="btn-secondary">Choose...</button>
            </div>
        </div>
        <div class="pt-2">
            <button @click="converter.startConversion()" :disabled="!converter.inputDir || !converter.outputDir || converter.isConverting" class="btn-primary w-full justify-center">
                <Loader2 v-if="converter.isConverting" class="w-4 h-4 mr-2 animate-spin" />
                Start Conversion
            </button>
        </div>
        <div v-if="converter.isConverting || converter.progress > 0" class="pt-2">
            <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                <div class="bg-blue-600 h-2.5 rounded-full" :style="{ width: `${converter.progress}%` }"></div>
            </div>
            <p class="text-xs text-center mt-1 font-mono">{{ converter.progressMessage }}</p>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useConverterStore } from '../../stores/converter';
import { Loader2 } from 'lucide-vue-next';

const converter = useConverterStore();
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
.btn-primary {
    @apply px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 flex items-center;
}
.btn-secondary {
    @apply px-4 py-2 text-sm bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500;
}
</style>