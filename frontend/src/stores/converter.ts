// frontend/src/stores/converter.ts
import { defineStore } from 'pinia';
import { ref, onMounted, onUnmounted } from 'vue';
import { openDirectoryPicker, batchConvertToMarkdown, onConversionProgress } from '../lib/api';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { useToasts } from '../composables/useToasts';

export const useConverterStore = defineStore('converter', () => {
    const inputDir = ref('');
    const outputDir = ref('');
    const isConverting = ref(false);
    const progress = ref(0);
    const progressMessage = ref('');
    let unlisten: UnlistenFn | null = null;
    const { success, error } = useToasts();

    async function selectInputDirectory() {
        const result = await openDirectoryPicker();
        if (result?.success && result.path) {
            inputDir.value = result.path;
        }
    }

    async function selectOutputDirectory() {
        const result = await openDirectoryPicker();
        if (result?.success && result.path) {
            outputDir.value = result.path;
        }
    }

    async function startConversion() {
        if (!inputDir.value || !outputDir.value) {
            alert('Please select both input and output directories.');
            return;
        }

        isConverting.value = true;
        progress.value = 0;
        progressMessage.value = 'Starting conversion process...';

        await batchConvertToMarkdown(inputDir.value, outputDir.value);
    }

    onMounted(async () => {
        unlisten = await onConversionProgress((payload) => {
            progress.value = payload.progress;
            progressMessage.value = payload.message;
            if (payload.error) {
                error(payload.message);
            }
            if (payload.progress >= 100) {
                isConverting.value = false;
                if (!payload.error) {
                    success('Conversion complete!');
                }
            }
        });
    });

    onUnmounted(() => {
        unlisten?.();
    });

    return {
        inputDir,
        outputDir,
        isConverting,
        progress,
        progressMessage,
        selectInputDirectory,
        selectOutputDirectory,
        startConversion,
    };
});