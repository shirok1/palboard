<template>
  <UButtonGroup orientation="horizontal">
    <UButton @click="update({ game: true, validate: true })" color="white" label="Update Server"
      :ui="{ truncate: 'hidden sm:block' }" truncate
      icon="i-heroicons-arrow-path-rounded-square" />
    <UDropdown :items="dropdownItems" :popper="{ placement: 'bottom-end' }">
      <UButton color="gray" icon="i-heroicons-chevron-down-20-solid" />
    </UDropdown>
  </UButtonGroup>
  <UModal v-model="modal" :ui="{ width: 'w-full sm:max-w-2xl' }" prevent-close>
    <UCard :ui="{ ring: '', divide: 'divide-y divide-gray-100 dark:divide-gray-800' }">
      <template #header>
        {{ headerTitle }}
      </template>
      <div class="flex flex-col gap-2">
        <UTextarea v-model="logs" disabled autoresize placeholder="Waiting..." />
        <UProgress :color="progressColor" :value="progressValue" />
      </div>
      <template #footer>
        <div class="flex gap-2 justify-end">
          <UButton :disabled="okDisabled" @click="modal = false" :color="okDisabled ? 'gray' : 'primary'" variant="solid"
            label="OK" />
        </div>
      </template>
    </UCard>
  </UModal>
</template>

<script setup lang="ts">
const logs = ref<string>("")
const modal = ref(false)
const okDisabled = ref(true)
const lastMsg = ref<UpdateSteamMessage>()

const dropdownItems = [
  [{
    label: 'Update w/o Validation',
    icon: "i-heroicons-bug-ant",
    click: () => {
      update({ game: true, validate: false })
    }
  }], [{
    label: 'Update Steam',
    icon: "i-mdi-steam",
    click: () => {
      update({ game: false })
    }
  }]
]

const headerTitle = computed(() =>
  lastMsg.value?.type === 'update_state' ? `Stage: ${lastMsg.value.state_name} (${lastMsg.value.progress}%)`
    : lastMsg.value?.type === 'steam_self_update' ? 'Updating Steam...'
      : lastMsg.value?.type === 'success' ? 'Update success'
        : lastMsg.value?.type === 'error' ? 'Update error'
          : 'Updating...'
)
const progressColor = computed(() =>
  lastMsg.value?.type === 'error' ? 'red'
    : lastMsg.value?.type === 'success' ? 'green'
      : 'primary'
)
const progressValue = computed(() =>
  lastMsg.value?.type === 'update_state' ? (+lastMsg.value.progress)
    : lastMsg.value?.type === 'success' ? 100
      : lastMsg.value?.type === 'error' ? 100 : undefined
)

const update = async (query: { game: false } | { game: true, validate: boolean }) => {
  logs.value = ""
  modal.value = true
  okDisabled.value = true
  const ws = await new Promise<WebSocket>((resolve, reject) => {
    const url = "ws://localhost:1145/steam/update?"
      + (query.game
        ? `game=true&validate=${query.validate}`
        : `game=false`)
    const ws = new WebSocket(url) // TODO: fix websocket proxy
    ws.onopen = () => resolve(ws)
    ws.onerror = (e) => reject(e)
  })
  ws.onmessage = async ({ data }) => {
    if (data instanceof Blob) {
      const text = await data.text();
      logs.value += text;
    } else if (typeof data === "string") {
      const msg: UpdateSteamMessage = JSON.parse(data);
      if (msg.type === "steam_self_update") {
        console.log(`Steam self update: ${msg.status}`);
      } else if (msg.type === "update_state") {
        console.log(`Update state: ${msg.state_name} (${msg.progress}%)`);
      } else if (msg.type === "success") {
        console.log("Update success");
      } else if (msg.type === "error") {
        console.log("Update error");
      } else {
        console.log("Received unknown message:", msg);
      }
      lastMsg.value = msg
    } else {
      // Handle other data types
      console.log("Received non blob data:", data);
    }
  }
  await new Promise((resolve) => ws.onclose = () => resolve(null))
  okDisabled.value = false
}
</script>
