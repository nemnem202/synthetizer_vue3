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
  overflow: hidden;
}

* {
  box-sizing: border-box;
}

*::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

*::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 10px;
}

*::-webkit-scrollbar-thumb {
  background-color: transparent;
  border-radius: 5px;
  border: 2px solid transparent;
}

*::-webkit-scrollbar-thumb:hover {
  background-color: transparent;
}

* {
  scrollbar-width: thin;
  scrollbar-color: #555 transparent;
}
</style>
