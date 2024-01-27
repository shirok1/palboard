<template>
  <UVerticalNavigation class="" :links="[{
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
}]" />
</template>
