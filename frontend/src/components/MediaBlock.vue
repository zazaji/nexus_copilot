<!-- frontend/src/components/MediaBlock.vue -->
<template>
  <div class="media-block my-2">
    <div 
      @click="openPreview" 
      class="relative max-w-96 h-48 bg-gray-100 dark:bg-gray-700 rounded-lg overflow-hidden cursor-pointer group flex items-center justify-center"
    >
      <img v-if="mediaType === 'image' && displayUrl" :src="displayUrl" class="w-full h-auto object-cover" alt="Image content" />
      <video v-else-if="mediaType === 'video' && displayUrl" :src="displayUrl" class="w-full h-full object-cover" controls></video>
      <audio v-else-if="mediaType === 'audio' && displayUrl" :src="displayUrl" class="w-full absolute bottom-0 left-0" controls></audio>
      <div v-else class="flex items-center justify-center h-full text-gray-500">
        <File class="w-12 h-12" />
      </div>
      <div class="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-30 transition-all flex items-center justify-center">
        <ZoomIn class="w-10 h-10 text-white opacity-0 group-hover:opacity-80 transition-opacity" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watchEffect } from 'vue';
import { useFilePreviewStore } from '../stores/filePreview';
import { File, ZoomIn } from 'lucide-vue-next';
import { readFileAsBase64 } from '../lib/api';

const props = defineProps<{
  url: string;
}>();

const previewStore = useFilePreviewStore();
const displayUrl = ref('');

const isLocalPath = computed(() => {
    const url = props.url;
    return !url.startsWith('http') && !url.startsWith('data:') && !url.startsWith('asset:');
});

const mediaType = computed(() => {
  const url = props.url;

  // Case 1: Handle Data URLs
  if (url.startsWith('data:image/')) return 'image';
  if (url.startsWith('data:video/')) return 'video';
  if (url.startsWith('data:audio/')) return 'audio';

  // Case 2: Handle standard HTTP/HTTPS URLs
  if (url.startsWith('http://') || url.startsWith('https://')) {
    try {
      const path = new URL(url).pathname;
      const extension = path.split('.').pop()?.toLowerCase() || '';
      if (['png', 'jpg', 'jpeg', 'webp', 'gif'].includes(extension)) return 'image';
      if (['mp4', 'webm', 'ogg'].includes(extension)) return 'video';
      if (['mp3', 'wav', 'oga'].includes(extension)) return 'audio';
    } catch (e) {
      return 'unknown';
    }
  }
  
  // Case 3: Handle local file paths by extension
  const extension = url.split('.').pop()?.toLowerCase() || '';
  if (['png', 'jpg', 'jpeg', 'webp', 'gif'].includes(extension)) return 'image';
  if (['mp4', 'webm', 'ogg'].includes(extension)) return 'video';
  if (['mp3', 'wav', 'oga'].includes(extension)) return 'audio';

  return 'unknown';
});

watchEffect(async () => {
  const url = props.url;
  if (isLocalPath.value) {
    const base64Data = await readFileAsBase64(url);
    if (base64Data) {
      const extension = url.split('.').pop()?.toLowerCase() || '';
      let mimeType = 'application/octet-stream';
      
      if (mediaType.value === 'image') mimeType = `image/${extension === 'jpg' ? 'jpeg' : extension}`;
      else if (mediaType.value === 'audio') mimeType = `audio/${extension}`;
      else if (mediaType.value === 'video') mimeType = `video/${extension}`;
      
      displayUrl.value = `data:${mimeType};base64,${base64Data}`;
    } else {
      displayUrl.value = ''; // or a placeholder image
    }
  } else {
    displayUrl.value = url;
  }
});

const openPreview = () => {
  if ((mediaType.value === 'image' || mediaType.value === 'video') && displayUrl.value) {
    // Pass the already converted (if necessary) displayUrl to the preview store
    previewStore.showPreview(displayUrl.value);
  }
};
</script>