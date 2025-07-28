// frontend/src/i18n.ts
import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import zh from './locales/zh.json';
import type { Settings } from './types';

export type MessageSchema = typeof en;

const getInitialLocale = (settings: Settings | null): 'en' | 'zh' => {
    const savedLang = settings?.appearance.language;
    if (savedLang && savedLang !== 'system') {
        return savedLang;
    }
    
    const browserLang = navigator.language.split('-')[0];
    if (browserLang === 'zh') {
        return 'zh';
    }
    
    return 'en';
};

export const setupI18n = (settings: Settings | null) => {
    return createI18n<[MessageSchema], 'en' | 'zh'>({
        legacy: false, // use composition API
        locale: getInitialLocale(settings),
        fallbackLocale: 'en',
        messages: {
            en,
            zh,
        },
    });
};