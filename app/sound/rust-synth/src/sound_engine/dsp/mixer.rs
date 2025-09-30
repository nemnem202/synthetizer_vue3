use std::iter::Filter;

use web_sys::console;

use crate::{
    sound_engine::dsp::fx::{BiquadFilter, Echo, EchoParams, EffectTrait},
    utils::{toolkit::ToolKit, types::Mix},
};

pub struct Mixer {
    pub effects: Vec<Box<dyn EffectTrait>>,
    pub ECHO_DEFAULT_PRESET: EchoParams,
}

impl Mixer {
    pub fn new() -> Self {
        let mut mixer = Self {
            effects: Vec::new(),
            ECHO_DEFAULT_PRESET: EchoParams {
                delay: ToolKit::convert_ms_to_sample(300.0),
                feedback: 0.7,
                l_delay_offset: ToolKit::convert_ms_to_sample(10.0),
                r_delay_offset: ToolKit::convert_ms_to_sample(50.0),
                mix: Mix { dry: 1.0, wet: 0.7 },
            },
        };

        mixer
    }
    pub fn render(&mut self, sample_l: &mut f32, sample_r: &mut f32) {
        for effect in &mut self.effects {
            effect.process(sample_l, sample_r);
        }
    }

    pub fn create_echo(&mut self, id: u32) {
        let echo = Echo::new(
            self.ECHO_DEFAULT_PRESET.delay,
            self.ECHO_DEFAULT_PRESET.feedback,
            self.ECHO_DEFAULT_PRESET.r_delay_offset,
            self.ECHO_DEFAULT_PRESET.l_delay_offset,
            self.ECHO_DEFAULT_PRESET.mix,
            id as usize,
        );

        self.effects.push(Box::new(echo));
    }

    pub fn update_fx(&mut self, id: u32, param_index: u32, value: f32) {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.id() == id as usize) {
            if let Some(echo) = effect.as_any_mut().downcast_mut::<Echo>() {
                match param_index {
                    0 => echo.delay = ToolKit::convert_ms_to_sample(value),
                    1 => echo.feedback = value.min(1.0),
                    2 => echo.l_delay_offset = ToolKit::convert_ms_to_sample(value),
                    3 => echo.r_delay_offset = ToolKit::convert_ms_to_sample(value),
                    4 => echo.mix.dry = value.min(1.0),
                    5 => echo.mix.wet = value.min(1.0),
                    _ => console::error_1(&format!("Cannot update {}", param_index).into()),
                }
            } else if let Some(filter) = effect.as_any_mut().downcast_mut::<BiquadFilter>() {
                match param_index {
                    0 => filter.edit(value as f32, filter.q, filter.filter_type, filter.gain),
                    1 => filter.edit(
                        filter.frequency,
                        value as f32,
                        filter.filter_type,
                        filter.gain,
                    ),
                    2 => filter.edit(filter.frequency, filter.q, value as u8, filter.gain),
                    3 => filter.edit(
                        filter.frequency,
                        filter.q,
                        filter.filter_type as u8,
                        value as f32,
                    ),
                    _ => console::error_1(&format!("Cannot update {}", param_index).into()),
                }
            }
        }
    }

    pub fn remove_fx(&mut self, id: u32) {
        self.effects.retain(|e| e.id() != id as usize);
    }

    pub fn create_filter(&mut self, id: u32) {
        let filter = BiquadFilter::new(800.0, 0.7, id as usize, 0, 5.0);
        self.effects.push(Box::new(filter));
    }
}
