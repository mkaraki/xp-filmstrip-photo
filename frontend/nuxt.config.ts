// https://nuxt.com/docs/api/configuration/nuxt-config
import pkg from './package.json'

export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  ssr: false,
  app: {
    head: {
      title: 'Film Strip Image Viewer',
    }
  },
  runtimeConfig: {
    public: {
      appVersion: pkg.version
    }
  },
  devServer: {
    port: 3000,
    proxy: {
      '/.__api': {
        target: 'http://localhost:8080',
        changeOrigin: true
      }
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
