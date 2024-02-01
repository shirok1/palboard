<template>
  <UButton @click="shutdownModal = true" v-bind="$attrs" />
  <InfoShutdownModal v-model="shutdownModal" />
</template>

<script setup lang="ts">
const shutdownModal = ref(false)
</script>
