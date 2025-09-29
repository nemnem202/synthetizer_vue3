<template>
  <div class="input-label-container">
    <div class="input_knob" @wheel="handle_wheel">
      <DefaultKnob v-model="sharedValue" />
      <DefaultNumberInput v-model="sharedValue" />
    </div>
    <p v-if="label">{{ label }}</p>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  value: number;
  callback: (value: number) => void;
  label: string | undefined;
}>();

const sharedValue = ref(props.value);

watch(sharedValue, (newValue) => {
  props.callback(newValue);
});

watch(
  () => props.value,
  (newValue) => {
    console.log("oeoe");
    sharedValue.value = newValue;
  }
);

const handle_wheel = (e: WheelEvent) => {
  e.preventDefault();
  if (e.deltaY > 0) {
    sharedValue.value = Math.max(sharedValue.value - 2, 0);
  } else if (e.deltaY < 0) {
    sharedValue.value = Math.min(sharedValue.value + 2, 100);
  }
};
</script>

<style scoped>
.input-label-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.input_knob {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  width: fit-content;
  justify-content: center;
  width: fit-content;
  height: fit-content;
  cursor: pointer;
}

p {
  all: unset;
}
</style>
