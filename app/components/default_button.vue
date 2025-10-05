<template>
  <button class="button" @click="props.callback" :style="{ fontSize: (props.size || 16) + 'px' }">
    {{ props.name }}
  </button>
</template>

<script setup lang="ts">
const props = defineProps<{ name: string; callback: () => void; size?: number }>();
</script>
