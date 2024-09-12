<template>
  <main :data-theme="theme ? 'dark' : 'light'" class="w-screen h-screen">
    Hello world!
  </main>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';

const theme = ref(false);

onMounted(() => {
  theme.value = localStorage.getItem("theme") === "dark";
})
</script>
