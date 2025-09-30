<template>
  <Synthetizer />
</template>

<script setup lang="ts">
import { MidiController } from "./midi/midi_controller_service";

onMounted(async () => {
  const synth_api = await use_synth_api();
  const midi = new MidiController();
});
</script>

<style>
body {
  background-color: rgb(36, 33, 48);
  color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
}

* {
  box-sizing: border-box;
}
</style>
