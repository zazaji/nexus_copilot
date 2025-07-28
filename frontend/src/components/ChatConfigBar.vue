<!-- frontend/src/components/ChatConfigBar.vue -->
<template>
  <div class="mt-3 flex items-center justify-between text-xs text-gray-600 dark:text-gray-400">
    <div class="flex items-center space-x-2">
      <!-- Creation Type Selector -->
      <template v-if="mode === 'creation'">
        <label for="type-select">Type:</label>
        <select 
          id="type-select" 
          :value="creationTypeValue"
          @change="$emit('update:creationTypeValue', ($event.target as HTMLSelectElement).value as CreationType)"
          class="bg-transparent border-none p-1 rounded focus:ring-0 text-xs"
        >
          <option value="image">Image</option>
          <option value="video">Video</option>
          <option value="audio">Audio</option>
        </select>
      </template>

      <!-- Model Selector -->
      <label for="model-select">Model:</label>
      <select 
        id="model-select" 
        :value="modelValue"
        @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
        class="bg-transparent border-none p-1 rounded focus:ring-0 text-xs"
      >
        <option v-for="model in availableModels" :key="model.value" :value="model.value">
          {{ isInCopilot ? truncate(model.label, 25) : model.label }}
        </option>
      </select>

      <!-- Creation Parameters -->
      <template v-if="mode === 'creation'">
        <!-- Image Params -->
        <template v-if="creationTypeValue === 'image' && imageParamsValue">
          <label for="size-select">Size:</label>
          <select 
            id="size-select" 
            :value="imageParamsValue.size"
            @change="$emit('update:imageParamsValue', { ...imageParamsValue, size: ($event.target as HTMLSelectElement).value })"
            class="bg-transparent border-none p-1 rounded focus:ring-0 text-xs"
          >
            <option>1024x1024</option>
            <option>1792x1024</option>
            <option>1024x1792</option>
          </select>
        </template>
        <!-- Audio Params -->
        <template v-if="creationTypeValue === 'audio' && audioParamsValue">
          <label for="voice-select">Voice:</label>
          <select 
            id="voice-select" 
            :value="audioParamsValue.voice"
            @change="$emit('update:audioParamsValue', { ...audioParamsValue, voice: ($event.target as HTMLSelectElement).value })"
            class="bg-transparent border-none p-1 rounded focus:ring-0 text-xs"
          >
            <option>alloy</option>
            <option>echo</option>
            <option>fable</option>
            <option>onyx</option>
            <option>nova</option>
            <option>shimmer</option>
          </select>
        </template>
      </template>
    </div>

    <!-- Knowledge Base Selector (Chat Mode Only) -->
    <div v-if="mode === 'chat'" class="flex items-center space-x-2">
      <label v-if="!isInCopilot" for="kb-select">Knowledge:</label>
      <select 
        id="kb-select" 
        :value="knowledgeSelection"
        @change="$emit('update:knowledgeSelection', ($event.target as HTMLSelectElement).value)"
        class="bg-transparent border-none p-1 rounded focus:ring-0 text-xs"
      >
        <option value="none">None</option>
        <option value="internet_search">Internet Search</option>
        <optgroup label="Local Files">
            <option value="all">All Local Files</option>
            <option v-for="dir in settingsStore.settings?.knowledgeBase.indexedDirectories" :key="dir" :value="dir">
            {{ dir.split(/[/\\]/).pop() }}
            </option>
        </optgroup>
        <optgroup v-if="kbStore.onlineKbs.length > 0" label="Online">
            <option v-for="kb in kbStore.onlineKbs" :key="kb.id" :value="`online::${kb.id}`">
                {{ kb.name }}
            </option>
        </optgroup>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useChatStore } from '../stores/chat';
import { useSettingsStore } from '../stores/settings';
import { useKnowledgeBaseStore } from '../stores/knowledgeBase';
import type { CreationType } from '../types';

const props = defineProps<{
  modelValue: string | null;
  knowledgeSelection: string;
  isInCopilot?: boolean;
  mode: 'chat' | 'creation';
  creationTypeValue?: CreationType;
  imageParamsValue?: { size: string };
  audioParamsValue?: { voice: string };
}>();

defineEmits([
  'update:modelValue', 
  'update:knowledgeSelection',
  'update:creationTypeValue',
  'update:imageParamsValue',
  'update:audioParamsValue'
]);

const chatStore = useChatStore();
const settingsStore = useSettingsStore();
const kbStore = useKnowledgeBaseStore();

const availableModels = computed(() => {
  if (props.mode === 'creation') {
    if (props.creationTypeValue === 'image') {
      return chatStore.getModelsByCapability('image_gen');
    }
    if (props.creationTypeValue === 'audio') {
      return chatStore.getModelsByCapability('tts');
    }
    if (props.creationTypeValue === 'video') {
      return chatStore.getModelsByCapability('video_gen');
    }
  }
  return chatStore.availableModels;
});

onMounted(() => {
    kbStore.fetchOnlineKbs();
});

const truncate = (text: string, length: number) => {
  if (text.length <= length) return text;
  return text.substring(0, length) + '...';
};
</script>

<style scoped>
select {
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  background-image: none;
  padding-right: 1.5rem; /* Add space for a custom arrow if needed */
}
/* You can add a custom arrow using a background image or pseudo-element if desired */
</style>