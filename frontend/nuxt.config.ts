// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  ssr: false, // For simpler image viewer
  app: {
    head: {
      title: 'Film Strip Image Viewer',
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
  }
})
