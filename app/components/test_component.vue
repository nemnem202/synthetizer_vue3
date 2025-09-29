<template>
  <button @click="increment_counter">{{ counter }}</button>
</template>

<script setup lang="ts">
import { counter, increment_counter } from "~/scripts/test";
</script>
