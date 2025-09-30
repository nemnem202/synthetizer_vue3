<template>
  <div class="echo_controls">
    <InputKnob :value="props.delay" :callback="update_delay" label="Delay" />
    <InputKnob :value="props.feedback" :callback="update_feedback" label="Feedback" />
    <InputKnob
      :value="props.r_delay_offset"
      :callback="update_r_delay_offset"
      label="Right Delay Offset"
    />
    <InputKnob
      :value="props.l_delay_offset"
      :callback="update_l_delay_offset"
      label="Left Delay Offset"
    />
    <InputKnob :value="props.dry" :callback="update_dry" label="Dry" />
    <InputKnob :value="props.wet" :callback="update_wet" label="Wet" />
  </div>
</template>

<script lang="ts" setup>
import type { Echo } from "~/types/echo";

const props = defineProps<Echo>();

// Tu pourras compléter chaque callback pour propager les changements
const update_delay = (val: number) => {
  console.log("new delay:", val);
};
const update_feedback = (val: number) => {
  console.log("new feedback:", val);
};
const update_r_delay_offset = (val: number) => {
  console.log("new r_delay_offset:", val);
};
const update_l_delay_offset = (val: number) => {
  console.log("new l_delay_offset:", val);
};
const update_dry = (val: number) => {
  console.log("new dry:", val);
};
const update_wet = (val: number) => {
  console.log("new wet:", val);
};
</script>

<style scoped>
.echo_controls {
  display: flex;
  gap: 1rem;
}
</style>
