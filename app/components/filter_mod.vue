<template>
  <div class="filter_controls">
    <InputKnob :value="props.frequency" :callback="update_frequency" label="Frequency" />
    <InputKnob :value="props.q" :callback="update_q" label="Q" />
    <InputKnob :value="props.gain" :callback="update_gain" label="Gain" />
  </div>
</template>

<script lang="ts" setup>
import type { Filter } from "~/types/filter";

const props = defineProps<Filter>();

const update_frequency = (val: number) => {
  console.log("new frequency:", val);
};
const update_q = (val: number) => {
  console.log("new q:", val);
};
const update_gain = (val: number) => {
  console.log("new gain:", val);
};
</script>

<style scoped>
.filter_controls {
  display: flex;
  gap: 1rem;
}
</style>
