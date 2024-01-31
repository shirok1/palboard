<script setup lang="ts">
import { ref } from 'vue';
import { extractFromXml, type FeedData, type FeedEntry } from '@extractus/feed-extractor'

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

const logs = ref<string>("")
const updateModal = ref(false)
const updateOkDisabled = ref(true)
const update_steam = async () => {
  logs.value = ""
  updateModal.value = true
  updateOkDisabled.value = true
  const ws = await new Promise<WebSocket>((resolve, reject) => {
    const url = "ws://localhost:1145/steam/update"
    const ws = new WebSocket(url) // TODO: fix websocket proxy
    ws.onopen = () => resolve(ws)
    ws.onerror = (e) => reject(e)
  })
  ws.onmessage = async ({ data }) => {
    if (data instanceof Blob) {
      const text = await data.text();
      logs.value += text;
    } else {
      // Handle other data types
      console.log("Received non blob data:", data);
    }
  }
  await new Promise((resolve) => ws.onclose = () => resolve(null))
  updateOkDisabled.value = false
}

const { data: serverUpdates } = await useFetch<FeedEntry[]>(
  '/proxy/steamdb/PatchnotesRSS/?appid=2394010', {
  parseResponse: (text) => extractFromXml(text).entries?.map((e) => ({
    ...e,
    ...e.published && { published: new Date(e.published) },
  })),
})
const { data: clientUpdates } = await useFetch<FeedEntry[]>(
  '/proxy/steamdb/PatchnotesRSS/?appid=1623730', {
  parseResponse: (text) => extractFromXml(text).entries?.map((e) => ({
    ...e,
    ...e.published && { published: new Date(e.published) },
  })),
})
const updates = computed(() =>
  [...clientUpdates.value ?? [], ...serverUpdates.value ?? []]
    .sort((a, b) => b.published - a.published))

const page = ref(1)
const pageCount = 8
const rows = computed(() => {
  return updates.value?.slice((page.value - 1) * pageCount, (page.value) * pageCount)
})

const shutdownModal = ref(false)
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
        <UButton @click="update_steam" color="primary" variant="outline" label="Update"
          icon="i-heroicons-arrow-path-rounded-square" />
        <UModal v-model="updateModal">
          <UCard :ui="{ ring: '', divide: 'divide-y divide-gray-100 dark:divide-gray-800' }">
            <template #header>
              Updating Steam...
            </template>
            <div class="flex flex-col gap-2">
              <UTextarea v-model="logs" disabled autoresize placeholder="Waiting..." />
            </div>
            <template #footer>
              <div class="flex gap-2 justify-end">
                <UButton :disabled="updateOkDisabled" @click="updateModal = false"
                  :color="updateOkDisabled ? 'gray' : 'primary'" variant="solid" label="OK" />
              </div>
            </template>
          </UCard>
        </UModal>
        <UButton @click="shutdownModal = true" color="red" variant="solid" label="Shutdown" icon="i-heroicons-power" />
        <UModal v-model="shutdownModal">
          <UCard :ui="{ ring: '', divide: 'divide-y divide-gray-100 dark:divide-gray-800' }">
            <template #header>
              How would you like to shutdown the server?
            </template>
            <div class="flex flex-col gap-2">
              <UFormGroup label="Time before shutdown">
                <UInput placeholder="0 => /DoExit" icon="i-heroicons-clock">
                  <template #trailing>
                    <span class="text-gray-500 dark:text-gray-400 text-xs">seconds</span>
                  </template>
                </UInput>
              </UFormGroup>
              <UFormGroup label="Broadcast message">
                <UInput placeholder="Optional" icon="i-heroicons-chat-bubble-bottom-center-text" />
              </UFormGroup>
            </div>
            <template #footer>
              <div class="flex gap-2 justify-end">
                <UButton color="primary" variant="solid" label="OK" />
                <UButton @click="shutdownModal = false" color="primary" variant="outline" label="Cancel" />
              </div>
            </template>
          </UCard>
        </UModal>
      </div>
    </div>
    <!-- <UDivider /> -->
    <div class="">
      <h1 class="text-2xl">Patch Notes</h1>
      <UTable :loading="updates === undefined" :rows="rows" :columns="[{
        key: 'id',
        label: 'ID'
      }, {
        key: 'title',
        label: 'Title'
      }, {
        key: 'published',
        label: 'Published'
      }]">
        <template #title-data="{ row }: { row: FeedEntry }">
          <a class="underline" target="_blank" :href="row.link">{{ row.title }}</a>
        </template>
        <template #published-data="{ row }: { row: FeedEntry }">
          {{ row.published?.toLocaleString() ?? "Unavailable" }}
        </template>
      </UTable>
      <div class="flex justify-end px-3 py-3.5 border-t border-gray-200 dark:border-gray-700">
        <UPagination v-if="updates" v-model="page" :page-count="pageCount" :total="updates.length" />
      </div>
    </div>
  </div>
</template>
