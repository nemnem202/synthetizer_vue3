<template>
  <Synthetizer />
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { MidiController } from "~/midi/midi_controller_service";
import { use_loaded_files_state } from "~/state/loaded_files";

const {loaded_files, add_file} = use_loaded_files_state()

const synth_api_promise = use_synth_api();

const file_names = ["5sec.wav", "sin.wav", "wave.wav"];

const fetch_file = async (file_name: string): Promise<File> => {
  const response = await fetch(`/sound_assets/${file_name}`);
  const blob = await response.blob();
  return new File([blob], file_name, { type: blob.type });
};

const fetch_all_files = async (): Promise<File[]> => {
  return Promise.all(file_names.map(fetch_file));
};

onMounted(async () => {
  const synth_api = await synth_api_promise;
  const midi_controller = new MidiController();
  const files = await fetch_all_files();

  for (const file of files) {
    const imported_sample = await synth_api.import_sample(file, false, 0);
    if (imported_sample) {
      add_file(imported_sample)
    }
  }
});
</script>
