<template>
  <UTable :loading="updates === undefined" :rows="rows" :columns="columns">
    <template #title-data="{ row }: { row: FeedEntry }">
      <a class="underline" target="_blank" :href="row.link">{{ row.title }}</a>
    </template>
    <template #published-data="{ row }: { row: FeedEntry }">
      {{ row.published?.toLocaleString() ?? "Unavailable" }}
    </template>
  </UTable>
  <div v-if="props.pageCount != undefined"
    class="flex justify-end px-3 py-3.5 border-t border-gray-200 dark:border-gray-700">
    <UPagination v-if="updates" v-model="page" :page-count="props.pageCount" :total="updates.length" />
  </div>
</template>

<script setup lang="ts">
import { extractFromXml, type FeedEntry } from '@extractus/feed-extractor'

const props = defineProps<{
  pageCount?: number
}>()

const columns = [{
  key: 'id',
  label: 'ID'
}, {
  key: 'title',
  label: 'Title'
}, {
  key: 'published',
  label: 'Published'
}]

const castPublished = (entry: FeedEntry) => ({
  ...entry,
  ...entry.published && { published: new Date(entry.published) },
})
const parseResponse = (text: string) => extractFromXml(text).entries?.map(castPublished)

const { data: serverUpdates } = await useFetch<FeedEntry[]>(
  '/proxy/steamdb/PatchnotesRSS/?appid=2394010', { parseResponse })
const { data: clientUpdates } = await useFetch<FeedEntry[]>(
  '/proxy/steamdb/PatchnotesRSS/?appid=1623730', { parseResponse })
const updates = computed(() =>
  [...clientUpdates.value ?? [], ...serverUpdates.value ?? []]
    .sort((a, b) => (b.published?.getTime() ?? 0) - (a.published?.getTime() ?? 0)))

const page = ref(1)
const rows = computed(() => {
  if (props.pageCount == undefined) {
    return updates.value
  }
  return updates.value?.slice((page.value - 1) * props.pageCount, (page.value) * props.pageCount)
})
</script>