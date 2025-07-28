// frontend/src/stores/agent.ts
import { defineStore } from 'pinia';
import { reactive, ref } from 'vue';
import { getTaskStatus, processAgenticInstruction, linkAgentTaskToMessage, generateResearchNodeContent as apiGenerateNodeContent, restartAgentTask as apiRestartTask, refineAgentTaskSection, resumeWriteTask, stopAgentTask as apiStopAgentTask } from '../lib/api';
import type { AgentTask, Settings, ChatMessage, KnowledgeSource, ModelEndpoint, ApiConfig, researchOutlineNode } from '../types';
import { useChatStore } from './chat';
import { useSettingsStore } from './settings';
import { useUiStore } from './ui';
import { useToasts } from '../composables/useToasts';
import { v4 as uuidv4 } from 'uuid';

const INITIAL_POLL_DELAY = 2500;
const SUBSEQUENT_POLL_INTERVAL = 5000;

export const useAgentStore = defineStore('agent', () => {
    const runningTasks = reactive<Record<string, AgentTask>>({});
    const pollingIntervals = reactive<Record<string, number>>({});
    const refiningNodeIds = ref(new Set<string>());

    const stopPollingForTask = (taskId: string) => {
        if (pollingIntervals[taskId]) {
            clearInterval(pollingIntervals[taskId]);
            delete pollingIntervals[taskId];
        }
    };

    const pollTask = async (taskId: string) => {
        const chatStore = useChatStore();
        const uiStore = useUiStore();
        const agentMessage = chatStore.findMessageByAgentTaskId(taskId);
        if (!agentMessage) {
            stopPollingForTask(taskId);
            return;
        }

        const responseString = await getTaskStatus(taskId);
        if (responseString) {
            try {
                const updatedTask: AgentTask = JSON.parse(responseString);
                
                const oldTask = runningTasks[taskId];
                if (oldTask && oldTask.researchContent && updatedTask.researchContent) {
                    for (const nodeId of refiningNodeIds.value) {
                        if (JSON.stringify(oldTask.researchContent[nodeId]) !== JSON.stringify(updatedTask.researchContent[nodeId])) {
                            refiningNodeIds.value.delete(nodeId);
                        }
                    }
                }

                runningTasks[taskId] = updatedTask;

                if (agentMessage.agentTask) {
                    agentMessage.agentTask = { ...updatedTask };
                }

                if (updatedTask.status === 'completed' || updatedTask.status === 'failed' || updatedTask.status === 'awaiting_user_input') {
                    stopPollingForTask(taskId);
                    refiningNodeIds.value.clear();
                    
                    if (updatedTask.finalReport && updatedTask.status !== 'awaiting_user_input') {
                        uiStore.showArtifactPanelForTask(taskId);
                    }
                }
            } catch (e) {
                console.error("Failed to parse agent task status:", e, "Raw response:", responseString);
                stopPollingForTask(taskId);
                if (agentMessage.agentTask) {
                    agentMessage.agentTask.status = 'failed';
                    agentMessage.agentTask.finalReport = 'Failed to parse task status from the server.';
                }
            }
        } else {
            stopPollingForTask(taskId);
            if (agentMessage.agentTask) {
                agentMessage.agentTask.status = 'failed';
                agentMessage.agentTask.finalReport = 'Failed to retrieve task status from the server.';
            }
        }
    };

    const startPollingForTask = (taskId: string) => {
        stopPollingForTask(taskId);
        setTimeout(() => {
            pollTask(taskId).then(() => {
                const task = runningTasks[taskId];
                if (task && (task.status === 'running' || task.status === 'planning')) {
                    pollingIntervals[taskId] = window.setInterval(() => pollTask(taskId), SUBSEQUENT_POLL_INTERVAL);
                }
            });
        }, INITIAL_POLL_DELAY);
    };

    const startAgentTask = async (conversationId: string, instruction: string, placeholderMessage: ChatMessage, model: string | null, knowledgeSelection: string, mode: 'plan' | 'explore' | 'write' | 'research' | 'debate') => {
        const settingsStore = useSettingsStore();
        const { error } = useToasts();

        if (!settingsStore.settings) {
            error("Settings are not loaded. Cannot start agent task.");
            return;
        }

        const apiConfigForAgent: ApiConfig = JSON.parse(JSON.stringify(settingsStore.settings.apiConfig));
        
        apiConfigForAgent.knowledgeBase = settingsStore.settings.knowledgeBase;
        apiConfigForAgent.execution = settingsStore.settings.execution;
        apiConfigForAgent.appearance = settingsStore.settings.appearance;

        if (model) {
            const [providerId, modelName] = model.split('::');
            const newChatEndpoint: ModelEndpoint = { providerId, modelName };
            apiConfigForAgent.assignments.chat = newChatEndpoint;
        }
        
        const result = await processAgenticInstruction(conversationId, instruction, apiConfigForAgent, mode, knowledgeSelection);
        
        if (result && result.task_id) {
            const taskId = result.task_id;
            
            await linkAgentTaskToMessage(placeholderMessage.id, taskId);

            const chatStore = useChatStore();
            const message = chatStore.findMessageById(placeholderMessage.id);
            if (message) {
                message.agentTaskId = taskId;
                if (message.agentTask) {
                    message.agentTask.id = taskId;
                }
            }
            
            startPollingForTask(taskId);
        } else {
            const chatStore = useChatStore();
            const message = chatStore.findMessageById(placeholderMessage.id);
            if (message) {
                message.error = 'Could not initiate agent task on the backend. Please check logs.';
                message.model = 'ai';
                message.content = [{ type: 'text', text: 'Agent task failed to start.' }];
            }
        }
    };

    const resumeWriteTaskWithPlan = async (taskId: string, payload: { elaboration: any; plan: any }) => {
        const chatStore = useChatStore();
        const { info } = useToasts();

        const agentMessage = chatStore.findMessageByAgentTaskId(taskId);
        if (agentMessage && agentMessage.agentTask) {
            agentMessage.agentTask.status = 'running';
            agentMessage.agentTask.plan = payload.plan;
        }

        info('Outline confirmed. Resuming writing process...');
        await resumeWriteTask(taskId, payload.elaboration, payload.plan);
        startPollingForTask(taskId);
    };

    const restartFailedTask = async (taskId: string) => {
        const chatStore = useChatStore();
        const { info } = useToasts();

        const agentMessage = chatStore.findMessageByAgentTaskId(taskId);
        if (agentMessage && agentMessage.agentTask) {
            agentMessage.agentTask.status = 'running';
        }
        
        await chatStore.deleteSubsequentError(taskId);

        info(`Attempting to resume task...`);
        
        await apiRestartTask(taskId);
        
        stopPollingForTask(taskId);
        startPollingForTask(taskId);
    };

    const generateResearchNodeContent = async (taskId: string, nodeId: string) => {
        const chatStore = useChatStore();
        const { info } = useToasts();

        const agentMessage = chatStore.findMessageByAgentTaskId(taskId);
        if (agentMessage && agentMessage.agentTask && agentMessage.agentTask.plan) {
            const findAndSetStatus = (nodes: researchOutlineNode[]) => {
                for (const node of nodes) {
                    if (node.id === nodeId) {
                        node.status = 'writing';
                        return true;
                    }
                    if (node.steps && findAndSetStatus(node.steps)) {
                        return true;
                    }
                }
                return false;
            };
            findAndSetStatus(agentMessage.agentTask.plan);
        }

        info(`Generating content for section ${nodeId}...`);
        
        await apiGenerateNodeContent(taskId, nodeId);

        stopPollingForTask(taskId);
        startPollingForTask(taskId);
    };

    const refineSection = async (taskId: string, nodeId: string, prompt: string, model: string) => {
        const { info } = useToasts();
        info(`Refining section ${nodeId}...`);
        refiningNodeIds.value.add(nodeId);
        await refineAgentTaskSection(taskId, nodeId, prompt, model);
        stopPollingForTask(taskId);
        startPollingForTask(taskId);
    };

    const fetchInitialTaskStates = async (messages: ChatMessage[]): Promise<ChatMessage[]> => {
        const agentMessages = messages.filter(m => m.agentTaskId && m.model === 'agent');
        if (agentMessages.length === 0) {
            return messages;
        }

        const taskPromises = agentMessages.map(m => getTaskStatus(m.agentTaskId!));
        const taskResults = await Promise.all(taskPromises);

        taskResults.forEach((resultStr, index) => {
            const message = agentMessages[index];
            if (resultStr) {
                try {
                    const taskData: AgentTask = JSON.parse(resultStr);
                    message.agentTask = taskData;
                } catch (e) {
                    console.error(`Failed to parse historical agent task ${message.agentTaskId}:`, e);
                    if (message.agentTask) {
                        message.agentTask.status = 'failed';
                        message.agentTask.finalReport = 'Error: Could not load final task state.';
                    }
                }
            } else {
                console.warn(`Received null status for historical agent task ${message.agentTaskId}.`);
                if (message.agentTask) {
                    message.agentTask.status = 'failed';
                    message.agentTask.finalReport = 'Error: Could not retrieve final task state from server.';
                }
            }
        });

        return messages;
    };

    const resumePollingForActiveTasks = () => {
        const chatStore = useChatStore();
        for (const convoId in chatStore.conversations) {
            const messages = chatStore.conversations[convoId];
            messages.forEach(message => {
                if (message.agentTaskId && message.agentTask) {
                    const status = message.agentTask.status;
                    if (status === 'running' || status === 'planning') {
                        console.log(`Resuming polling for active task: ${message.agentTaskId}`);
                        startPollingForTask(message.agentTaskId);
                    }
                }
            });
        }
    };
    
    const stopAgentTask = async (taskId: string) => {
        const chatStore = useChatStore();
        const { info } = useToasts();
        info(`Stopping task ${taskId}...`);
        
        // Optimistic UI update
        const agentMessage = chatStore.findMessageByAgentTaskId(taskId);
        if (agentMessage && agentMessage.agentTask) {
            agentMessage.agentTask.status = 'failed';
            agentMessage.agentTask.finalReport = 'Task manually stopped by user.';
        }

        await apiStopAgentTask(taskId);
        stopPollingForTask(taskId);
    };

    return {
        runningTasks,
        refiningNodeIds,
        startAgentTask,
        stopAgentTask,
        resumeWriteTaskWithPlan,
        restartFailedTask,
        generateResearchNodeContent,
        refineSection,
        fetchInitialTaskStates,
        resumePollingForActiveTasks,
    };
});