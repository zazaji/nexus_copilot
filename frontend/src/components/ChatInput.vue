<!-- frontend/src/components/ChatInput.vue -->
<template>
  <div class="relative">
    <div v-if="!isOnline" class="absolute -top-8 left-0 text-xs text-yellow-600 dark:text-yellow-400">
      Offline mode: Chat is disabled.
    </div>
    <div class="bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg focus-within:ring-2 focus-within:ring-blue-500 flex flex-col">
      <div v-if="attachments.length > 0" class="p-2 flex flex-wrap gap-2 border-b border-gray-200 dark:border-gray-700">
        <div v-for="(file, index) in attachments" :key="index" class="relative group">
          <div 
            @click="previewAttachment(file)"
            class="w-20 h-20 bg-gray-100 dark:bg-gray-700 rounded-md flex items-center justify-center overflow-hidden cursor-pointer"
          >
            <img v-if="file.type.startsWith('image/')" :src="file.preview" class="w-full h-full object-cover" />
            <FileText v-else class="w-8 h-8 text-gray-500" />
          </div>
          <button @click="removeAttachment(index)" class="absolute -top-1.5 -right-1.5 bg-gray-600 text-white rounded-full p-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
            <X class="w-3 h-3" />
          </button>
          <div class="text-xs truncate w-20 text-center mt-1">{{ file.file.name }}</div>
        </div>
      </div>
      <div class="flex items-end space-x-2 p-2">
        <button @click="triggerFileInput" class="p-3 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 self-end">
          <Paperclip class="w-5 h-5" />
        </button>
        <input type="file" ref="fileInputRef" @change="handleFileChange" accept="image/*, .txt, .md, .pdf, .docx" class="hidden" multiple />
        <div class="flex-1 max-h-48 overflow-y-auto">
          <textarea
            ref="textareaRef"
            v-model="model"
            @keydown.enter.exact.prevent
            @keydown.enter.meta.prevent="handleButtonClick"
            @keydown.enter.ctrl.prevent="handleButtonClick"
            @paste="handlePaste"
            :disabled="isLoading"
            :placeholder="placeholderText"
            class="w-full p-2 bg-transparent focus:outline-none resize-none disabled:bg-gray-100 dark:disabled:bg-gray-700 overflow-hidden"
            rows="1"
          ></textarea>
        </div>
        <button
          @click="handleButtonClick"
          :disabled="!isLoading && ((model || '').trim() === '' && attachments.length === 0)"
          :aria-label="isLoading ? 'Stop generation' : 'Send message'"
          class="h-10 w-10 flex-shrink-0 flex items-center justify-center rounded-full text-white disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors self-end"
          :class="isLoading ? 'bg-red-500 hover:bg-red-600' : 'bg-blue-500 hover:bg-blue-600'"
        >
          <Square v-if="isLoading" class="w-5 h-5" />
          <Send v-else class="w-5 h-5" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Send, Paperclip, X, FileText, Square } from 'lucide-vue-next';
import { useOnlineStatus } from '../composables/useOnlineStatus';
import { useFilePreviewStore } from '../stores/filePreview';
import { saveTempAssetWithHash } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import SparkMD5 from 'spark-md5';

interface Attachment {
  file: File;
  preview: string; // Always a Base64 Data URL for immediate display
  type: string;
  localPath: string | null; // Path of the saved asset on disk
}

const props = defineProps({
  isLoading: {
    type: Boolean,
    default: false,
  },
  mode: {
    type: String as () => 'chat' | 'creation',
    default: 'chat',
  }
});

const { t } = useI18n();
const { error } = useToasts();
const emit = defineEmits(['send', 'stop']);
const model = defineModel<string>();

const { isOnline } = useOnlineStatus();
const previewStore = useFilePreviewStore();
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const fileInputRef = ref<HTMLInputElement | null>(null);
const attachments = ref<Attachment[]>([]);

