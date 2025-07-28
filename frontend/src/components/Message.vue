<!-- frontend/src/components/Message.vue -->
<template>
  <div class="flex flex-col group" :class="isUser ? 'items-end' : 'items-start'">
    <div 
      class="p-4 rounded-lg relative"
      :class="[
        isUser 
          ? 'bg-blue-500 text-white rounded-br-none max-w-2xl' 
          : 'bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 rounded-bl-none shadow-sm w-11/12',
        message.error ? 'border border-red-500/50' : '',
        message.model === 'agent' ? 'w-full' : ''
      ]"
    >
      <!-- Message Actions -->
      <div v-if="!isEditing" class="absolute top-1 right-1 flex items-center space-x-1 opacity-0 group-hover:opacity-100 transition-opacity p-0.5 rounded-md z-10"
        :class="isUser ? 'bg-blue-600/50' : 'bg-gray-100 dark:bg-gray-700'"
      >
        <button v-if="!isUser && isTextOnly" @click="playAudio" class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-600" title="Play Audio">
          <Loader2 v-if="isAudioLoading" class="w-3 h-3 animate-spin" />
          <PlayCircle v-else class="w-3 h-3" />
        </button>
        <button @click="copyContent" class="p-1 rounded" :class="isUser ? 'hover:bg-blue-700/50' : 'hover:bg-gray-200 dark:hover:bg-gray-600'" title="Copy">
          <Copy class="w-3 h-3" />
        </button>
        <button v-if="isTextOnly" @click="startEditing" class="p-1 rounded" :class="isUser ? 'hover:bg-blue-700/50' : 'hover:bg-gray-200 dark:hover:bg-gray-600'" title="Edit">
          <Pencil class="w-3 h-3" />
        </button>
        <button v-if="!isUser" @click="$emit('refresh', message.id)" class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-600" title="Refresh">
          <RefreshCw class="w-3 h-3" />
        </button>
        <button @click="$emit('delete', message.id)" class="p-1 rounded" :class="isUser ? 'hover:bg-red-500/50' : 'hover:bg-red-100 dark:hover:bg-red-900/50'" title="Delete">
          <Trash2 class="w-3 h-3" :class="isUser ? 'text-white' : 'text-red-500'" />
        </button>
      </div>

      <!-- Unified Thinking Process Display -->
      <div v-if="!isUser && thinkingContent" class="mb-4 border-l-4 border-gray-300 dark:border-gray-600 pl-4 py-2 bg-gray-50 dark:bg-gray-700/50 rounded-r-lg">
        <div class="flex items-center space-x-2 text-sm font-semibold text-gray-500 dark:text-gray-400">
          <BrainCircuit class="w-4 h-4" />
          <span>Thinking...</span>
        </div>
        <div class="mt-2 text-sm text-gray-600 dark:text-gray-400 whitespace-pre-wrap">
          {{ thinkingContent }}
        </div>
      </div>

      <div v-if="message.error" class="flex items-start space-x-2 text-red-700 dark:text-red-400">
        <AlertTriangle class="w-5 h-5 flex-shrink-0 mt-0.5" />
        <div class="text-sm">
          <p class="font-bold">An error occurred</p>
          <p class="mt-1 font-mono text-xs">{{ message.error }}</p>
        </div>
      </div>
      <div v-else class="flex items-end w-full">
        <!-- Editing View -->
        <div v-if="isEditing" class="w-full">
          <textarea 
            ref="editInputRef"
            v-model="editableContent"
            @input="adjustTextareaHeight"
            class="w-full p-0 bg-transparent focus:outline-none resize-none font-sans text-base"
            :class="isUser ? 'text-white' : 'text-gray-800 dark:text-gray-200'"
            rows="1"
          ></textarea>
          <div class="mt-2 flex justify-end space-x-2">
            <button @click="cancelEditing" class="px-3 py-1 text-xs rounded-md" :class="isUser ? 'bg-blue-400/80 hover:bg-blue-400' : 'bg-gray-200 dark:bg-gray-600 hover:bg-gray-300'">Cancel</button>
            <button @click="saveEdit" class="px-3 py-1 text-xs rounded-md" :class="isUser ? 'bg-white text-blue-600 hover:bg-gray-100' : 'bg-blue-500 text-white hover:bg-blue-600'">Save</button>
          </div>
        </div>
        <!-- Display View -->
        <div v-else class="prose prose-base dark:prose-invert max-w-none w-full" :class="{ 'prose-invert-user': isUser }">
          <template v-for="(part, index) in parsedContent" :key="index">
            <div v-if="part.type === 'text'" v-html="renderedMarkdown(part.content)"></div>
            <CodeBlock 
              v-else-if="part.type === 'code'"
              :code="part.content"
              :language="part.language"
              :is-executing="!!message.isExecuting"
              @execute="handleExecuteCode"
            />
            <MediaBlock
              v-else-if="part.type === 'image'"
              :url="part.url"
            />
          </template>
          <div v-if="isStreaming && !thinkingContent" class="w-2 h-5 bg-blue-500 dark:bg-blue-400 animate-pulse ml-1 mb-0.5 inline-block"></div>
        </div>
      </div>
    </div>

    <!-- Metadata Area: Model, Sources, and Attachments -->
    <div v-if="!isUser" class="w-full  mt-2 flex flex-wrap gap-2 items-center">
      <div v-if="modelLabel" class="flex-shrink-0">
        <span class="px-2 py-0.5 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full text-xs font-mono">
          {{ modelLabel }}
        </span>
      </div>
      <SourcePill 
        v-for="source in message.sources" 
        :key="source.id" 
        :source="source"
        @request-open-in-main="handleOpenRequest"
      />
      <AttachmentPill 
        v-for="attachment in message.attachments" 
        :key="attachment.url" 
        :attachment="attachment"
      />
    </div>

    <!-- Code Execution Output -->
    <div v-if="message.executionOutput" class="mt-2 w-full max-w-6xl">
      <div class="bg-black/80 rounded-lg border border-gray-700">
        <div class="px-4 py-2 border-b border-gray-700 text-xs font-semibold text-gray-300">Execution Output</div>
        <pre class="p-4 text-sm text-white whitespace-pre-wrap font-mono overflow-x-auto max-h-60">{{ message.executionOutput }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType, ref, nextTick, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useKnowledgeExplorerStore } from '../stores/knowledgeExplorer';
