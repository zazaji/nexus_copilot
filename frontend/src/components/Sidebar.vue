<!-- frontend/src/components/Sidebar.vue -->
<template>
  <aside class="w-64 bg-white dark:bg-gray-800 flex flex-col border-r border-gray-200 dark:border-gray-700">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <button @click="uiStore.toggleSidebar" class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 -ml-2">
          <PanelLeftClose class="w-6 h-6" />
        </button>
        <h1 class="font-bold text-lg">{{ $t('sidebar.nexus') }}</h1>
      </div>
      <button 
        @click="launchCopilot" 
        class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700"
        title="Launch Copilot (CmdOrCtrl+Shift+C)"
      >
        <Sparkles class="w-5 h-5 text-blue-500" />
      </button>
    </div>

    <nav class="flex-1 px-2 py-2 space-y-1 overflow-y-auto">
      <div class="flex justify-between items-center px-3 py-2">
        <p class="text-xs font-semibold text-gray-400 uppercase">{{ $t('sidebar.chats') }}</p>
        <div class="flex items-center space-x-1">
            <n-tooltip trigger="hover">
              <template #trigger>
                <button 
                    @click="handleNewChat"
                    :disabled="chatStore.isAgentRunning"
                    class="p-1 rounded-md text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
                >
                    <MessageSquarePlus class="w-4 h-4" />
                </button>
              </template>
              New Chat
            </n-tooltip>
            <n-tooltip trigger="hover">
              <template #trigger>
                <button 
                    @click="handleNewCreation"
                    :disabled="chatStore.isAgentRunning"
                    class="p-1 rounded-md text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
                >
                    <Sparkles class="w-4 h-4 text-purple-500" />
                </button>
              </template>
              New Creation
            </n-tooltip>
            <n-tooltip trigger="hover">
              <template #trigger>
                <button 
                    @click="handleClearAll" 
                    :disabled="chatStore.conversationList.length === 0 || chatStore.isAgentRunning"
                    class="p-1 rounded-md text-gray-400 hover:bg-red-100 hover:text-red-600 dark:hover:bg-red-900/50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    <Trash class="w-4 h-4" />
                </button>
              </template>
              Clear All Chats
            </n-tooltip>
        </div>
      </div>
      <router-link
        v-for="convo in chatStore.conversationList"
        :key="convo.id"
        :to="convo.sessionType === 'creation' ? `/creation/${convo.id}` : `/chat/${convo.id}`"
        :aria-label="convo.title"
        class="group flex items-center justify-between px-3 py-2 text-sm font-medium rounded-md"
        :class="[
          isActive(convo)
            ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300'
            : 'text-gray-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700',
          chatStore.isAgentRunning ? 'pointer-events-none opacity-50' : ''
        ]"
      >
        <div class="truncate flex-1 flex items-center space-x-2" @dblclick.prevent="startEditing(convo)">
          <component 
            :is="convo.sessionType === 'creation' ? Sparkles : MessageSquare" 
            class="w-4 h-4 flex-shrink-0"
            :class="isActive(convo) ? '' : 'text-gray-400 dark:text-gray-500'"
          />
          <input
            v-if="editingConversationId === convo.id"
            ref="titleInputRef"
            v-model="editingTitle"
            @blur="finishEditing"
            @keydown.enter="finishEditing"
            @keydown.esc="cancelEditing"
            class="bg-transparent w-full focus:outline-none focus:ring-1 focus:ring-blue-500 rounded-sm px-1"
          />
          <span v-else class="truncate block">{{ convo.title }}</span>
        </div>
        <button 
          v-if="isActive(convo)" 
          @click.prevent.stop="handleDeleteConversation"
          class="ml-2 p-1 rounded text-gray-400 hover:bg-red-100 hover:text-red-600 dark:hover:bg-red-900/50 opacity-0 group-hover:opacity-100 transition-opacity"
          aria-label="Delete conversation"
        >
          <Trash2 class="w-4 h-4" />
        </button>
      </router-link>
    </nav>

    <div class="p-2 border-t border-gray-200 dark:border-gray-700">
        <p class="px-3 py-2 text-xs font-semibold text-gray-400 uppercase">{{ $t('sidebar.tools') }}</p>
        <router-link
            v-for="item in toolItems"
            :key="item.name"
            :to="item.path"
            :aria-label="item.name"
            class="flex items-center px-3 py-2 text-sm font-medium rounded-md"
            :class="[
                $route.name === item.name
                ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300'
                : 'text-gray-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700',
                chatStore.isAgentRunning ? 'pointer-events-none opacity-50' : ''
            ]"
        >
            <component :is="item.icon" class="w-5 h-5 mr-3" />
            <span>{{ $t(item.label) }}</span>
        </router-link>
    </div>

    <div class="p-4 border-t border-gray-200 dark:border-gray-700">
      <router-link to="/settings" class="flex items-center space-x-3 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-black dark:hover:text-white">
        <div class="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center">
          <User class="w-5 h-5 text-gray-500" />
        </div>
        <span class="flex-1">{{ $t('sidebar.userSettings') }}</span>
        <Settings class="w-5 h-5" />
      </router-link>
      <div v-if="!isOnline" class="mt-3 flex items-center justify-center space-x-2 text-xs text-yellow-600 dark:text-yellow-400 bg-yellow-100 dark:bg-yellow-900/50 p-1 rounded">
        <WifiOff class="w-3 h-3" />
        <span>{{ $t('sidebar.offline') }}</span>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { Database, Settings, WifiOff, User, ClipboardPaste, Trash2, Wrench, Sparkles, PanelLeftClose, Trash, MessageSquarePlus, MessageSquare } from 'lucide-vue-next';
