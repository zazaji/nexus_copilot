<template>
  <transition name="modal-fade">
    <div v-if="isVisible" class="fixed inset-0 bg-black bg-opacity-60 z-50 flex items-center justify-center" @click.self="close">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-4xl m-4 flex flex-col max-h-[90vh]">
        <header class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
          <h2 class="text-lg font-semibold">{{ isEditing ? 'Edit Action' : 'Add New Action' }}</h2>
          <button @click="close" class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700">
            <X class="w-5 h-5" />
          </button>
        </header>

        <main class="flex-1 overflow-y-auto p-6 grid grid-cols-1 md:grid-cols-2 gap-x-6">
          <!-- Left Column: Basic Info & Runtime Config -->
          <div class="space-y-6">
            <div>
              <label for="tool-name" class="block text-sm font-medium">Action Name</label>
              <input type="text" id="tool-name" v-model="editableTool.name" class="mt-1 input-style" placeholder="e.g., Get System Memory">
            </div>
            <div>
              <label for="tool-desc" class="block text-sm font-medium">Description</label>
              <textarea id="tool-desc" v-model="editableTool.description" rows="2" class="mt-1 input-style" placeholder="A short description for the AI to understand what this action does."></textarea>
            </div>
            <div>
              <label for="tool-runtime" class="block text-sm font-medium">Runtime</label>
              <select id="tool-runtime" v-model="editableTool.runtime" class="mt-1 input-style">
                  <option value="python">Python</option>
                  <option value="node">Node.js</option>
                  <option value="shell">Shell</option>
                  <option value="webhook">Webhook</option>
              </select>
            </div>

            <!-- Script Runtime Config -->
            <div v-if="isScriptRuntime" class="space-y-4 p-4 border dark:border-gray-700 rounded-md">
              <h4 class="font-medium">Script Configuration</h4>
              <div>
                <label for="tool-script" class="block text-sm font-medium">Script Path</label>
                <div class="flex items-center space-x-2 mt-1">
                  <input type="text" id="tool-script" v-model="editableTool.scriptPath" class="input-style flex-1" placeholder="/path/to/your/script.py">
                </div>
              </div>
            </div>

            <!-- Webhook Runtime Config -->
            <div v-if="editableTool.runtime === 'webhook'" class="space-y-4 p-4 border dark:border-gray-700 rounded-md">
              <h4 class="font-medium">Webhook Configuration</h4>
              <div>
                <label for="tool-webhook-url" class="block text-sm font-medium">Webhook URL</label>
                <input type="text" id="tool-webhook-url" v-model="editableTool.webhookUrl" class="mt-1 input-style" placeholder="https://api.example.com/...">
              </div>
              <div>
                <label for="tool-webhook-method" class="block text-sm font-medium">HTTP Method</label>
                <select id="tool-webhook-method" v-model="editableTool.webhookMethod" class="mt-1 input-style">
                    <option>POST</option>
                    <option>GET</option>
                    <option>PUT</option>
                    <option>PATCH</option>
                </select>
              </div>
            </div>
          </div>

          <!-- Right Column: Schema & Advanced Config -->
          <div class="space-y-6">
            <div>
              <label for="tool-input-schema" class="block text-sm font-medium">Input Schema (JSON Schema)</label>
              <p class="text-xs text-gray-500 mt-1 mb-2">Defines the parameters the AI can use. Use an empty object `{}` for actions with no parameters.</p>
              <textarea id="tool-input-schema" v-model="editableTool.inputSchema" rows="8" class="mt-1 input-style font-mono text-xs" placeholder='{\n  "type": "object",\n  "properties": {\n    "param_name": {\n      "type": "string",\n      "description": "A description of the parameter."\n    }\n  },\n  "required": ["param_name"]\n}'></textarea>
            </div>

            <div v-if="editableTool.runtime === 'webhook'" class="space-y-4">
              <div>
                <label for="tool-webhook-headers" class="block text-sm font-medium">Headers (JSON)</label>
                <textarea id="tool-webhook-headers" v-model="editableTool.webhookHeaders" rows="3" class="mt-1 input-style font-mono text-xs" placeholder='{\n  "Content-Type": "application/json",\n  "Authorization": "Bearer YOUR_API_KEY"\n}'></textarea>
              </div>
              <div>
                <label for="tool-webhook-body" class="block text-sm font-medium">Body Template (JSON)</label>
                <p class="text-xs text-gray-500 mt-1 mb-2">Use `{{param_name}}` to insert parameters from the Input Schema.</p>
                <textarea id="tool-webhook-body" v-model="editableTool.webhookBodyTemplate" rows="5" class="mt-1 input-style font-mono text-xs" placeholder='{\n  "text": "{{message}}",\n  "user": "Nexus"\n}'></textarea>
              </div>
            </div>
          </div>
        </main>

        <footer class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3 flex-shrink-0">
          <button @click="close" class="px-4 py-2 bg-gray-200 dark:bg-gray-600 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-500">{{ $t('common.cancel') }}</button>
          <button @click="save" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">{{ $t('common.save') }}</button>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ConfiguredTool } from '../../types';
