<!-- frontend/src/components/copilot/ClipboardPreview.vue -->
<template>
  <div class="bg-black/20 rounded-lg p-3 border border-white/10">
    <div v-if="!context || !context.content">
      <p class="text-sm text-gray-400 text-center py-4">Clipboard is empty.</p>
    </div>
    <div v-else>
      <textarea 
        v-if="context.contentType === 'text'" 
        v-model="editableContent"
        class="w-full bg-transparent text-sm text-gray-200 whitespace-pre-wrap break-words font-mono focus:outline-none resize-none"
        rows="10"
        ref="textareaRef"
      ></textarea>
      <div v-else-if="context.contentType === 'image'" class="flex justify-center items-center max-h-72 overflow-y-auto">
        <img :src="imageUrl" class="max-w-full max-h-full object-contain rounded" alt="Copied image" />
      </div>
      <div v-else-if="context.contentType === 'files'" class="max-h-72 overflow-y-auto">
         <div class="flex flex-wrap gap-2">
            <div v-for="(file, index) in (context.content as string[])" :key="index" class="w-24 h-24 bg-black/30 rounded-md flex items-center justify-center overflow-hidden">
                <img :src="convertFileSrc(file)" class="w-full h-full object-cover" :alt="`Copied file ${index + 1}`" />
            </div>
         </div>
      </div>
      <div v-else class="text-sm text-gray-400 italic">
        Unsupported content type: {{ context.contentType }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref } from 'vue';
import type { OverlayContext } from '../../types';
import { useOverlayStore } from '../../stores/overlay';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const props = defineProps({
  context: {
    type: Object as PropType<OverlayContext | null>,
    required: true,
  },
});

const overlayStore = useOverlayStore();
const textareaRef = ref<HTMLTextAreaElement | null>(null);

const editableContent = computed({
  get() {
    if (props.context?.contentType === 'text') {
      return props.context.content as string;
    }
    return '';
  },
  set(value) {
    overlayStore.updateContextContent(value);
  }
});

const imageUrl = computed(() => {
  if (props.context?.contentType === 'image' && typeof props.context.content === 'string') {
    // It can be a data URL or a tauri asset URL
    return props.context.content.startsWith('data:image') 
      ? props.context.content 
      : convertFileSrc(props.context.content);
  }
  return '';
});
</script>