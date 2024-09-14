<template>
  <ul class="w-full h-full overflow-auto p-4">
    <li
      v-for="(content, index) in props.contents"
      :key="index"
      :class="{ 'text-red-700': content.is_dir }"
      class="whitespace-nowrap cursor-pointer text-ellipsis overflow-hidden"
      @click="emits('select', index)"
    >
      {{ content.name }}
    </li>
  </ul>
</template>

<script setup lang="ts">
const props = defineProps<{ contents: Item[] }>();

const emits = defineEmits<{select: [target: number]}>()
</script>
