<template>
  <div>
    <h1>Welcome to the homepage of PalBoard</h1>
    <p v-if="version == gatewayVersion">Frontend & Gateway version: {{ version }}</p>
    <span v-if="version != gatewayVersion">
      <p>Frontend version: {{ version }}</p>
      <p>Gateway version: {{ gatewayVersion }}</p>
    </span>
    <p>PalBoard is released by shirok1 under <a class="underline" target="_blank" href="https://github.com/shirok1/palboard/blob/main/LICENSE">SSPL-1.0</a>.</p>
  </div>
</template>

<script setup lang="ts">
const { public: { version } } = useRuntimeConfig()
const { data: gatewayVersion } = await useFetch<string>('/proxy/gateway/version')
</script>
  