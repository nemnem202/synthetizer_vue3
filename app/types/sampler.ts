export type Adsr = {
  attack: number;
  decay: number;
  delay: number;
  sustain: number;
  release: number;
};

export type Sampler = {
  id: number;
  sample_id: number;
  adsr: Adsr;
  pan: number;
  gain: number;
  shift: number;
};
