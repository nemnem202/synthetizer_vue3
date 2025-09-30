import { SynthApi } from "~/sound/synth_api_service";

export default defineNuxtPlugin((nuxtApp) => {
  const api = new SynthApi();
  nuxtApp.provide("synth_api", api);
});
