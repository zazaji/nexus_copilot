// frontend/src/router/index.ts
import { createRouter, createWebHashHistory } from 'vue-router';
import MainLayout from '../views/MainLayout.vue';

const routes = [
  {
    path: '/',
    component: MainLayout,
    redirect: '/dashboard',
    children: [
      { 
        path: 'dashboard', 
        name: 'Dashboard', 
        component: () => import('../views/DashboardView.vue') 
      },
      { 
        path: 'chat/:id?', 
        name: 'Chat', 
        component: () => import('../views/ChatView.vue'), 
        props: true 
      },
      { 
        path: 'creation/:id', 
        name: 'Creation', 
        component: () => import('../views/CreationView.vue'), // New View
        props: true 
      },
      { 
        path: 'knowledge-base', 
        name: 'KnowledgeBase', 
        component: () => import('../views/KnowledgeBaseView.vue') 
      },
      { 
        path: 'tools', 
        name: 'Tools', 
        component: () => import('../views/ToolsView.vue') 
      },
      { 
        path: 'settings', 
        name: 'Settings', 
        component: () => import('../views/SettingsView.vue') 
      },
      { 
        path: 'clipboard', 
        name: 'Clipboard', 
        component: () => import('../views/ClipboardHistoryView.vue') 
      },
    ],
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;