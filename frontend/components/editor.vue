<template>
  <div class="flex flex-col gap-4 h-dvh py-10 -my-10">
    <div class="flex-none flex flex-row justify-end gap-2">
      <UButton to="https://tech.palworldgame.com/optimize-game-balance" variant="link" label="Offical Guide" />
      <UButton :disabled="!ready" @click="load('default')" color="white" label="Load Default" />
      <UButton :disabled="!ready" @click="load('current')" color="red" label="Load Current" />
      <UButton :disabled="!ready" @click="save" label="Save" />
    </div>
    <USkeleton v-if="!ready" class="grow" />
    <div ref="containerRef" :class="ready ? 'grow' : ''" />
  </div>
</template>

<script setup lang="ts">
import { onUnmounted } from 'vue'
import { useMonaco } from '@guolao/vue-monaco-editor'
import type { editor } from '@monaco-editor/loader/node_modules/monaco-editor/esm/vs/editor/editor.api.d.ts'

const containerRef = ref()
const { monacoRef, unload } = useMonaco()
const editorRef = shallowRef<editor.IStandaloneCodeEditor>()
const ready = computed(() => editorRef.value !== undefined)

const defaultValue = "; Waiting for loading to complete..."
const stopTryLoading = watchEffect(async () => {
  if (monacoRef.value && containerRef.value) {
    nextTick(() => stopTryLoading())
    editorRef.value = monacoRef.value.editor.create(containerRef.value, {
      automaticLayout: true,
      formatOnType: true,
      formatOnPaste: true,
      wordWrap: 'on',
      theme: 'vs-dark',
      language: 'ini',
      readOnly: true,
      value: defaultValue
    })
    const value = await $fetch<string>("/proxy/gateway/game_config/current").catch(() => {
      toast.add({ title: "Loading default", description: "Since config file is not created yet" })
      return $fetch<string>("/proxy/gateway/game_config/default")
    }
    )
    editorRef.value?.setValue(value)
    editorRef.value?.updateOptions({ readOnly: false })
  }
})

onUnmounted(() => {
  if (!monacoRef.value) {
    unload()
  } else {
    editorRef.value?.dispose()
  }
})

const toast = useToast()

const load = async (path: "default" | "current") => {
  const value = await $fetch<string>(`/proxy/gateway/game_config/${path}`, {
    onResponseError: async () => {
      toast.add({ title: "Failed to load", description: "Maybe it is not created yet, try saving", color: "red", icon: 'i-heroicons-x-circle-20-solid' })
    }
  })
  toast.add({ title: "Loaded", description: `Loaded ${path} config`, icon: 'i-heroicons-check-circle-20-solid' })
  editorRef.value?.setValue(value)
  editorRef.value?.updateOptions({ readOnly: false })
}

const save = async () => {
  const value = editorRef.value?.getValue()
  if (value) {
    await $fetch<string>('/proxy/gateway/game_config/save', {
      method: 'POST',
      body: value,
      onResponseError: async () => {
        toast.add({ title: "Failed to save", description: "Check the console for more information", color: "red", icon: 'i-heroicons-x-circle-20-solid' })
      }
    })
    toast.add({ title: "Saved", description: "Saved the config", icon: 'i-heroicons-check-circle-20-solid' })
  }
}

</script>
