<!-- frontend/src/components/ToastProvider.vue -->
<template>
  <div class="fixed top-5 right-5 z-[100] space-y-3 w-full max-w-sm">
    <Toast
      v-for="toast in toasts"
      :key="toast.id"
      :toast="toast"
      @close="removeToast(toast.id)"
    />
  </div>
</template>

<script setup lang="ts">
import { useToasts } from '../composables/useToasts';
import Toast from './Toast.vue';

const { toasts, removeToast } = useToasts();
</script>