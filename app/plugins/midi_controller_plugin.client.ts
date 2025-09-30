import { MidiController } from "~/midi/midi_controller_service";

export default defineNuxtPlugin((nuxtApp) => {
  const midi_controller = new MidiController();
  nuxtApp.provide("midi-controller", midi_controller);
});
