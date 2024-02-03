<template>
  <UVerticalNavigation :links="links" :ui="{ label: 'truncate relative hidden sm:block', wrapper: 'float-left pr-4' }" />
</template>

<script setup lang="ts">
const links = [{
  label: 'Home',
  icon: 'i-heroicons-home',
  to: '/'
}, {
  label: 'Server Info',
  icon: 'i-heroicons-command-line',
  to: '/info'
}, {
  label: 'Player List',
  icon: 'i-heroicons-users',
  to: '/players'
}, {
  label: 'Saves Management',
  icon: 'i-heroicons-archive-box',
  to: '/saves'
}, {
  label: 'Config Editor',
  icon: 'i-heroicons-adjustments-horizontal',
  to: '/editor'
}]
</script>
