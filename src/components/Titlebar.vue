<template>
  <Transition name="appear">
    <div v-if="isHovering" id="titlebar" data-tauri-drag-region @mouseleave="isHovering = false">
      <div class="flex-1"></div>
      <div class="flex-none gap-1 pr-2">
        <button class="btn btn-sm btn-square btn-ghost" @click="changeTheme">
          <span class="iconify w-6 h-6" :class="getThemeIcon"></span>
        </button>
        <button class="btn btn-sm btn-square btn-ghost" @click="minimize">
          <span class="iconify fa6-regular--window-minimize w-4 h-4"></span>
        </button>
        <button class="btn btn-sm btn-square btn-ghost" @click="maximize">
          <span class="iconify w-4 h-4" :class="getMaximizeIcon"></span>
        </button>
        <button class="btn btn-sm btn-square btn-ghost" @click="close">
          <span class="iconify mingcute--close-line w-6 h-6"></span>
        </button>
      </div>
    </div>
  </Transition>
  <div v-if="!isHovering" id="titlebar-hover" @mouseenter="isHovering = true"></div>
</template>

<script setup lang="ts">
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { computed, onMounted, onUnmounted, ref } from "vue";

const props = defineProps({
  theme: Boolean,
});

const emits = defineEmits<{
  setTheme: [value: boolean];
}>();

let unlisten: UnlistenFn;
const isMaximized = ref(false);
const isHovering = ref(false);

// Computed

const getThemeIcon = computed(() => (props.theme ? "mingcute--moon-stars-line" : "mingcute--sun-line"));

const getMaximizeIcon = computed(() =>
  isMaximized.value ? "fa6-regular--window-restore" : "fa6-regular--window-maximize"
);

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

function close() {
  appWindow.close();
}

// Events

onMounted(async () => {
  unlisten = await listen("tauri://resize", () => {
    appWindow.isMaximized().then((res) => (isMaximized.value = res));
  });
});

onUnmounted(() => {
  unlisten();
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
