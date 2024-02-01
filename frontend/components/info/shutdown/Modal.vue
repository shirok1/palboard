<template>
  <UModal v-model="model">
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
          <UButton @click="model = false" color="primary" variant="outline" label="Cancel" />
        </div>
      </template>
    </UCard>
  </UModal>
</template>

<script setup lang="ts">
const model = defineModel<boolean>()
</script>
