<template>
  <div class="synth-container">
    <div class="samplers-container">
      <Sampler v-for="(sampler, index) in samplers" :key="sampler.id ?? index" :config="sampler" />
      <default-button name="+" :callback="addSampler" />
    </div>
    <div class="mixer-container">
      <div class="buttons-container">
        <default-button name="+ Echo" :callback="addEcho" />
        <default-button name="+ Filter" :callback="addEcho" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { default_sampler_config } from "~/config/default_sampler";
import type { Sampler } from "~/types/sampler";

const samplers = ref<Sampler[]>([]);

const addSampler = () => {
  samplers.value.push(default_sampler_config);
};

const addEcho = () => {};

const addFilter = () => {};
</script>

<style scoped>
.synth-container {
  width: 1000px;
  justify-content: space-between;
  border: 2px solid white;
}
.synth-container,
.samplers-container,
.mixer-container {
  display: flex;

  padding: 10px;
  gap: 10px;
}

.samplers-container,
.mixer-container {
  flex-direction: column;
  max-height: 80vh;
  overflow-y: scroll;
  align-items: center;
}

.samplers-container {
  flex: 1;
}

.mixer-container {
  width: 400px;
}
</style>
