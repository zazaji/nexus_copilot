// frontend/src/composables/useToasts.ts
import { ref, readonly } from 'vue';
import { v4 as uuidv4 } from 'uuid';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'info';
  message: string;
}

const toasts = ref<Toast[]>([]);

export function useToasts() {
  const addToast = (type: Toast['type'], message: string, duration: number = 3000) => {
    const id = uuidv4();
    toasts.value.push({ id, type, message });
    setTimeout(() => removeToast(id), duration);
  };

  const removeToast = (id: string) => {
    const index = toasts.value.findIndex(t => t.id === id);
    if (index > -1) {
      toasts.value.splice(index, 1);
    }
  };

  return {
    toasts: readonly(toasts),
    success: (message: string) => addToast('success', message),
    error: (message: string) => addToast('error', message, 5000),
    info: (message: string) => addToast('info', message),
    removeToast,
  };
}