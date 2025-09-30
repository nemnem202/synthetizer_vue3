import { SynthApi } from "~/sound/synth_api_service";

let synth_api: SynthApi | null = null;

export async function use_synth_api() {
  if (!synth_api) {
    synth_api = new SynthApi();
    await synth_api.init();
  }
  return synth_api;
}
