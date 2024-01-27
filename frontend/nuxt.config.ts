// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ['@nuxt/ui', '@pinia/nuxt'],
  routeRules: {
    '/proxy/steamdb/**': { proxy: { to: "https://steamdb.info/api/**" } },
    '/proxy/gateway/**': { proxy: { to: `http://${process.env.GATEWAY_ADDR ?? "127.0.0.1:8080"}/**` } },
  }
})
