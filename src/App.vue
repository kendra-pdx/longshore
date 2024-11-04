<script setup lang="ts">
import { RouterView } from 'vue-router';
import TopMenu from './components/TopMenu.vue';

</script>


<template>
  <header >
    <TopMenu />
  </header>
  <main>
    <RouterView />
  </main>
</template>

<style scoped>
header {
  @apply sticky flex flex-row gap-2 h-full
}

main {
  @apply w-full
}
</style>