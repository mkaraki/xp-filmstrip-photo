// https://nuxt.com/docs/api/configuration/nuxt-config
import pkg from './package.json'

// Get current build time in YYYY.MMDD.HHMMSS format
const now = new Date();
const pad = (n: number) => n.toString().padStart(2, '0');
const buildTime = `${now.getFullYear()}.${pad(now.getMonth() + 1)}${pad(now.getDate())}.${pad(now.getHours())}${pad(now.getMinutes())}${pad(now.getSeconds())}`;

export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  ssr: false,
  modules: ['@nuxtjs/i18n'],
  i18n: {
    locales: [
      { code: 'en', file: 'en.json', name: 'English' },
      { code: 'ja', file: 'ja.json', name: 'Japanese' }
    ],
    defaultLocale: 'en',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    },
    langDir: 'locales'
  },
  runtimeConfig: {
    public: {
      appVersion: pkg.version,
      buildTime: buildTime
    }
  },
  // Map internal paths to safe filenames
  hooks: {
    'pages:extend'(pages) {
      pages.push({
        name: 'custom-slideshow',
        path: '/.__slideshow/:slug*',
        file: '~/pages/slideshow/[...slug].vue'
      });
      pages.push({
        name: 'custom-about',
        path: '/.__about',
        file: '~/pages/about-internal.vue'
      });
    }
  }
})
