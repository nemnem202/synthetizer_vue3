use js_sys::Float32Array;
use web_sys::console;

use crate::utils::{toolkit::ToolKit, types::Sample};

pub struct SampleManager {
    pub samples: Vec<Sample>,
}

impl SampleManager {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, id: u32, raw_values: Float32Array, length: u32, hq: u8) {
        // Récupérer uniquement la portion utile du Float32Array
        console::log_1(&"Création d'un sample".into());
        let useful_slice = raw_values.subarray(0, length);

        // Convertir directement en Vec<f32>
        let values: Vec<f32> = useful_slice.to_vec();

        // Boxer pour stockage
        let boxed_values = values.into_boxed_slice();

        let sample = Sample {
            id,
            values: boxed_values,
            hq: hq,
        };
        self.samples.push(sample);
    }

    pub fn get_value(&self, sample_id: u32, index: u64, frequency: f32) -> f32 {
        if let Some(sample) = self.samples.iter().find(|s| s.id == sample_id) {
            let table = &sample.values;
            if table.is_empty() {
                return 0.0;
            }
            let table_len = table.len() as f32;
            let base_frequency: f32 = if sample.hq == 0 {
                ToolKit::midi_to_freq(60) // C4
            } else {
                ToolKit::midi_to_freq(12) // C0
            };
            let step = frequency / base_frequency; // combien de cycles par index ?
            let pos_in_table = (index as f32 * step) % table_len;
            let i0 = pos_in_table.floor() as usize;
            let i1 = (i0 + 1) % table.len();

            let frac = pos_in_table - i0 as f32;
            let v0 = table[i0];
            let v1 = table[i1];
            (v0 * (1.0 - frac)) + (v1 * frac)
        } else {
            0.0
        }
    }
}
