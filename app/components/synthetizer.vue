<template>
  <div class="synth-container">
    <div class="samplers-container">
      <Sampler
        v-for="sampler in samplers"
        :key="`sampler-${sampler.id}`"
        :config="sampler"
      />
      <default-button name="+" :callback="add_sampler" />
    </div>

    <div class="mixer-container">
      <div class="buttons-container">
        <div v-for="(effect, index) in fx" :key="effect.id ?? index" class="effect-item">
          <Echo_mod
            v-if="is_echo(effect)"
            v-bind="effect"
            :on_delete="() => remove_fx(effect.id)"
            :key="`echo-${effect.id}`"
          />
          <Filter_mod
            v-else-if="is_filter(effect)"
            v-bind="effect"
            :on_delete="() => remove_fx(effect.id)"
            :key="`filter-${effect.id}`"
          />
        </div>

        <default-button name="+ Echo" :callback="add_echo" />
        <default-button name="+ Filter" :callback="add_filter" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { default_echo_config } from "~/config/default_echo";
import { default_filter_config } from "~/config/default_filter";
import { default_sampler_config } from "~/config/default_sampler";
import { Effects, type SampleData } from "~/sound/synth_api_service";
import type { Sampler } from "~/types/sampler";
import type { Filter } from "~/types/filter";
import type { Echo } from "~/types/echo";
import Echo_mod from "./echo_mod.vue";
import Filter_mod from "./filter_mod.vue";

const samplers = ref<Sampler[]>([]);

const fx = ref<(Echo | Filter)[]>([]);


const remove_fx = (id: number) => {
  fx.value = fx.value.filter((e) => e.id !== id);
};

onMounted(async () => {
  const midi = await useMidi();
  midi();
});

const add_sampler = async () => {
  const synth_api = await use_synth_api();
  const id = synth_api.create_sampler();
  const config = { ...default_sampler_config, id };
  samplers.value.push(config);
  console.log("nouveau sampler avec la config ", config);
};

const add_echo = async () => {
  const synth_api = await use_synth_api();
  const id = synth_api.add_fx(Effects.ECHO);
  const config = { ...default_echo_config };
  config.id = id;
  console.log("nouvel echo avec l'id" + id);
  fx.value.push(config);
};

const add_filter = async () => {
  const synth_api = await use_synth_api();
  const id = synth_api.add_fx(Effects.FILTER);
  const config = { ...default_filter_config };
  config.id = id;
  fx.value.push(config);
};

const is_echo = (effect: Echo | Filter): effect is Echo => {
  return (effect as Echo).delay !== undefined;
};

const is_filter = (effect: Echo | Filter): effect is Filter => {
  return (effect as Filter).frequency !== undefined;
};
</script>


<style scoped>
.synth-container {
  width: 1000px;
  display: flex;
  justify-content: space-between;
  border: 1px solid white;
  padding: 10px;
  gap: 10px;
}

.samplers-container,
.mixer-container {
  display: flex;
  flex-direction: column;
  max-height: 80vh;
  overflow-y: auto;
  align-items: center;
  padding: 10px;
  gap: 10px;
}

.samplers-container {
  flex: 1;
}

.mixer-container {
  width: 400px;
}
</style>
