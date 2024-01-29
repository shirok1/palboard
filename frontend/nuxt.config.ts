// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  runtimeConfig: {
    gatewayAddr: "127.0.0.1:8080",
    public: {
      version: process.env.VERSION ?? "unknown"
    }
  },
  devtools: { enabled: true },
  modules: ['@nuxt/ui', '@pinia/nuxt'],
  routeRules: {
    '/proxy/steamdb/**': { proxy: { to: "https://steamdb.info/api/**" } },
  }
})
