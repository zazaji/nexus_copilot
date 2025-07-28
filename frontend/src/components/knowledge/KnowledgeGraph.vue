<!-- frontend/src/components/knowledge/KnowledgeGraph.vue -->
<template>
  <div class="flex-1 flex flex-col bg-white dark:bg-gray-800 overflow-hidden relative">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <h3 class="font-semibold">{{ $t('knowledge.graphTitle') }}</h3>
      <p class="text-sm text-gray-500">{{ $t('knowledge.graphDesc') }}</p>
    </div>
    <div class="flex-1 relative" ref="graphContainerRef">
      <div v-if="isLoading" class="absolute inset-0 flex items-center justify-center bg-white/50 dark:bg-gray-800/50 z-10">
        <Loader2 class="w-8 h-8 animate-spin text-blue-500" />
      </div>
      <div v-else-if="!kbStore.graphData || kbStore.graphData.nodes.length === 0" class="absolute inset-0 flex items-center justify-center">
        <p class="text-gray-500">{{ $t('knowledge.graphEmpty') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue';
import ForceGraph3D from '3d-force-graph';
import { Sprite, SpriteMaterial, CanvasTexture } from 'three';
import { useKnowledgeBaseStore } from '../../stores/knowledgeBase';
import { useKnowledgeExplorerStore } from '../../stores/knowledgeExplorer';
import { useSettingsStore } from '../../stores/settings';
import { Loader2 } from 'lucide-vue-next';

const kbStore = useKnowledgeBaseStore();
const explorerStore = useKnowledgeExplorerStore();
const settingsStore = useSettingsStore();
const graphContainerRef = ref<HTMLElement | null>(null);
const isLoading = ref(false);
let graphInstance: any = null;
let resizeObserver: ResizeObserver | null = null;

const isDarkMode = computed(() => {
    const theme = settingsStore.settings?.appearance.theme;
    if (theme === 'dark') return true;
    if (theme === 'system') return window.matchMedia('(prefers-color-scheme: dark)').matches;
    return false;
});

const createTextSprite = (node: any) => {
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('2d');
    if (!context) return new Sprite();

    const fontSize = 32;
    context.font = `Bold ${fontSize}px Sans-serif`;
    
    const label = node.label;
    const textWidth = context.measureText(label).width;
    const canvasWidth = textWidth + 4; // Add a little padding for the outline
    const canvasHeight = fontSize + 4;

    canvas.width = canvasWidth;
    canvas.height = canvasHeight;

    context.textAlign = 'center';
    context.textBaseline = 'middle';

    // Create a subtle outline for better visibility on any background
    const outlineColor = isDarkMode.value ? 'rgba(0, 0, 0, 0.8)' : 'rgba(255, 255, 255, 0.8)';
    context.fillStyle = outlineColor;
    // context.fillText(label, canvasWidth / 2 + 1, canvasHeight / 2 + 1);
    // context.fillText(label, canvasWidth / 2 - 1, canvasHeight / 2 - 1);
    // context.fillText(label, canvasWidth / 2 + 1, canvasHeight / 2 - 1);
    // context.fillText(label, canvasWidth / 2 - 1, canvasHeight / 2 + 1);

    // Main text
    context.fillStyle = node.type === 'ghost' 
        ? (isDarkMode.value ? '#f87171' : '#ef4444') 
        : (isDarkMode.value ? '#e5e7eb' : '#1f2937');
    context.fillText(label, canvasWidth / 2, canvasHeight / 2);

    const texture = new CanvasTexture(canvas);
    const material = new SpriteMaterial({ map: texture, depthWrite: false, transparent: true });
    const sprite = new Sprite(material);
    
    const scaleFactor = 0.5;
    sprite.scale.set(canvasWidth * scaleFactor, canvasHeight * scaleFactor, 1.0);

    return sprite;
};

const updateGraphStyles = () => {
    if (!graphInstance) return;

    const linkColor = isDarkMode.value ? 'rgba(255, 255, 255, 1)' : 'rgba(0, 0, 0, 1)';

    graphInstance
        .linkColor(() => linkColor)
        .linkWidth(1) 
        .linkDirectionalParticles(0) 
        .nodeThreeObject((node: any) => createTextSprite(node));
};


const initializeGraph = () => {
  if (!graphContainerRef.value || !kbStore.graphData) {
    return;
  }

  const { nodes, links } = kbStore.graphData;
  
  graphInstance = ForceGraph3D()(graphContainerRef.value)
    .graphData({ nodes, links })
    .nodeLabel('')
    .onNodeClick((node: any) => {
      if (node && node.id && !node.id.startsWith('ghost::')) {
        explorerStore.selectFile(node.id);
      }
    })
    .width(graphContainerRef.value.clientWidth)
    .height(graphContainerRef.value.clientHeight);

    const force = graphInstance.d3Force('charge');
    if (force) {
        force.strength(-120);
    }
    const linkForce = graphInstance.d3Force('link');
    if (linkForce) {
        linkForce.strength(0.2).distance(50);
    }
    const centerForce = graphInstance.d3Force('center');
    if (centerForce) {
        centerForce.strength(0.1);
    }

    updateGraphStyles();
};

const handleResize = () => {
  if (graphInstance && graphContainerRef.value) {
    graphInstance.width(graphContainerRef.value.clientWidth);
    graphInstance.height(graphContainerRef.value.clientHeight);
  }
};

onMounted(async () => {
  isLoading.value = true;
  await kbStore.fetchGraphData();
  await nextTick();
  initializeGraph();
  isLoading.value = false;

  if (graphContainerRef.value) {
    resizeObserver = new ResizeObserver(handleResize);
    resizeObserver.observe(graphContainerRef.value);
  }
});

onUnmounted(() => {
  if (resizeObserver && graphContainerRef.value) {
    resizeObserver.unobserve(graphContainerRef.value);
  }
  if (graphInstance && typeof graphInstance._destructor === 'function') {
    graphInstance._destructor();
  }
});

watch(() => kbStore.graphData, (newData) => {
  if (graphInstance && newData) {
    graphInstance.graphData(newData);
  }
}, { deep: true });

watch(isDarkMode, () => {
    if (graphInstance) {
        updateGraphStyles();
    }
});
</script>