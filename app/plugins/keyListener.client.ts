// plugins/synth-keyboard.client.ts
import { defineNuxtPlugin } from "#app";
import { SynthApi } from "~/sound/synth_api_service";

export default defineNuxtPlugin((nuxtApp) => {
  // Map des touches vers des numéros de note
  const key_to_note: Record<string, number> = {
    q: 60, // Do
    z: 61,
    s: 62,
    e: 63,
    d: 64,
    f: 65,
    t: 66,
    g: 67,
    y: 68,
    h: 69,
    u: 70,
    j: 71,
  };

  let playedNotes: number[] = [];

  // Listener pour keydown
  const handle_keydown = (event: KeyboardEvent) => {
    const note = key_to_note[event.key];
    if (note !== undefined && !playedNotes.includes(note)) {
      playedNotes.push(note);
      SynthApi.playNote({ value: note, velocity: 1 });
      event.preventDefault();
    }
  };

  // Listener pour keyup
  const handle_keyup = (event: KeyboardEvent) => {
    const note = key_to_note[event.key];
    if (note !== undefined && playedNotes.includes(note)) {
      playedNotes = playedNotes.filter((n) => n !== note);
      SynthApi.stopNote(note);
      event.preventDefault();
    }
  };

  // Ajouter les listeners
  window.addEventListener("keydown", handle_keydown);
  window.addEventListener("keyup", handle_keyup);
});
