<!-- frontend/src/components/Toast.vue -->
<template>
  <transition name="toast">
    <div
      class="p-4 rounded-lg shadow-lg text-white flex items-center space-x-3"
      :class="toastClasses"
    >
      <component :is="toastIcon" class="w-6 h-6 flex-shrink-0" />
      <span class="flex-1">{{ toast.message }}</span>
      <button @click="$emit('close')" class="p-1 rounded-full hover:bg-white/20">
        <X class="w-4 h-4" />
      </button>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import type { Toast } from '../composables/useToasts';
import { CheckCircle2, XCircle, Info, X } from 'lucide-vue-next';

const props = defineProps({
  toast: {
    type: Object as PropType<Toast>,
    required: true,
  },
});

defineEmits(['close']);

const toastClasses = computed(() => ({
  'bg-green-500': props.toast.type === 'success',
  'bg-red-500': props.toast.type === 'error',
  'bg-blue-500': props.toast.type === 'info',
}));

const toastIcon = computed(() => {
  switch (props.toast.type) {
    case 'success': return CheckCircle2;
    case 'error': return XCircle;
    case 'info': return Info;
    default: return Info;
  }
});
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.5s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}
</style>