<template>
  <div class="filter_controls">
    <InputKnob :value="props.frequency" :callback="update_frequency" label="Frequency" />
    <InputKnob :value="props.q" :callback="update_q" label="Q" />
    <InputKnob :value="props.gain" :callback="update_gain" label="Gain" />
    <Default_button name="Delete" :callback="deletefx" />
  </div>
</template>

<script lang="ts" setup>
import { FilterParams } from "~/sound/synth_api_service";
import type { Filter } from "~/types/filter";

const props = defineProps<Filter & { on_delete: () => void }>();

const deletefx = async () => {
  const synth_api = await use_synth_api();
  await synth_api.remove_fx(props.id);
  props.on_delete();
};

const update_frequency = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.FREQUENCY, val);
};
const update_q = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.Q, val);
};
const update_gain = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.GAIN, val);
};
</script>

<style scoped>
.filter_controls {
  display: flex;
  gap: 1rem;
}
</style>
