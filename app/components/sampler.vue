<template>
  <div class="sampler-container">
    <CanvasVue :curve="{ color: 'green', points: Array.from({ length: 5000 }, () => 0.5) }" />
    <div class="controls-container">
      <div class="gain-pan-container">
        <InputKnob :value="props.config.gain" :callback="updateGain" label="Gain" />
        <InputKnob :value="props.config.pan" :callback="updatePan" label="Pan" />
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
import type { Sampler } from "~/types/sampler";

const props = defineProps<{
  config: Sampler;
}>();

const updateGain = (value: number) => {
  console.log("gain updated:", value);
};

const updatePan = (value: number) => {
  console.log("pan updated:", value);
};

const updateAttack = (value: number) => {
  console.log("attack updated:", value);
};

const updateDecay = (value: number) => {
  console.log("decay updated:", value);
};

const updateRelease = (value: number) => {
  console.log("release updated:", value);
};

const updateSustain = (value: number) => {
  console.log("sustain updated:", value);
};

const updateDelay = (value: number) => {
  console.log("delay updated:", value);
};
</script>

<style scoped>
.sampler-container {
  display: flex;
  flex-direction: column;
  border: 2px solid white;
  max-width: 800px;
  padding: 10px;
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
