<!-- frontend/src/components/FilePreviewModal.vue -->
<template>
  <transition name="modal-fade">
    <div v-if="previewStore.isVisible" class="fixed inset-0 bg-black bg-opacity-60 z-50 flex items-center justify-center" @click.self="previewStore.hidePreview()">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-4xl h-[90vh] m-4 flex flex-col">
        <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
          <p class="font-mono text-sm truncate" :title="previewStore.filePath || ''">
            {{ (previewStore.filePath || '').split(/[/\\]/).pop() }}
          </p>
          <div class="flex items-center space-x-2">
            <n-tooltip v-if="!previewStore.isImage" trigger="hover">
              <template #trigger>
                <button 
                  @click="handleSaveToKb" 
                  class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700" 
                  title="Save to Knowledge Base"
                >
                  <Save class="w-5 h-5" />
                </button>
              </template>
              <span>Save to Knowledge Base</span>
            </n-tooltip>
            <n-tooltip v-if="previewStore.isImage" trigger="hover">
              <template #trigger>
                <button 
                  @click="handleCopyImage" 
                  :disabled="isHttpUrl"
                  class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed" 
                  title="Copy Image"
                >
                  <Copy class="w-5 h-5" />
                </button>
              </template>
              <span v-if="isHttpUrl">Cannot copy remote image directly. Please right-click and 'Copy Image'.</span>
              <span v-else>Copy Image</span>
            </n-tooltip>
            <button v-if="previewStore.isImage" @click="handleCopyUrl" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700" title="Copy URL">
              <Link class="w-5 h-5" />
            </button>
            <button v-if="previewStore.sourceUrl" @click="openInBrowser" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700" title="Open in Browser">
              <ExternalLink class="w-5 h-5" />
            </button>
            <button @click="previewStore.hidePreview()" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700">
              <X class="w-5 h-5" />
            </button>
          </div>
        </header>
        <main class="flex-1 overflow-y-auto p-4">
          <div v-if="previewStore.isLoading" class="flex items-center justify-center h-full">
            <Loader2 class="w-8 h-8 animate-spin text-blue-500" />
          </div>
          <div v-else-if="previewStore.error" class="p-8 text-red-500">
            {{ previewStore.error }}
          </div>
          <div v-else-if="previewStore.isImage" class="flex items-center justify-center h-full">
            <img :src="previewStore.fileContent || ''" class="max-w-full max-h-full object-contain" />
          </div>
          <div v-else-if="previewStore.isPdf" class="h-full">
            <vue-pdf-embed :source="previewStore.fileContent || ''" class="h-full w-full" />
          </div>
          <MdPreview v-else-if="previewStore.isMarkdown" :modelValue="previewStore.fileContent || ''" theme="dark" />
          <pre v-else-if="previewStore.isText" class="whitespace-pre-wrap font-mono text-sm">
            {{ previewStore.fileContent }}
          </pre>
        </main>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useFilePreviewStore } from '../stores/filePreview';
import { useSettingsStore } from '../stores/settings';
import { useKnowledgeExplorerStore } from '../stores/knowledgeExplorer';
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import VuePdfEmbed from 'vue-pdf-embed';
import { X, Loader2, ExternalLink, Copy, Link, Save } from 'lucide-vue-next';
import { openExternalLink, readFileAsBase64, saveNoteToKb } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import { NTooltip } from 'naive-ui';

const previewStore = useFilePreviewStore();
const settingsStore = useSettingsStore();
const explorerStore = useKnowledgeExplorerStore();
const { success, error } = useToasts();

const isHttpUrl = computed(() => previewStore.filePath?.startsWith('http'));

const openInBrowser = () => {
  if (previewStore.sourceUrl) {
    openExternalLink(previewStore.sourceUrl);
  }
};

const handleCopyUrl = async () => {
  if (!previewStore.filePath) {
    error('No URL to copy.');
    return;
  }
  try {
    await navigator.clipboard.writeText(previewStore.filePath);
    success('URL copied to clipboard!');
  } catch (err) {
    console.error('Failed to copy URL:', err);
    error('Failed to copy URL.');
  }
};

const b64toBlob = (b64Data: string, contentType = '', sliceSize = 512) => {
  const byteCharacters = atob(b64Data);
  const byteArrays = [];
  for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
    const slice = byteCharacters.slice(offset, offset + sliceSize);
    const byteNumbers = new Array(slice.length);
    for (let i = 0; i < slice.length; i++) {
      byteNumbers[i] = slice.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    byteArrays.push(byteArray);
  }
  return new Blob(byteArrays, { type: contentType });
};

const handleCopyImage = async () => {
  const pathOrUrl = previewStore.filePath;
  const contentUrl = previewStore.fileContent;

  if (!pathOrUrl || !contentUrl || isHttpUrl.value) {
    error('Image source not available for direct copy.');
    return;
  }

  try {
    let imageBlob: Blob;

    if (contentUrl.startsWith('data:')) {
      const response = await fetch(contentUrl);
      imageBlob = await response.blob();
    } else {
      const base64Data = await readFileAsBase64(pathOrUrl);
      if (!base64Data) {
        throw new Error('Failed to read local image file.');
      }
      const extension = pathOrUrl.split('.').pop()?.toLowerCase() || 'png';
      const mimeType = `image/${extension === 'jpg' ? 'jpeg' : extension}`;
      imageBlob = b64toBlob(base64Data, mimeType);
    }

    await navigator.clipboard.write([
      new ClipboardItem({
        [imageBlob.type]: imageBlob,
      }),
    ]);
    success('Image copied to clipboard!');
  } catch (err) {
    console.error('Failed to copy image:', err);
    error('Failed to copy image.');
  }
};

const handleSaveToKb = async () => {
  const content = previewStore.fileContent;
  if (!content) {
    error('No content to save.');
    return;
  }

  const saveDir = settingsStore.settings?.knowledgeBase.defaultSaveDirectory;
  if (!saveDir) {
    error('No default knowledge base directory set in Settings.');
    return;
  }

  const baseFilename = content.substring(0, 16).replace(/[<>:"/\\|?*]/g, '').trim() + '.md';

  const finalPath = await saveNoteToKb(saveDir, baseFilename, content);
  if (finalPath) {
    const finalFilename = finalPath.split(/[/\\]/).pop();
    success(`Report saved to knowledge base as "${finalFilename}"`);
    explorerStore.loadFileTree();
  }
};
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>