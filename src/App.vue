<template>
  <main :data-theme="theme ? 'dark' : 'light'" class="w-screen h-screen">
    <Titlebar :theme="theme" @set-theme="setTheme"></Titlebar>
  </main>
</template>

<script setup lang="ts">
import Titlebar from './components/Titlebar.vue';
import { onMounted, ref } from 'vue';

const theme = ref(false)

function setTheme(value: boolean) {
  theme.value = value
  if (theme.value) localStorage.setItem("theme", "dark")
  else localStorage.setItem("theme", "light")
}

onMounted(() => {
  theme.value = localStorage.getItem("theme") === "dark"
})
</script>
