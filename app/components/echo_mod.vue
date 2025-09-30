<template>
  <div class="echo_controls">
    <InputKnob :value="props.delay" :callback="update_delay" label="Delay" />
    <InputKnob :value="props.feedback" :callback="update_feedback" label="Feedb" />
    <InputKnob :value="props.r_delay_offset" :callback="update_r_delay_offset" label="Right" />
    <InputKnob :value="props.l_delay_offset" :callback="update_l_delay_offset" label="Left" />
    <InputKnob :value="props.dry" :callback="update_dry" label="Dry" />
    <InputKnob :value="props.wet" :callback="update_wet" label="Wet" />
    <Default_button name="Delete" :callback="deletefx" />
  </div>
</template>

<script lang="ts" setup>
import type { Echo } from "~/types/echo";
import Default_button from "./default_button.vue";
import { EchoParams, FilterParams } from "~/sound/synth_api_service";

const props = defineProps<Echo & { on_delete: () => void }>();

const deletefx = async () => {
  const synth_api = await use_synth_api();
  await synth_api.remove_fx(props.id);
  props.on_delete(); // Supprime le composant du DOM côté parent
};

// Tu pourras compléter chaque callback pour propager les changements
const update_delay = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.DELAY, val * 20);
};
const update_feedback = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.FEEDBACK, val / 100);
};
const update_r_delay_offset = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.R_DELAY_OFFSET, val);
};
const update_l_delay_offset = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.L_DELAY_OFFSET, val);
};
const update_dry = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.DRY, val / 100);
};
const update_wet = async (val: number) => {
  const synth_api = await use_synth_api();
  synth_api.edit_fx(props.id, EchoParams.WET, val / 100);
};
</script>

<style scoped>
.echo_controls {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  width: 100%;
  gap: 10px;
  padding: 10px;
}
</style>
