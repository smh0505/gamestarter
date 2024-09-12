<template>
  <main :data-theme="theme ? 'dark' : 'light'" class="w-screen h-screen" @mousemove="handleMouseMove">
    <Titlebar :theme="theme" :is-hovering="isHovering" @set-theme="setTheme"></Titlebar>
    <section class="w-full h-full p-4">
      <button class="btn btn-primary" @click="openDialog">Open Window</button>
    </section>
    <span class="absolute bottom-4 right-4">{{ pos.x }}, {{ pos.y }}</span>
  </main>
</template>

<script setup lang="ts">
import Titlebar from "./components/Titlebar.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, reactive, ref } from "vue";

const theme = ref(false);
const isHovering = ref(false);
const pos = reactive({ x: 0, y: 0 });

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

function handleMouseMove(e: MouseEvent) {
  pos.x = e.clientX;
  pos.y = e.clientY;
  isHovering.value = e.clientY <= 52;
}

onMounted(() => {
  theme.value = localStorage.getItem("theme") === "dark";
});
</script>
