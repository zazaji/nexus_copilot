<!-- frontend/src/components/AttachmentPill.vue -->
<template>
  <button 
    @click="openPreview"
    class="flex items-center space-x-1.5 px-2.5 py-1 bg-gray-100 dark:bg-gray-700/50 hover:bg-gray-200 dark:hover:bg-gray-600/50 rounded-full text-xs transition-colors text-left"
    :title="attachment.url"
  >
    <Paperclip class="w-3 h-3 text-gray-500" />
    <span class="font-medium text-gray-700 dark:text-gray-300 truncate max-w-xs">{{ attachment.name }}</span>
  </button>
</template>

<script setup lang="ts">
import { PropType } from 'vue';
import type { ChatMessageAttachment } from '../types';
import { Paperclip } from 'lucide-vue-next';
import { useFilePreviewStore } from '../stores/filePreview';

const props = defineProps({
  attachment: {
    type: Object as PropType<ChatMessageAttachment>,
    required: true,
  },
});

const previewStore = useFilePreviewStore();

const openPreview = () => {
  previewStore.showPreview(props.attachment.url);
};
</script>