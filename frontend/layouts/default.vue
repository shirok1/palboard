<template>
  <div class="w-screen h-screen flex p-4">
    <SideNav />
    <UContainer>
      <slot />
    </UContainer>
    <UNotifications />
  </div>
</template>
