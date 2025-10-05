<template>
  <input type="file" accept=".wav" style="display: none" ref="file_input" @input="handleinput" />
  <div class="canvas-container">
    <button class="import-btn" @click="open_file_dialog">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="24px"
        viewBox="0 -960 960 960"
        width="24px"
        fill="black"
      >
        <path
          d="M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h200v80H160v480h640v-480H600v-80h200q33 0 56.5 23.5T880-720v480q0 33-23.5 56.5T800-160H160Zm320-184L280-544l56-56 104 104v-304h80v304l104-104 56 56-200 200Z"
        />
      </svg>
    </button>
    <select name="loaded_samples" id="loaded_samples" @input="handleloadedinput">
      <option v-for="s in loaded_samples" :value="s.sample_id" :selected="s.sample_id === 0">
        {{ s.title }}
      </option>
    </select>
    <CanvasVue v-if="points" :curve="{ color: '#fe621b', points: points }" />
    <div v-if="!points" class="placeholder">[No Sample loaded]</div>
  </div>
</template>

<script lang="ts" setup>
import type { SampleDataWithChannels } from "~/types/sampler";

const props = defineProps<{ id: number; loaded_samples: SampleDataWithChannels[] }>();
const file_input = ref<HTMLInputElement | null>(null);

const points = ref<number[] | null>(null);

const open_file_dialog = () => {
  file_input.value?.click();
};

onMounted(() => {
  setCurrentLoadedInput(0);
});

const setCurrentLoadedInput = (sample_id: number) => {
  const sample = props.loaded_samples.find((e) => e.sample_id === sample_id);
  if (!sample) return;

  points.value = Array.from(sample.channels[0]);
};

const handleinput = async (e: Event) => {
  const target = e.target as HTMLInputElement;
  if (!target.files) return;
  const file = target.files[0];
  if (!file) return;
  const synth_api = await use_synth_api();

  synth_api.import_sample(target.files[0], false, props.id);
  const array_buffer = await file.arrayBuffer();
  const audio_ctx = new AudioContext();
  const audio_buffer = await audio_ctx.decodeAudioData(array_buffer);

  const channels: Float32Array[] = [];
  for (let i = 0; i < audio_buffer.numberOfChannels; i++) {
    channels.push(audio_buffer.getChannelData(i));
  }
  if (channels[0]) {
    points.value = sincInterpolation(Array.from(channels[0]));
  } else {
    points.value = []; // fallback si aucun canal
  }
};

const handleloadedinput = async (e: Event) => {
  const target = e.target as HTMLInputElement;
  const synth_api = await use_synth_api();

  synth_api.set_existing_sample(parseInt(target.value), props.id);

  setCurrentLoadedInput(parseInt(target.value));
};

const sincInterpolation = (arr: number[], targetLength = 5000): number[] => {
  const inputLength = arr.length;
  if (inputLength === targetLength) return arr;

  const result: number[] = new Array(targetLength);
  const scale = inputLength / targetLength;

  const sinc = (x: number) => {
    if (x === 0) return 1;
    const piX = Math.PI * x;
    return Math.sin(piX) / piX;
  };

  for (let i = 0; i < targetLength; i++) {
    const pos = i * scale; // position dans l'array d'origine
    const left = Math.floor(pos) - 10; // fenêtre de 10 échantillons de chaque côté
    const right = Math.floor(pos) + 10;

    let sum = 0;
    let weightSum = 0;

    for (let j = left; j <= right; j++) {
      if (j < 0 || j >= inputLength) continue; // on ne teste que l'indice
      const weight = sinc(pos - j);
      sum += arr[j]! * weight; // le ! dit à TS que c'est bien défini
      weightSum += weight;
    }

    result[i] = sum / weightSum;
  }

  return result;
};
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

#loaded_samples {
  position: absolute;
  top: 50px;
  left: 10px;
  width: 30px;
  height: 30px;
  padding: 0;
}
</style>
