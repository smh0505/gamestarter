<template>
  <main :data-theme="theme ? 'dark' : 'light'" class="w-screen h-screen grid grid-rows-[48px_1fr]">
    <Titlebar :theme="theme" @set-theme="setTheme">
      <span @click="goUp">
        <strong>{{ currentDir }}</strong>
      </span>
    </Titlebar>
    <Filepicker :contents="contents" @select="select"></Filepicker>
  </main>
</template>

<script setup lang="ts">
import Filepicker from "./components/Filepicker.vue";
import Titlebar from "./components/Titlebar.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";

const theme = ref(false);
const haveParent = ref(false);
const currentDir = ref("");
const contents = ref<Item[]>([]);

function setTheme(value: boolean) {
  theme.value = value;
  if (theme.value) localStorage.setItem("theme", "dark");
  else localStorage.setItem("theme", "light");
}

async function getItems() {
  const response = (await invoke("get_items", { dir: currentDir.value }).catch(console.error)) as Folder;
  currentDir.value = response.directory;
  contents.value = response.items;
  haveParent.value = response.have_parent;
  console.log(haveParent.value)
}

function select(index: number) {
  const target = contents.value[index];
  if (target.is_dir) {
    currentDir.value = target.path;
    getItems();
  }
}

async function goUp() {
  if (haveParent.value) {
    const target = await invoke("get_parent", { dir: currentDir.value }).catch(console.error) as string;
    currentDir.value = target;
    console.log(currentDir.value)
    await getItems();
  }
}

onMounted(() => {
  theme.value = localStorage.getItem("theme") === "dark";
  getItems();
});
</script>
