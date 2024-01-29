<template>
  <div>
    Sorry but currently we can only...
  </div>
  <UButton @click="do_save" color="primary" variant="solid" label="Save the World Data" />
</template>

<script setup lang="ts">
const toast = useToast()
const do_save = async () => {
  const res = await $fetch<string>(`/proxy/gateway/save`, {
    method: 'POST'
  })
  const title_type = 'Save'
  const description = `Response: "${res.trim()}"`
  toast.add(res.startsWith('Failed')
    ? { title: `${title_type} Failed`, description, color: 'red', icon: 'i-heroicons-x-circle-20-solid' }
    : { title: `${title_type} Successful`, description, icon: 'i-heroicons-check-circle-20-solid' })
}
</script>
