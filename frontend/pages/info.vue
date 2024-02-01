<script setup lang="ts">
import { ref } from 'vue';

const { data: info } = await useFetch<{
  version: string;
  name: string;
}>('/proxy/gateway/pal/info', {
  parseResponse: (text) => {
    const regex = /\[v([\d\.]+)\] (.+)/g;
    const matches = regex.exec(text);
    if (matches === null) {
      return undefined;
    } else {
      const version = matches[1];
      const name = matches[2];
      return { version, name };
    }
  }
})

</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-2">
      <div class="flex gap-2">
        <UIcon class="block h-20 w-20" name="i-heroicons-server-stack" />
        <div class="flex flex-col justify-center">
          <p v-if="!info">Loading...</p>
          <h1 v-if="info" class="text-4xl font-bold">{{ info.name }}</h1>
          <p v-if="info" class="text-gray-500 dark:text-gray-400">Version: {{ info.version }}</p>
        </div>
      </div>
      <div class="flex justify-end px-3 gap-2">
        <UButton color="primary" variant="solid" label="Broadcast" icon="i-heroicons-chat-bubble-bottom-center-text" />
        <InfoUpdateButton />
        <InfoShutdownButton color="red" variant="solid" label="Shutdown" icon="i-heroicons-power" />
      </div>
    </div>
    <!-- <UDivider /> -->
    <div class="">
      <h1 class="text-2xl">Patch Notes</h1>
      <InfoPatchNotes :pageCount="8" />
    </div>
  </div>
</template>
