import { useLang, withBase } from 'rspress/runtime';

import { useCallback } from 'react';
import { EN_US } from './en_us';
import { ZH_CN } from './zh_cn';

const translations = {
    en: EN_US,
    zh: ZH_CN,
} as const;


export function useUrl(url: string) {
    const lang = useLang();
    return withBase(lang === 'zh' ? `/zh${url}` : url);
}

export function useI18nUrl() {
    const lang = useLang();

    const tUrl = useCallback(
        (url: string) => {
        return withBase(lang === 'en' ? url : `/${lang}${url}`);
        },
        [lang],
    );

    return tUrl;
}

export function useI18n() {
    const lang = useLang() as keyof typeof translations;
    return (key: keyof typeof EN_US) => translations[lang][key];
}