<template>
  <Synthetizer :loaded-samples="loaded_samples" />
</template>

<script setup lang="ts">
import { MidiController } from "~/midi/midi_controller_service";
import type { SampleDataWithChannels } from "~/types/sampler";

const loaded_samples = ref<SampleDataWithChannels[]>([]);

const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};
onMounted(async () => {
  const synth_api = await use_synth_api();
  const midi = new MidiController();

  const files_array = await getFilesInPublicFolder();

  for (const f of files_array) {
    const importedSample = await synth_api.import_sample(f, false, 0);
    if (!importedSample) continue;

    loaded_samples.value.push(importedSample);

    // await sleep(1000);
  }
});

const getFilesInPublicFolder = async (): Promise<File[]> => {
  const file_names = ["5sec.wav", "sin.wav", "wave.wav"];
  const promises = file_names.map(async (file_name) => {
    const response = await fetch(`/sound_assets/${file_name}`);
    const blob = await response.blob();
    const file = new File([blob], file_name, { type: blob.type });
    return file;
  });

  const files_array = await Promise.all(promises);

  return files_array;
};
</script>
