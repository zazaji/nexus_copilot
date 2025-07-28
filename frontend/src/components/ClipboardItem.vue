<!-- frontend/src/components/ClipboardItem.vue -->
<template>
  <div 
    class="p-4 group transition-colors"
    :class="item.isPinned ? 'bg-blue-50 dark:bg-blue-900/50' : 'hover:bg-gray-50 dark:hover:bg-gray-700/50'"
  >
    <div class="flex items-start space-x-4">
      <div class="flex-shrink-0 text-gray-400 mt-1">
        <Type v-if="item.type === 'text'" class="w-5 h-5" />
        <ImageIcon v-else-if="item.type === 'image'" class="w-5 h-5" />
        <File v-else-if="item.type === 'files'" class="w-5 h-5" />
      </div>
      <div class="flex-1 min-w-0">
        <!-- Text Content -->
        <div 
          v-if="item.type === 'text'" 
          @click="toggleExpansion"
          class="text-sm text-gray-800 dark:text-gray-200 break-words whitespace-pre-wrap font-mono cursor-pointer"
          :class="{ 'line-clamp-1': !isExpanded }"
          title="Click to expand/collapse"
        >
          {{ item.content }}
        </div>
        
        <!-- Image Content -->
        <div v-else-if="item.type === 'image'" @click="previewItem" class="cursor-pointer">
          <img :src="imageUrl" class="max-h-32 rounded-md border dark:border-gray-700" />
        </div>

        <!-- Files Content -->
        <div v-else-if="item.type === 'files' && Array.isArray(parsedContent)" class="space-y-1">
            <div 
              v-for="(file, index) in parsedContent" 
              :key="index" 
              @click="showInFolder(file)"
              class="text-sm text-gray-500 italic flex items-center space-x-2 cursor-pointer hover:text-blue-500 dark:hover:text-blue-400"
              :title="`Show '${file}' in folder`"
            >
                <FileText class="w-4 h-4" />
                <span>{{ file.split(/[/\\]/).pop() }}</span>
            </div>
        </div>

        <p class="text-xs text-gray-400 mt-2">{{ new Date(item.timestamp).toLocaleString() }}</p>
      </div>
      <div class="flex-shrink-0 flex flex-row items-center space-x-2">
        <button @click.stop="togglePin" class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" :aria-label="item.isPinned ? 'Unpin item' : 'Pin item'">
          <Pin class="w-4 h-4 text-gray-500" :class="{ 'fill-current text-blue-500': item.isPinned }" />
        </button>
        <button @click.stop="copyToClipboard" class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600" aria-label="Copy to clipboard">
          <Copy class="w-4 h-4 text-gray-500" />
        </button>
        <button @click.stop="deleteItem" class="p-2 rounded-md hover:bg-red-100 dark:hover:bg-red-900/50 opacity-0 group-hover:opacity-100 transition-opacity" aria-label="Delete item">
          <Trash2 class="w-4 h-4 text-red-500" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, PropType, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ClipboardItem } from '../types';
import { useClipboardStore } from '../stores/clipboard';
import { useFilePreviewStore } from '../stores/filePreview';
import { Type, ImageIcon, Copy, Trash2, File, FileText, Pin } from 'lucide-vue-next';
import { useToasts } from '../composables/useToasts';
import { showInFolder as apiShowInFolder } from '../lib/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const props = defineProps({
  item: {
    type: Object as PropType<ClipboardItem>,
    required: true,
  },
});

const { t } = useI18n();
const { success, error, info } = useToasts();
const clipboardStore = useClipboardStore();
const previewStore = useFilePreviewStore();
const isExpanded = ref(false);

const parsedContent = computed(() => {
    if (props.item.type === 'files' && typeof props.item.content === 'string') {
        try {
            return JSON.parse(props.item.content);
        } catch (e) {
            console.error("Failed to parse file content from clipboard history:", e);
            return [];
        }
    }
    return props.item.content;
});

const imageUrl = computed(() => {
    if (props.item.type === 'image' && typeof props.item.content === 'string') {
        // The content can be a data URL or a file path that needs conversion
        if (props.item.content.startsWith('data:image')) {
            return props.item.content;
        }
        return convertFileSrc(props.item.content);
    }
    return '';
});

const toggleExpansion = () => {
  if (props.item.type === 'text') {
    isExpanded.value = !isExpanded.value;
  }
};

const previewItem = () => {
    if (props.item.type === 'image') {
        previewStore.showPreview(props.item.content);
    }
};

const showInFolder = (path: string) => {
    apiShowInFolder(path);
};

const copyToClipboard = async () => {
  if (props.item.type === 'text') {
    try {
      await navigator.clipboard.writeText(props.item.content);
      success('Text copied to clipboard!');
    } catch (err) {
      error('Failed to copy text.');
      console.error('Clipboard write failed:', err);
    }
  } else {
    info('Copying images or files from history is not yet supported.');
  }
};

const togglePin = () => {
    clipboardStore.togglePin(props.item.id);
};

const deleteItem = () => {
    clipboardStore.removeItem(props.item.id);
};
</script>