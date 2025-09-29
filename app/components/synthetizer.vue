<template>
  <div class="synth-container">
    <Sampler v-for="(sampler, index) in samplers" :key="sampler.id ?? index" :config="sampler" />
    <default-button name="+" :callback="addSampler" />
  </div>
</template>

<script lang="ts" setup>
import { default_sampler_config } from "~/config/default_sampler";
import type { Sampler } from "~/types/sampler";

const samplers = ref<Sampler[]>([]);

const addSampler = () => {
  samplers.value.push(default_sampler_config);
};
</script>

<style scoped>
.synth-container {
  display: flex;
  flex-direction: column;
  border: 2px solid white;
  padding: 10px;
  gap: 10px;
}
</style>
