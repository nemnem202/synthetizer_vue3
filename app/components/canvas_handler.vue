<template>
  <div class="canvas-container">
    <input
      ref="file_input"
      type="file"
      accept=".wav"
      style="display: none"
      @input="handle_input"
      multiple
    />
    <button class="import-btn" @click="open_file_dialog">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="24px"
        width="24px"
        viewBox="0 -960 960 960"
        fill="black"
      >
        <path
          d="M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h200v80H160v480h640v-480H600v-80h200q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H160Zm320-184L280-544l56-56 104 104v-304h80v304l104-104 56 56-200 200Z"
        />
      </svg>
    </button>

    <select class="loaded_samples" name="loaded_samples" @input="handle_loaded_input">
      <option
        v-for="sample in loaded_files"
        :key="sample.sample_id"
        :value="sample.sample_id"
        :selected="sample.sample_id === selected_sample_id"
      >
        {{ sample.title }}
      </option>
    </select>

    <CanvasVue v-if="points.length > 0" :curve="{ color: '#fe621b', points: points }" />

    <div v-else class="placeholder">[No Sample loaded]</div>
  </div>
</template>

<script setup lang="ts">
import { use_loaded_files_state } from "~/state/loaded_files";

const selected_sample_id = ref(0);

const points = ref<number[]>([]);

const file_input = ref<HTMLInputElement | null>(null);

const { loaded_files, add_file } = use_loaded_files_state();

const props = defineProps<{ id: number }>();

const handle_input = async (e: Event) => {
  const input = e.target as HTMLInputElement;

  if (!input.files || input.files.length === 0) return;

  const synthapi = await use_synth_api();

  let i;
  for (i = 0; i < input.files.length; i++) {
    const sample_data = await synthapi.import_sample(input.files[i], false, props.id);
    if (!sample_data) {
      console.error("[ERROR] : sample could not be used");
      continue;
    }

    add_file(sample_data);
  }

  const sortedFiles = loaded_files.value.sort((a, b) => b.sample_id - a.sample_id);
  selected_sample_id.value =
    sortedFiles.length > 0 && sortedFiles[0] ? sortedFiles[0].sample_id : 0;
};

const open_file_dialog = () => {
  if (!file_input.value) return;
  file_input.value.click();
};

const handle_loaded_input = (e: Event) => {
  const input = e.target as HTMLSelectElement;
  selected_sample_id.value = parseInt(input.value);
};

watchEffect(async () => {
  const selectedFile = loaded_files.value[selected_sample_id.value];
  points.value =
    selectedFile && selectedFile.channels && selectedFile.channels[0]
      ? Array.from(selectedFile.channels[0])
      : [];

  const synthapi = await use_synth_api();

  synthapi.set_existing_sample(selected_sample_id.value, props.id);
});
</script>

<style scoped>
.canvas-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  position: relative;
}

.placeholder {
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: black;
  height: 300px;
}

.import-btn {
  position: absolute;
  top: 10px;
  left: 10px;
  width: 30px;
  height: 30px;
  padding: 0;
}

.loaded_samples {
  position: absolute;
  top: 50px;
  left: 10px;
  width: 30px;
  height: 30px;
  padding: 0;
}
</style>