import { useOnlineStatus } from '../composables/useOnlineStatus';
import { useChatStore } from '../stores/chat';
import { useCreationStore } from '../stores/creation';
import { useUiStore } from '../stores/ui';
import { show_window } from '../lib/api';
import type { Conversation } from '../types';
import { NTooltip } from 'naive-ui';

const { isOnline } = useOnlineStatus();
const chatStore = useChatStore();
const creationStore = useCreationStore();
const uiStore = useUiStore();
const router = useRouter();
const route = useRoute();

const editingConversationId = ref<string | null>(null);
const editingTitle = ref('');
const titleInputRef = ref<HTMLInputElement[] | null>(null);

const toolItems = computed(() => [
  { name: 'Dashboard', path: '/dashboard', icon: Database, label: 'sidebar.dashboard' },
  { name: 'KnowledgeBase', path: '/knowledge-base', icon: Database, label: 'sidebar.knowledgeBase' },
  { name: 'Tools', path: '/tools', icon: Wrench, label: 'sidebar.toolsLink' },
  { name: 'Clipboard', path: '/clipboard', icon: ClipboardPaste, label: 'sidebar.clipboard' },
]);

onMounted(() => {
  chatStore.loadConversationList();
});

const isActive = (convo: Conversation) => {
  const expectedRouteName = convo.sessionType === 'creation' ? 'Creation' : 'Chat';
  return route.params.id === convo.id && route.name === expectedRouteName;
};

const handleNewChat = () => {
  chatStore.startNewConversation();
};

const handleNewCreation = () => {
  creationStore.startNewSession();
};

const launchCopilot = () => {
  show_window('copilot', true);
};

const handleDeleteConversation = async () => {
  await chatStore.deleteCurrentConversation();
};

const handleClearAll = async () => {
  await chatStore.clearAllConversations();
};

const startEditing = (convo: Conversation) => {
  editingConversationId.value = convo.id;
  editingTitle.value = convo.title;
  nextTick(() => {
    titleInputRef.value?.[0]?.focus();
  });
};

const finishEditing = () => {
  if (editingConversationId.value && editingTitle.value.trim()) {
    chatStore.updateConversationTitle(editingConversationId.value, editingTitle.value.trim());
  }
  cancelEditing();
};

const cancelEditing = () => {
  editingConversationId.value = null;
  editingTitle.value = '';
};
</script>