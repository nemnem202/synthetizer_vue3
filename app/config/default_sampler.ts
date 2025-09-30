import type { Sampler } from "~/types/sampler";

export const default_sampler_config: Sampler = {
  gain: 50,
  pan: 50,
  sample_id: 0,
  id: 0,
  shift: 50,
  adsr: {
    attack: 10,
    decay: 10,
    delay: 10,
    release: 10,
    sustain: 10,
  },
};
