// frontend/src/stores/filePreview.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { readFileContent } from '../lib/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';

type ContentType = 'markdown' | 'text' | 'image' | 'pdf' | 'auto';

interface ShowPreviewOptions {
  contentType?: ContentType;
  content?: string; // Allow passing content directly
}

export const useFilePreviewStore = defineStore('filePreview', () => {
  const isVisible = ref(false);
  const isLoading = ref(false);
  const filePath = ref<string | null>(null); // Will hold the original path or name
  const fileContent = ref<string | null>(null); // Will hold the content (text, data URL, or asset URL)
  const error = ref<string | null>(null);
  const contentType = ref<ContentType>('auto');
  const sourceUrl = ref<string | null>(null); // To store the original URL if content is fetched

  const isImage = computed(() => contentType.value === 'image');
  const isPdf = computed(() => contentType.value === 'pdf');
  const isMarkdown = computed(() => contentType.value === 'markdown');
  const isText = computed(() => contentType.value === 'text');

  async function showPreview(pathOrUrl: string, options: ShowPreviewOptions = {}) {
    isVisible.value = true;
    isLoading.value = true;
    error.value = null;
    fileContent.value = null;
    filePath.value = pathOrUrl; // Store the original identifier
    sourceUrl.value = null;
    contentType.value = options.contentType || 'auto';

    // Case 1: Content is passed directly (e.g., for online KB snippets)
    if (options.content) {
        fileContent.value = options.content;
        isLoading.value = false;
        return;
    }

    // Case 2: It's a Data URL (e.g., base64 image)
    if (pathOrUrl.startsWith('data:')) {
        fileContent.value = pathOrUrl;
        if (pathOrUrl.startsWith('data:image/')) contentType.value = 'image';
        isLoading.value = false;
        return;
    }

    // Case 3: It's an HTTP/HTTPS URL
    if (pathOrUrl.startsWith('http://') || pathOrUrl.startsWith('https://')) {
        sourceUrl.value = pathOrUrl;
        // Check if it's an image URL
        if (/\.(png|jpg|jpeg|gif|webp)$/i.test(pathOrUrl)) {
            contentType.value = 'image';
            fileContent.value = pathOrUrl;
            isLoading.value = false;
            return;
        }
        // Otherwise, assume it's text content to be fetched
        try {
            const response = await fetch(pathOrUrl);
            if (response.ok) {
                fileContent.value = await response.text();
                contentType.value = 'markdown'; // Assume logs and reports are markdown
            } else {
                error.value = `Failed to fetch content: ${response.status} ${response.statusText}`;
            }
        } catch (e: any) {
            error.value = `Error fetching URL: ${e.toString()}`;
        }
        isLoading.value = false;
        return;
    }

    // Case 4: It's a local file path
    const path = pathOrUrl;
    if (contentType.value === 'auto') {
        if (/\.(png|jpg|jpeg|gif|webp)$/i.test(path)) contentType.value = 'image';
        else if (/\.pdf$/i.test(path)) contentType.value = 'pdf';
        else contentType.value = 'markdown'; // Default to markdown/text
    }

    if (contentType.value === 'image' || contentType.value === 'pdf') {
        fileContent.value = convertFileSrc(path);
    } else {
        const content = await readFileContent(path);
        if (content !== null) {
          fileContent.value = content;
        } else {
          error.value = `Failed to read file: ${path}`;
        }
    }
    isLoading.value = false;
  }

  function hidePreview() {
    isVisible.value = false;
    filePath.value = null;
    fileContent.value = null;
    error.value = null;
    sourceUrl.value = null;
    contentType.value = 'auto';
  }

  return {
    isVisible,
    isLoading,
    filePath,
    fileContent,
    error,
    contentType,
    sourceUrl,
    isImage,
    isPdf,
    isMarkdown,
    isText,
    showPreview,
    hidePreview,
  };
});