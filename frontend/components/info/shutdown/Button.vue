<template>
  <UButton @click="shutdownModal = true" v-bind="$attrs" :ui="{ truncate: 'hidden md:block' }" truncate />
  <InfoShutdownModal v-model="shutdownModal" />
</template>

<script setup lang="ts">
const shutdownModal = ref(false)
</script>
