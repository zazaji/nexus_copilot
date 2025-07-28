// frontend/src/i18n/index.ts
import { createI18n } from 'vue-i18n';
import type { Settings } from '../types';

import en from './locales/en.json';
import zh from './locales/zh.json';

export function setupI18n(settings: Settings | null) {
  let locale = 'en';
  const userLangSetting = settings?.appearance.language;
  const browserLang = navigator.language.split('-')[0];

  if (userLangSetting && userLangSetting !== 'system') {
    locale = userLangSetting;
  } else {
    locale = browserLang === 'zh' ? 'zh' : 'en';
  }

  return createI18n({
    legacy: false,
    locale: locale,
    fallbackLocale: 'en',
    messages: {
      en,
      zh,
    },
  });
}