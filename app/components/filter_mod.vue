<template>
  <div class="filter_controls">
    <div class="row">
      <InputKnob :value="props.frequency" :callback="update_frequency" label="Frequency" />
      <InputKnob :value="props.q" :callback="update_q" label="Q" />
      <InputKnob v-if="type === 2" :value="props.gain" :callback="update_gain" label="Gain" />
    </div>
    <div class="row">
      <select name="filter-type" id="filter-type" @input="update_type">
        <option value="0" selected>lowpass</option>
        <option value="1">highpass</option>
        <option value="2">bell</option>
      </select>
      <Default_button name="Delete" :callback="deletefx" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { FilterParams } from "~/sound/synth_api_service";
import type { Filter } from "~/types/filter";

const props = defineProps<Filter & { on_delete: () => void }>();
const type = ref(0);
const deletefx = async () => {
  const synth_api = await use_synth_api();
  await synth_api.remove_fx(props.id);
  props.on_delete();
};

const update_frequency = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.FREQUENCY, val * 80);
};
const update_q = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.Q, val / 5 + 0.2);
};
const update_gain = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.GAIN, val / 5);
};

const update_type = async (e: Event) => {
  const event = e as InputEvent;
  const target = event.target as HTMLInputElement;
  if (!target) return;
  const value = target.value;
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, FilterParams.TYPE, parseInt(value));
  type.value = parseInt(value);
};
</script>

<style scoped>
.filter_controls {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  flex-direction: column;
  width: 100%;
  gap: 10px;
  padding: 10px;
}

.row {
  display: flex;
  width: 100%;
  gap: 10px;
}
</style>
