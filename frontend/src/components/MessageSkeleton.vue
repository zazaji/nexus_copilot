<!-- frontend/src/components/MessageSkeleton.vue -->
<template>
  <div class="flex justify-start animate-pulse">
    <div class="flex items-start space-x-3 max-w-2xl">
      <div class="w-10 h-10 rounded-full bg-gray-200 dark:bg-gray-700 flex-shrink-0"></div>
      <div class="p-3 rounded-lg bg-gray-200 dark:bg-gray-700 space-y-2 w-64">
        <div class="h-2 bg-gray-300 dark:bg-gray-600 rounded w-5/6"></div>
        <div class="h-2 bg-gray-300 dark:bg-gray-600 rounded w-full"></div>
        <div class="h-2 bg-gray-300 dark:bg-gray-600 rounded w-3/4"></div>
      </div>
    </div>
  </div>
</template>