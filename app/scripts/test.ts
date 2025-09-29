export const counter = ref<number>(0);
export const increment_counter = () => {
  counter.value++;
};
watchEffect(() => {
  console.log("le conteur a une valeur de " + counter.value);
});