const placeholderText = computed(() => {
  if (props.mode === 'creation') {
    return t('creation.inputPlaceholder');
  }
  return t('chat.inputPlaceholder') + ' (or /plan, /explore, /write, /research for tasks)';
});

const adjustHeight = () => {
  nextTick(() => {
    if (textareaRef.value) {
      textareaRef.value.style.height = 'auto';
      textareaRef.value.style.height = `${textareaRef.value.scrollHeight}px`;
    }
  });
};

watch(model, adjustHeight, { immediate: true });

const triggerFileInput = () => {
  fileInputRef.value?.click();
};

const processAndSaveImage = (file: File): Promise<{ dataUrl: string, base64: string, extension: string }> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const img = new Image();
      img.onload = () => {
        const MAX_DIMENSION = 2048;
        let { width, height } = img;

        if (width > MAX_DIMENSION || height > MAX_DIMENSION) {
          if (width > height) {
            height = Math.round(height * (MAX_DIMENSION / width));
            width = MAX_DIMENSION;
          } else {
            width = Math.round(width * (MAX_DIMENSION / height));
            height = MAX_DIMENSION;
          }
        }

        const canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext('2d');
        ctx?.drawImage(img, 0, 0, width, height);
        
        const webpDataUrl = canvas.toDataURL('image/webp', 0.8);
        const webpBase64 = webpDataUrl.split(',')[1];
        
        // Base64 encoding adds ~33% overhead. We estimate the binary size.
        const webpEstimatedSize = webpBase64.length * 0.75;
        const useWebp = webpEstimatedSize < (file.size * 0.95);

        if (useWebp) {
          resolve({ dataUrl: webpDataUrl, base64: webpBase64, extension: 'webp' });
        } else {
          // If WebP is not smaller, use the original file data.
          // The reader already has the original file as a data URL.
          const originalDataUrl = e.target?.result as string;
          const originalBase64 = originalDataUrl.split(',')[1];
          const originalExtension = file.type.split('/')[1] || 'png';
          resolve({ dataUrl: originalDataUrl, base64: originalBase64, extension: originalExtension });
        }
      };
      img.onerror = reject;
      img.src = e.target?.result as string;
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
};

const addFileAsAttachment = async (file: File) => {
  if (file.type.startsWith('image/')) {
    try {
      const { dataUrl, base64, extension } = await processAndSaveImage(file);
      const hash = SparkMD5.hash(base64);
      
      const savedPath = await saveTempAssetWithHash(base64, hash, extension);
      
      if (savedPath) {
        attachments.value.push({
          file,
          preview: dataUrl,
          type: `image/${extension}`,
          localPath: savedPath,
        });
      } else {
        error('Failed to save processed image locally.');
      }
    } catch (err) {
      console.error('Image processing failed:', err);
      error('Failed to process image.');
    }
  } else {
    console.log('Non-image file attachments are not yet fully supported for sending.');
  }
};

const handleFileChange = (event: Event) => {
  const files = (event.target as HTMLInputElement).files;
  if (!files) return;
  for (const file of files) {
    addFileAsAttachment(file);
  }
};

const handlePaste = (event: ClipboardEvent) => {
  const items = event.clipboardData?.items;
  if (!items) return;
  for (const item of items) {
    if (item.kind === 'file') {
      const file = item.getAsFile();
      if (file) {
        event.preventDefault();
        addFileAsAttachment(file);
      }
    }
  }
};

const removeAttachment = (index: number) => {
  attachments.value.splice(index, 1);
};

const previewAttachment = (attachment: Attachment) => {
  if (attachment.type.startsWith('image/')) {
    previewStore.showPreview(attachment.preview);
  } else {
    console.log("Preview for this file type is not supported yet.");
  }
};

const send = () => {
  if ((model.value || '').trim() === '' && attachments.value.length === 0) return;
  
  emit('send', { text: model.value, attachments: attachments.value });
  model.value = '';
  attachments.value = [];
  adjustHeight();
};

const handleButtonClick = () => {
  if (props.isLoading) {
    emit('stop');
  } else {
    send();
  }
};
</script>