import { useChatStore } from '../stores/chat';
import { useSettingsStore } from '../stores/settings';
import { AlertTriangle, Pencil, RefreshCw, Loader2, Trash2, BrainCircuit, PlayCircle, Copy } from 'lucide-vue-next';
import type { ChatMessage, KnowledgeSource, ChatMessageContentPart } from '../types';
import { parseMarkdown } from '../utils/markdownParser';
import { getTtsAudio } from '../lib/api';
import { useToasts } from '../composables/useToasts';
import SourcePill from './SourcePill.vue';
import AttachmentPill from './AttachmentPill.vue';
import CodeBlock from './CodeBlock.vue';
import MediaBlock from './MediaBlock.vue';

interface ParsedContentPart {
  type: 'text' | 'code' | 'image';
  content?: string;
  language?: string;
  url?: string;
}

const props = defineProps({
  message: {
    type: Object as PropType<ChatMessage>,
    required: true,
  },
  isStreaming: {
    type: Boolean,
    default: false,
  },
  isInCopilot: {
    type: Boolean,
    default: false,
  }
});

const emit = defineEmits(['save-edit', 'refresh', 'delete', 'parsed-content-updated']);

const router = useRouter();
const explorerStore = useKnowledgeExplorerStore();
const chatStore = useChatStore();
const settingsStore = useSettingsStore();
const { success } = useToasts();

const isEditing = ref(false);
const editableContent = ref('');
const editInputRef = ref<HTMLTextAreaElement | null>(null);
const isAudioLoading = ref(false);
let currentAudio: HTMLAudioElement | null = null;

const isUser = computed(() => props.message.role === 'user');

const isTextOnly = computed(() => {
  return props.message.content.every(part => part.type === 'text');
});

const modelLabel = computed(() => {
  if (!props.message.model || props.message.model.startsWith('agent')) {
    return null;
  }
  const modelInfo = chatStore.availableModels.find(m => m.value === props.message.model);
  if (modelInfo) {
    return modelInfo.label;
  }
  return props.message.model.split('::')[1] || props.message.model;
});

const getTextContent = (content: ChatMessageContentPart[]): string => {
  return content.filter(p => p.type === 'text').map(p => (p as { text: string }).text).join('\n');
};

const fullTextContent = computed(() => getTextContent(props.message.content));

const thinkingContent = computed(() => {
  if (props.isStreaming && props.message.thinkingProcess && props.message.thinkingProcess.length > 0) {
    return props.message.thinkingProcess.join('');
  }
  if (!props.isStreaming) {
    const thinkRegex = /<think>([\s\S]*?)<\/think>|<thinking>([\s\S]*?)<\/thinking>/g;
    const thoughts = [...fullTextContent.value.matchAll(thinkRegex)].map(match => match[1] || match[2]);
    return thoughts.join('\n');
  }
  return null;
});

