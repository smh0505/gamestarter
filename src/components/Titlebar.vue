<template>
  <Transition name="appear">
    <div
      v-show="isHovering || !isMainWindow"
      class="navbar min-h-12 bg-base-100 shadow-lg shadow-base-300 z-50"
      :class="{absolute: isMainWindow}"
      data-tauri-drag-region
    >
      <div class="flex-1" data-tauri-drag-region>
        <slot></slot>
      </div>
      
      <div class="flex-none gap-1">
        <label v-if="isMainWindow" class="swap swap-rotate btn btn-sm btn-square btn-ghost">
          <input type="checkbox" class="theme-controller" :checked="props.theme" @click="changeTheme" />
          <span class="swap-off iconify mingcute--sun-line w-6 h-6"></span>
          <span class="swap-on iconify mingcute--moon-stars-line w-6 h-6"></span>
        </label>

        <button v-if="isMainWindow" class="btn btn-sm btn-square btn-ghost" @click="minimize">
          <span class="iconify fa6-regular--window-minimize w-4 h-4"></span>
        </button>

        <label v-if="isMainWindow" class="swap btn btn-sm btn-square btn-ghost">
          <input type="checkbox" :checked="isMaximized" @click="maximize" />
          <span class="swap-off iconify fa6-regular--window-maximize w-4 h-4"></span>
          <span class="swap-on iconify fa6-regular--window-restore w-4 h-4"></span>
        </label>

        <button class="btn btn-sm btn-square btn-ghost" @click="close">
          <span class="iconify mingcute--close-line w-6 h-6"></span>
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { computed, onMounted, onUnmounted, ref } from "vue";

const props = defineProps({
  theme: Boolean,
  isHovering: Boolean
});

const emits = defineEmits<{
  setTheme: [value: boolean];
}>();

let unlistenResize: UnlistenFn;
const isMaximized = ref(false);

// Computed

const isMainWindow = computed(() => appWindow.label === "main");

// Methods

function changeTheme() {
  emits("setTheme", !props.theme);
}

function minimize() {
  appWindow.minimize();
}

function maximize() {
  appWindow.toggleMaximize();
}

async function close() {
  if (appWindow.label === "main") await WebviewWindow.getByLabel("file")?.close();
  appWindow.close();
}

// Events

onMounted(async () => {
  unlistenResize = await listen("tauri://resize", () => {
    appWindow.isMaximized().then((res) => (isMaximized.value = res));
  });
  appWindow.isMaximized().then((res) => (isMaximized.value = res));
});

onUnmounted(() => {
  unlistenResize();
});
</script>

<style>
.appear-enter-active,
.appear-leave-active {
  @apply transition-transform duration-300 ease-in-out;
}

.appear-enter-from,
.appear-leave-to {
  @apply -translate-y-20;
}
</style>
