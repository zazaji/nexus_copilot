<!-- frontend/src/components/WelcomeGuide.vue -->
<template>
  <div class="flex flex-col items-center justify-center h-full text-center text-gray-500 dark:text-gray-400">
    <Bot v-if="mode === 'chat'" class="w-24 h-24 mb-4 text-gray-400" />
    <Sparkles v-else class="w-24 h-24 mb-4 text-gray-400" />
    
    <h1 class="text-2xl font-semibold">{{ title }}</h1>
    <p class="mt-2 max-w-3xl">{{ description }}</p>

    <div class="mt-8 w-full max-w-4xl grid grid-cols-1 md:grid-cols-2 gap-4">
      <div 
        v-for="example in examples" 
        :key="example.title"
        @click="$emit('selectExample', example.prompt)"
        class="p-4 bg-white dark:bg-gray-800/50 rounded-lg shadow-sm hover:shadow-md transition-shadow cursor-pointer text-left"
      >
        <p class="font-semibold text-sm text-gray-700 dark:text-gray-300">{{ example.title }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1 truncate">{{ example.prompt }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, PropType } from 'vue';
import { Bot, Sparkles } from 'lucide-vue-next';

const props = defineProps({
  mode: {
    type: String as PropType<'chat' | 'creation'>,
    required: true,
  }
});

defineEmits(['selectExample']);

const title = computed(() => {
  return props.mode === 'chat' ? 'Nexus' : '开始创作';
});

const description = computed(() => {
  return props.mode === 'chat' 
    ? '我能为您做些什么？您可以直接提问，或使用下面的引导模式来处理复杂任务。'
    : '选择一个示例，或在下方的输入框中描述您想创作的内容。';
});

const examples = computed(() => {
  if (props.mode === 'chat') {
    return [
      { title: '问答', prompt: '解释一下什么是量子纠缠？' },
      { title: '规划模式 (Plan)', prompt: '/plan 特斯拉股票值得买吗' },
      { title: '探索模式 (Explore)', prompt: '/explore 帮我调研一下最近AI领域开源的图像生成模型' },
      { title: '写作模式 (Write)', prompt: '/write 写一篇关于“可持续发展”的博客文章，不少于3000字' },
      { title: '研究模式 (Research)', prompt: '/research 撰写一份关于“未来城市交通”的深度研究报告，不少于1万字' },
      { title: '对抗模式 (Debate)', prompt: '/debate AI会给人类的未来带来灾难吗？' },
    ];
  }
  return [
    { title: '生成一张图片', prompt: '一只穿着宇航服的猫在火星上喝咖啡，数字艺术风格' },
    { title: '生成一张图片', prompt: '赛博朋克，高楼、美女、空中飞铁，高质量照片' },
    { title: '生成一段音频', prompt: '今天天气真不错，阳光明媚，万里无云。' },
    { title: '生成另一段音频', prompt: '人工智能正在深刻地改变我们的世界。' },
  ];
});
</script>