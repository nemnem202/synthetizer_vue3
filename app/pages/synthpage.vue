<template>
  <Synthetizer />
</template>

<script setup lang="ts">
import { MidiController } from "~/midi/midi_controller_service";

onMounted(async () => {
  const synth_api = await use_synth_api();
  const midi = new MidiController();
});
</script>
