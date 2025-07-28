// frontend/src/main.ts
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import naive from 'naive-ui';
import App from './App.vue';
import router from './router';
import './assets/main.css';
import { getUserSettings } from './lib/api';
import { applyTheme } from './utils/theme';
import { useClipboardStore } from './stores/clipboard';
import { setupI18n } from './i18n';
import type { I18n } from 'vue-i18n';

export let i18n: I18n<any, any, any, any, false>;

// --- Pre-Vue Initialization ---
async function initialize() {
  try {
    const settings = await getUserSettings();
    if (settings) {
      applyTheme(settings.appearance.theme);
    } else {
      applyTheme('system');
    }
    i18n = setupI18n(settings);
  } catch (e) {
    console.error("Failed to initialize settings:", e);
    applyTheme('system');
    i18n = setupI18n(null);
  }

  const pinia = createPinia();
  const app = createApp(App);

  app.use(pinia);
  app.use(router); // Router is now used by the main app view within App.vue
  app.use(naive);
  app.use(i18n);
  app.mount('#app');

  // Initialize clipboard history after app is mounted
  const clipboardStore = useClipboardStore();
  clipboardStore.initialize();
}

initialize();