import { X } from 'lucide-vue-next';
import { v4 as uuidv4 } from 'uuid';
import { useToasts } from '../../composables/useToasts';

const props = defineProps<{
  modelValue: boolean;
  tool: ConfiguredTool | null;
}>();

const emit = defineEmits(['update:modelValue', 'save']);

const { t } = useI18n();
const { error } = useToasts();

const isVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEditing = computed(() => !!props.tool);

const createDefaultTool = (): ConfiguredTool => ({
  id: uuidv4(),
  name: '',
  description: '',
  runtime: 'python',
  scriptPath: '',
  webhookUrl: '',
  webhookMethod: 'POST',
  webhookHeaders: '{\n  "Content-Type": "application/json"\n}',
  webhookBodyTemplate: '{\n  "text": "{{user_prompt}}"\n}',
  inputSchema: '{\n  "type": "object",\n  "properties": {},\n  "required": []\n}',
  parameters: [], // This is now deprecated in favor of inputSchema, but kept for DB model compatibility
  showInCopilot: false,
  isFavorite: false,
  inputSource: 'user_input',
  requiresAiPreProcessing: false,
  preProcessingPrompt: '',
  outputHandling: 'raw_text',
  requiresAiPostProcessing: false,
  postProcessingPrompt: '',
});

const editableTool = ref<ConfiguredTool>(createDefaultTool());

const isScriptRuntime = computed(() => {
  const runtime = editableTool.value.runtime;
  return runtime === 'python' || runtime === 'node' || runtime === 'shell';
});

watch(() => props.tool, (newTool) => {
  if (newTool) {
    editableTool.value = { ...createDefaultTool(), ...JSON.parse(JSON.stringify(newTool)) };
  } else {
    editableTool.value = createDefaultTool();
  }
}, { immediate: true });

const close = () => {
  isVisible.value = false;
};

const validateJson = (jsonString: string | undefined, fieldName: string): boolean => {
  if (!jsonString || !jsonString.trim()) return true; // Empty is valid
  try {
    JSON.parse(jsonString);
    return true;
  } catch (e) {
    error(`${fieldName} is not valid JSON.`);
    return false;
  }
};

const save = () => {
  // Validate JSON fields before saving
  if (!validateJson(editableTool.value.inputSchema, 'Input Schema')) return;
  if (editableTool.value.runtime === 'webhook') {
    if (!validateJson(editableTool.value.webhookHeaders, 'Headers')) return;
    if (!validateJson(editableTool.value.webhookBodyTemplate, 'Body Template')) return;
  }

  // Clean up fields based on runtime
  if (isScriptRuntime.value) {
    editableTool.value.webhookUrl = undefined;
    editableTool.value.webhookMethod = undefined;
    editableTool.value.webhookHeaders = undefined;
    editableTool.value.webhookBodyTemplate = undefined;
  } else {
    editableTool.value.scriptPath = undefined;
  }

  emit('save', editableTool.value);
  close();
};
</script>

<style scoped>
.input-style {
  @apply block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm bg-transparent;
}
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>