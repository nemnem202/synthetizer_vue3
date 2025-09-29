<template>
  <button class="button" @click="props.callback">{{ props.name }}</button>
</template>

<script setup lang="ts">
const props = defineProps<{ name: string; callback: () => void }>();
</script>
