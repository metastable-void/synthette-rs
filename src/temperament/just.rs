
use crate::temperament::{
    PeriodicTemperament,
    TemperamentBase,
};

const TONES_IN_OCTAVE: usize = 12;
const PYTHAGOREAN_SCALE: [f32; TONES_IN_OCTAVE] = [256.0 / 243.0, 9.0 / 8.0, 32.0 / 27.0, 81.0 / 64.0, 4.0 / 3.0, 729.0 / 512.0, 3.0 / 2.0, 128.0 / 81.0, 27.0 / 16.0, 16.0 / 9.0, 243.0 / 128.0, 2.0];

pub const fn pythagorean<Base: TemperamentBase>(base: Base) -> PeriodicTemperament<TONES_IN_OCTAVE, Base> {
    PeriodicTemperament {
        base_note: base,
        scale: PYTHAGOREAN_SCALE,
        is_octave: true,
    }
}
