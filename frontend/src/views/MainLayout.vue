<!-- frontend/src/views/MainLayout.vue -->
<template>
  <div class="flex h-screen bg-gray-100 dark:bg-gray-900 text-gray-800 dark:text-gray-200">
    <Sidebar v-if="!uiStore.isSidebarCollapsed" />
    <main class="flex-1 flex flex-col overflow-hidden relative">
      <button 
        v-if="uiStore.isSidebarCollapsed"
        @click="uiStore.toggleSidebar" 
        class="absolute top-4 left-4 z-20 p-2 rounded-full bg-gray-200/50 dark:bg-gray-800/50 hover:bg-gray-300/80 dark:hover:bg-gray-700/80 backdrop-blur-sm"
        title="Show Sidebar"
      >
        <PanelRightClose class="w-5 h-5" />
      </button>

      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" :key="$route.fullPath" />
        </transition>
      </router-view>
    </main>
    <FilePreviewModal />
  </div>
</template>

<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue';
import FilePreviewModal from '../components/FilePreviewModal.vue';
import { useUiStore } from '../stores/ui';
import { PanelRightClose } from 'lucide-vue-next';

const uiStore = useUiStore();
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>