import { createProxyEventHandler } from "h3-proxy"

export default defineEventHandler(
  createProxyEventHandler({
    target: `http://${useRuntimeConfig().gatewayAddr}`,
    changeOrigin: true,
    pathRewrite: {
      "^/proxy/gateway/": "/",
    },
    pathFilter: ["/proxy/gateway/"],
  }),
)
