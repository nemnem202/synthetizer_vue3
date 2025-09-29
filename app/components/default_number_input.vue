<template>
    <input @change="handleInputChange" v-model.number="modelValue" type="number"  @wheel.prevent="handle_wheel"></input>
</template>

<script setup lang="ts">
const modelValue = defineModel<number>({ required: true });
const handleInputChange = (e: Event) => {
  const target = e.target as HTMLInputElement; 
  if (!target) return;

  const value = Number(target.value);
  modelValue.value = Math.min(Math.max(value, 0), 100);

  
};

const handle_wheel = (e: WheelEvent) => {
  // évite que la molette modifie l'input et renvoie le focus
  ;(e.currentTarget as HTMLInputElement).blur()
}
</script>

<style scoped>
input {
    all: unset;
  width: 30px;
  text-align: center;
  position: absolute;
  /* remove spinner */
  -moz-appearance: textfield;
  appearance: text;
  opacity: 0;
    cursor: pointer;
    color: black;
}
input:focus {
    opacity: 1;
    cursor: text;
}
input::-webkit-inner-spin-button,
input::-webkit-outer-spin-button { -webkit-appearance: none; margin: 0; }
</style>