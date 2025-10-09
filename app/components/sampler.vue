<template>
  <div class="sampler-container">
    <CanvasHandler :id="config.id" />
    <div class="controls-container">
      <div class="gain-pan-container">
        <InputKnob :value="props.config.gain" :callback="updateGain" label="Gain" />
        <InputKnob :value="props.config.pan" :callback="updatePan" label="Pan" />
        <InputKnob :value="props.config.shift" :callback="updateShift" label="Shift" />
      </div>

      <div class="adsr-container">
        <InputKnob :value="props.config.adsr.attack" :callback="updateAttack" label="Attack" />
        <InputKnob :value="props.config.adsr.decay" :callback="updateDecay" label="Decay" />
        <InputKnob :value="props.config.adsr.release" :callback="updateRelease" label="Release" />
        <InputKnob :value="props.config.adsr.sustain" :callback="updateSustain" label="Sustain" />
        <InputKnob :value="props.config.adsr.delay" :callback="updateDelay" label="Delay" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { OscKey } from "~/sound/synth_api_service";
import type { Sampler } from "~/types/sampler";

const props = defineProps<{
  config: Sampler;
}>();

const updateGain = async (value: number) => {
  const synth_api = await use_synth_api();
  synth_api.update_sampler(props.config.id, OscKey.GAIN, value);
};

const updatePan = async (value: number) => {
  const synth_api = await use_synth_api();

  synth_api.update_sampler(props.config.id, OscKey.PAN, ((value - 50) * 2) / 100);
};

const updateAttack = async (value: number) => {
  const synth_api = await use_synth_api();

  synth_api.update_sampler(props.config.id, OscKey.ATTACK, value * 100);
};

const updateDecay = async (value: number) => {
  const synth_api = await use_synth_api();

  synth_api.update_sampler(props.config.id, OscKey.DECAY, value * 100);
};

const updateRelease = async (value: number) => {
  const synth_api = await use_synth_api();
  synth_api.update_sampler(props.config.id, OscKey.RELEASE, value * 100);
};

const updateSustain = async (value: number) => {
  const synth_api = await use_synth_api();
  synth_api.update_sampler(props.config.id, OscKey.SUSTAIN, value / 100);
};

const updateDelay = async (value: number) => {
  const synth_api = await use_synth_api();
  synth_api.update_sampler(props.config.id, OscKey.DELAY, value * 100);
};
const updateShift = async (value: number) => {
  const synth_api = await use_synth_api();
  synth_api.update_sampler(props.config.id, OscKey.PITCH, value - 50);
};
</script>

<style scoped>
.sampler-container {
  display: flex;
  flex-direction: column;
  border: 1px solid white;
  gap: 10px;
  max-width: 800px;
  padding: 10px;
  width: 100%;
}

.controls-container {
  display: flex;
  justify-content: space-between;
  gap: 30px;
}
.gain-pan-container {
  display: flex;
  gap: 10px;
}

.adsr-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 10px;
}
</style>
