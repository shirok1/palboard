<script setup lang="ts">
import { ref } from 'vue';

const { data: players, refresh: refreshPlayers } = await useFetch<Player[]>('/proxy/gateway/pal/players')
const page = ref(1)
const pageCount = 5
const rows = computed(() => {
  return players.value?.slice((page.value - 1) * pageCount, (page.value) * pageCount)
})

const toast = useToast()
const kick_or_ban = async (type: 'kick' | 'ban', steamid: string) => {
  const res = await $fetch<string>(`/proxy/gateway/pal/${type}`, {
    method: 'POST',
    body: { steamid }
  })
  const title_type = type === 'kick' ? 'Kick' : 'Ban'
  const description = `Response: "${res.trim()}"`
  toast.add(res.startsWith('Failed')
    ? { title: `${title_type} Failed`, description, color: 'red', icon: 'i-heroicons-x-circle-20-solid' }
    : { title: `${title_type} Successful`, description, icon: 'i-heroicons-check-circle-20-solid' })
  refreshPlayers()
}

const actions = (row: Player) => [[{
  label: 'Copy Steam ID',
  icon: 'i-heroicons-document-duplicate-20-solid',
  click: () => navigator.clipboard.writeText(row.steamid)
}, {
  label: 'View Steam Profile',
  icon: 'i-heroicons-magnifying-glass-20-solid',
  click: () => window.open(`https://steamcommunity.com/profiles/${row.steamid}`)
}], [{
  label: 'Kick from server',
  icon: 'i-heroicons-face-frown-20-solid',
  click: () => kick_or_ban('kick', row.steamid)
}, {
  label: 'Ban from server',
  icon: 'i-heroicons-no-symbol-20-solid',
  click: () => kick_or_ban('ban', row.steamid)
}]]
</script>

<template>
  <div class="">
    <div class="">
      <UTable :loading="!players" :rows="rows" :columns="[{
        key: 'name',
        label: 'Name'
      }, {
        key: 'playeruid',
        label: 'Player UID'
      }, {
        key: 'steamid',
        label: 'Steam ID'
      }, { key: 'action' }]">
        <template #steamid-data="{ row }: { row: Player }">
          <a :href="row.steamid">{{ row.steamid }}</a>
        </template>
        <template #action-header="{ }">
          <UButton @click="refreshPlayers()" color="gray" variant="ghost" icon="i-heroicons-arrow-path-20-solid" />
        </template>
        <template #action-data="{ row }: { row: Player }">
          <UDropdown :items="actions(row)">
            <UButton color="gray" variant="ghost" icon="i-heroicons-ellipsis-horizontal-20-solid" />
          </UDropdown>
        </template>
      </UTable>
      <div class="flex justify-end px-3 py-3.5 border-t border-gray-200 dark:border-gray-700">
        <UPagination v-if="players" v-model="page" :page-count="pageCount" :total="players.length" />
      </div>
    </div>
  </div>
</template>
