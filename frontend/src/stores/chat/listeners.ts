// frontend/src/stores/chat/listeners.ts
import { onMounted, onUnmounted, ToRefs } from 'vue';
import type { UnlistenFn } from '@tauri-apps/api/event';
import {
    onConversationTitleUpdated,
    onChatMessageChunk,
    onChatMessageEnd,
    onCodeExecutionOutput,
    onCodeExecutionComplete,
} from '../../lib/api';
import type { ReturnType_useChatState } from './index';
import type { ChatMessage } from '../../types';

type ChatActions = {
  findMessageById: (id: string) => ChatMessage | undefined;
  addAndSaveMessage: (message: ChatMessage) => Promise<void>;
  handleInteractionCompletion: (message: ChatMessage) => Promise<void>;
};

export function setupChatListeners(
  state: ToRefs<ReturnType_useChatState>,
  actions: ChatActions
) {
  const { conversationList, isLoading, streamingMessageId } = state;
  const { findMessageById, handleInteractionCompletion } = actions;

  let unlistenTitleUpdated: UnlistenFn | null = null;
  let unlistenStreamChunk: UnlistenFn | null = null;
  let unlistenStreamEnd: UnlistenFn | null = null;
  let unlistenCodeOutput: UnlistenFn | null = null;
  let unlistenCodeComplete: UnlistenFn | null = null;

  onMounted(async () => {
    unlistenTitleUpdated = await onConversationTitleUpdated((updatedConvo) => {
      const existing = conversationList.value.find(c => c.id === updatedConvo.id);
      if (existing) {
        existing.title = updatedConvo.title;
      }
    });

    unlistenStreamChunk = await onChatMessageChunk(({ messageId, chunk }) => {
        const message = findMessageById(messageId);
        if (!message) return;

        // Initialize transient state properties if they don't exist
        message.isThinking = message.isThinking ?? false;
        message.thinkingProcess = message.thinkingProcess ?? [];
        if (message.content.length === 0 || message.content[0].type !== 'text') {
            message.content.unshift({ type: 'text', text: '' });
        }
        const textContentPart = message.content[0] as { type: 'text', text: string };

        let reasoningContent: string | null = null;
        let newContent: string | null = null;

        try {
            const parsedData = JSON.parse(chunk);
            const delta = parsedData.choices?.[0]?.delta;
            if (delta) {
                reasoningContent = delta.reasoning_content;
                newContent = delta.content;
            }
        } catch (e) {
            newContent = chunk;
        }

        // Priority 1: Handle structured reasoning content
        if (reasoningContent) {
            message.isThinking = true;
            if (message.thinkingProcess.length === 0) message.thinkingProcess.push('');
            message.thinkingProcess[message.thinkingProcess.length - 1] += reasoningContent;
        }

        if (!newContent) return;

        // Priority 2: Handle text content with tag-based state machine
        // Append new content to the currently active buffer
        if (message.isThinking && !reasoningContent) { // Append to thought only if not handled by reasoningContent
            if (message.thinkingProcess.length === 0) message.thinkingProcess.push('');
            message.thinkingProcess[message.thinkingProcess.length - 1] += newContent;
        } else {
            textContentPart.text += newContent;
        }

        // Process buffers for state transitions (e.g., finding <think> tags)
        let bufferProcessed = true;
        while(bufferProcessed) {
            bufferProcessed = false;
            if (message.isThinking) {
                const currentThought = message.thinkingProcess[message.thinkingProcess.length - 1];
                const endTagMatch = currentThought.match(/<\/(think|thinking)>/);
                if (endTagMatch && endTagMatch.index !== undefined) {
                    const thoughtPart = currentThought.substring(0, endTagMatch.index);
                    const remainingText = currentThought.substring(endTagMatch.index + endTagMatch[0].length);
                    
                    message.thinkingProcess[message.thinkingProcess.length - 1] = thoughtPart;
                    textContentPart.text += remainingText;
                    message.isThinking = false;
                    bufferProcessed = true; // There might be another <think> tag in the remainingText
                }
            } else {
                const currentText = textContentPart.text;
                const startTagMatch = currentText.match(/<(think|thinking)>/);
                if (startTagMatch && startTagMatch.index !== undefined) {
                    const textPart = currentText.substring(0, startTagMatch.index);
                    const remainingThought = currentText.substring(startTagMatch.index + startTagMatch[0].length);

                    textContentPart.text = textPart;
                    message.thinkingProcess.push(remainingThought);
                    message.isThinking = true;
                    bufferProcessed = true; // There might be a </think> tag in the remainingThought
                }
            }
        }
    });

    unlistenStreamEnd = await onChatMessageEnd(({ messageId, finalMessage }) => {
        const message = findMessageById(messageId);
        if (message) {
            message.suggestions = finalMessage.suggestions;
            message.sources = finalMessage.sources;
            message.error = finalMessage.error;
            if (finalMessage.content) {
                message.content = [{ type: 'text', text: finalMessage.content }];
            }
            // Clean up transient state
            delete message.thinkingProcess;
            delete message.isThinking;
            handleInteractionCompletion(message);
        }
        isLoading.value = false;
        streamingMessageId.value = null;
    });

    unlistenCodeOutput = await onCodeExecutionOutput(({ executionId, chunk }) => {
        const message = findMessageById(executionId);
        if (message) {
            message.executionOutput = (message.executionOutput || '') + chunk;
        }
    });

    unlistenCodeComplete = await onCodeExecutionComplete(({ executionId, status, error }) => {
        const message = findMessageById(executionId);
        if (message) {
            message.isExecuting = false;
            if (status === 'error') {
                message.executionOutput = (message.executionOutput || '') + `\n\n--- EXECUTION FAILED ---\n${error}`;
            }
        }
    });
  });

  onUnmounted(() => {
    unlistenTitleUpdated?.();
    unlistenStreamChunk?.();
    unlistenStreamEnd?.();
    unlistenCodeOutput?.();
    unlistenCodeComplete?.();
  });
}