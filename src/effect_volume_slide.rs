use crate::effect::*;
use crate::helper::clamp;
use core::default::Default;

#[derive(Clone, Default)]
pub struct Volume {
    pub value: f32,
}

#[derive(Clone, Default)]
pub struct EffectVolumeSlide {
    pub data: Volume,
}

impl EffectPlugin for EffectVolumeSlide {
    fn tick0(&mut self, value: f32, _param2: f32) -> f32 {
        self.data.value = value;
        self.value()
    }

    fn tick(&mut self) -> f32 {
        self.value()
    }

    fn in_progress(&self) -> bool {
        self.data.value != 0.0
    }

    fn retrigger(&mut self) -> f32 {
        self.value()
    }

    fn clamp(&self, volume: f32) -> f32 {
        let mut v = volume;
        clamp(&mut v);
        v
    }

    fn value(&self) -> f32 {
        self.data.value
    }
}

impl EffectXM2EffectPlugin for EffectVolumeSlide {
    fn xm_convert(rawval: u8, _special: u8) -> Option<(Option<f32>, Option<f32>)> {
        if (rawval & 0xF0 != 0) && (rawval & 0x0F != 0) {
            /* Illegal state */
            return None;
        }
        if rawval & 0xF0 != 0 {
            /* Slide up */
            let f = (rawval >> 4) as f32 / 64.0;
            Some((Some(f), None))
        } else {
            /* Slide down */
            let f = (rawval & 0x0F) as f32 / 64.0;
            Some((Some(-f), None))
        }
    }

    /// updown=1:up, updown=2:down, else auto-choose
    fn xm_update_effect(&mut self, param: u8, updown: u8, _special2: f32) {
        let arg = match updown {
            1 => (param & 0x0F) << 4,
            2 => param & 0x0F,
            _ => param,
        };
        if let Some((Some(vol_slide), None)) = Self::xm_convert(arg, 0) {
            self.tick0(vol_slide, 0.0);
        }
        self.retrigger();
    }
}
