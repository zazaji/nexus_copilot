<!-- frontend/src/components/knowledge/MilkdownEditor.vue -->
<template>
  <div class="milkdown-container h-full w-full overflow-y-auto">
    <Milkdown :editor="editor" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { Editor, rootCtx, defaultValueCtx, editorViewOptionsCtx } from '@milkdown/core';
import { listener, listenerCtx } from '@milkdown/plugin-listener';
import { Milkdown, useEditor } from '@milkdown/vue';
import { nord } from '@milkdown/theme-nord';
import { commonmark } from '@milkdown/preset-commonmark';
import { gfm } from '@milkdown/preset-gfm';
import { history } from '@milkdown/plugin-history';
import { clipboard } from '@milkdown/plugin-clipboard';
import { cursor } from '@milkdown/plugin-cursor';
import { replaceAll } from '@milkdown/utils';
import { debounce } from 'lodash-es';

const props = defineProps({
  modelValue: {
    type: String,
    default: '',
  },
});

const emit = defineEmits(['update:modelValue']);

const editorReady = ref(false);
let editorInstance: Editor | undefined;

const { editor, getInstance } = useEditor((root) => {
  const instance = Editor.make()
    .config((ctx) => {
      ctx.set(rootCtx, root);
      ctx.set(defaultValueCtx, props.modelValue);
      ctx.set(editorViewOptionsCtx, { editable: () => true });
      
      ctx.get(listenerCtx).markdownUpdated((_, markdown) => {
        if (editorReady.value && markdown !== props.modelValue) {
          debouncedUpdate(markdown);
        }
      });
    })
    .use(nord)
    .use(commonmark)
    .use(gfm)
    .use(history)
    .use(clipboard)
    .use(cursor)
    .use(listener);
  
  editorInstance = instance;
  return instance;
});

const debouncedUpdate = debounce((content: string) => {
  emit('update:modelValue', content);
}, 400);

watch(() => props.modelValue, (newValue) => {
  const instance = getInstance();
  if (instance && editorReady.value) {
    const currentContent = instance.action((ctx) => {
      return ctx.get(listenerCtx).getMarkdown();
    });
    
    if (newValue !== currentContent) {
      instance.action(replaceAll(newValue));
    }
  }
});

onMounted(() => {
  setTimeout(() => {
    editorReady.value = true;
  }, 200);
});

onUnmounted(() => {
  // Manual and delayed destruction to prevent race conditions with Vue Router
  if (editorInstance) {
    setTimeout(() => {
      getInstance()?.destroy();
    }, 0);
  }
});
</script>

<style>
/* Basic styling for the editor container */
.milkdown-container .milkdown {
  height: 100%;
  box-sizing: border-box;
}

.milkdown-container .milkdown-editor {
  padding: 1rem;
  background-color: var(--nord1);
  color: var(--nord6);
  min-height: 100%;
  outline: none;
  height: 100%;
  box-sizing: border-box;
}

.milkdown-container {
  background-color: var(--nord1);
}

/* Ensure prose styles from tailwind don't interfere too much */
.milkdown-editor .prose {
  max-width: none;
}
</style>