<template>
  <main :data-theme="theme ? 'dark' : 'light'" class="w-screen h-screen">
    <Titlebar :theme="theme" @set-theme="setTheme"></Titlebar>
    <button class="btn btn-primary" @click="openDialog">Open Window</button>
    <button class="btn btn-primary" @click="closeDialog">Close Window</button>
  </main>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import Titlebar from "./components/Titlebar.vue";
import { onMounted, ref } from "vue";

const theme = ref(false);

function setTheme(value: boolean) {
  theme.value = value;
  if (theme.value) localStorage.setItem("theme", "dark");
  else localStorage.setItem("theme", "light");
}

function openDialog() {
  invoke("open_file_dialog")
    .then((res) => console.log(res))
    .catch((err) => console.error(err));
}

function closeDialog() {
  invoke("close_file_dialog")
    .then((res) => console.log(res))
    .catch((err) => console.error(err));
}

onMounted(() => {
  theme.value = localStorage.getItem("theme") === "dark";
});
</script>