const parsedContent = computed<ParsedContentPart[]>(() => {
  const finalParts: ParsedContentPart[] = [];

  props.message.content.forEach(part => {
    if (part.type === 'text') {
      let textContent = part.text;
      if (!props.isStreaming) {
        textContent = textContent.replace(/<(?:think|thinking)>[\s\S]*?<\/(?:think|thinking)>/g, '');
      }

      const regex = /(```(\w+)?\n[\s\S]*?```)/g;
      let lastIndex = 0;
      let match;

      while ((match = regex.exec(textContent)) !== null) {
        if (match.index > lastIndex) {
          finalParts.push({ type: 'text', content: textContent.substring(lastIndex, match.index) });
        }
        const codeContent = match[1].replace(/```(\w+)?\n|```/g, '').trim();
        finalParts.push({ type: 'code', language: match[2] || 'plaintext', content: codeContent });
        lastIndex = match.index + match[0].length;
      }

      if (lastIndex < textContent.length) {
        finalParts.push({ type: 'text', content: textContent.substring(lastIndex) });
      }
    } else if (part.type === 'image_url') {
      finalParts.push({ type: 'image', url: part.image_url.url });
    }
  });
  
  return finalParts;
});

const copyContent = async () => {
  if (isTextOnly.value) {
    const textToCopy = parsedContent.value
      .map(part => part.content)
      .join('\n\n');
    await navigator.clipboard.writeText(textToCopy);
    success('Content copied to clipboard!');
  } else {
    const mediaPart = props.message.content.find(part => part.type === 'image_url');
    if (mediaPart && mediaPart.type === 'image_url') {
      await navigator.clipboard.writeText(mediaPart.image_url.url);
      success('Media path copied to clipboard!');
    }
  }
};

const playAudio = async () => {
  if (isAudioLoading.value) return;
  if (currentAudio) {
    currentAudio.pause();
    currentAudio = null;
  }

  isAudioLoading.value = true;
  const textToPlay = parsedContent.value
    .filter(p => p.type === 'text')
    .map(p => p.content)
    .join('\n');

  if (!textToPlay.trim() || !settingsStore.settings) {
    isAudioLoading.value = false;
    return;
  }

  const audioBlob = await getTtsAudio(textToPlay, settingsStore.settings);
  isAudioLoading.value = false;

  if (audioBlob) {
    const audioUrl = URL.createObjectURL(audioBlob);
    currentAudio = new Audio(audioUrl);
    currentAudio.play();
    currentAudio.onended = () => {
      URL.revokeObjectURL(audioUrl);
      currentAudio = null;
    };
  }
};

watch(parsedContent, (newVal) => {
    emit('parsed-content-updated', newVal);
}, { immediate: true });

const renderedMarkdown = (text: string) => {
  return parseMarkdown(text);
};

const adjustTextareaHeight = () => {
  const textarea = editInputRef.value;
  if (textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = `${textarea.scrollHeight}px`;
  }
};

const startEditing = () => {
  editableContent.value = getTextContent(props.message.content);
  isEditing.value = true;
  nextTick(() => {
    if (editInputRef.value) {
      editInputRef.value.focus();
      adjustTextareaHeight();
    }
  });
};

const cancelEditing = () => {
  isEditing.value = false;
};

const saveEdit = () => {
  emit('save-edit', { messageId: props.message.id, newContent: editableContent.value });
  isEditing.value = false;
};

const handleExecuteCode = (code: string) => {
  chatStore.executeCodeInMessage(props.message.id, code);
};

const handleOpenRequest = (source: KnowledgeSource) => {
  if (props.isInCopilot || source.file_path.startsWith('online-kb://')) {
    return;
  }
  router.push({ name: 'KnowledgeBase' });
  router.isReady().then(() => {
    explorerStore.selectFile(source.file_path);
  });
};

watch(editableContent, () => {
  adjustTextareaHeight();
});
</script>

<style>
/* Custom styles for user message prose */
.prose-invert-user {
  --tw-prose-body: theme('colors.white');
  --tw-prose-headings: theme('colors.white');
  --tw-prose-lead: theme('colors.blue.200');
  --tw-prose-links: theme('colors.white');
  --tw-prose-bold: theme('colors.white');
  --tw-prose-counters: theme('colors.blue.300');
  --tw-prose-bullets: theme('colors.blue.300');
  --tw-prose-hr: theme('colors.blue.400');
  --tw-prose-quotes: theme('colors.blue.100');
  --tw-prose-quote-borders: theme('colors.blue.300');
  --tw-prose-captions: theme('colors.blue.200');
  --tw-prose-code: theme('colors.white');
  --tw-prose-pre-code: theme('colors.blue.100');
  --tw-prose-pre-bg: theme('colors.blue.900');
  --tw-prose-th-borders: theme('colors.blue.300');
  --tw-prose-td-borders: theme('colors.blue.200');
}
</style>