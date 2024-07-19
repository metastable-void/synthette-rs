
use crate::synthette_trap;
use crate::note::NoteNumber;
use crate::temperament::{Temperament, TemperamentBase};


/// A temperament that divides the period (usually an octave) into a fixed number of pitch ratios.
#[derive(Debug)]
pub struct PeriodicTemperament<const N: usize, Base: TemperamentBase> {
    pub(crate) base_note: Base,
    pub(crate) scale: [f32; N],
    pub(crate) is_octave: bool,
}

impl<const N: usize, Base: TemperamentBase> PeriodicTemperament<N, Base> {
    pub fn new(base_note: Base, scale: [f32; N]) -> Self {
        if scale.len() < 1 {
            synthette_trap!("scale must have at least one element");
        }
        
        if scale[0] <= 1.0 {
            synthette_trap!("scale must have a first element greater than 1.0");
        }

        for i in 1..scale.len() {
            if scale[i] <= scale[i - 1] {
                synthette_trap!("scale must monotonically increase");
            }
        }

        let is_octave = scale[scale.len() - 1] == 2.0;
        
        PeriodicTemperament {
            base_note,
            scale,
            is_octave,
        }
    }

    pub fn base(&self) -> &Base {
        &self.base_note
    }

    pub fn scale(&self) -> &[f32; N] {
        &self.scale
    }

    pub fn is_octave(&self) -> bool {
        self.is_octave
    }

    pub fn period(&self) -> f32 {
        self.scale[self.scale.len() - 1]
    }

    pub fn scale_length(&self) -> usize {
        self.scale.len()
    }
}

impl<const N: usize, Base: TemperamentBase> Temperament for PeriodicTemperament<N, Base> {
    fn frequency(&self, note_number: NoteNumber) -> f32 {
        let base_note_number = self.base_note.base_note_number();
        let base_frequency = self.base_note.base_frequency();
        let note_number_diff = (note_number.value - base_note_number.value) as isize;
        let scale_length = N as isize;
        let remainder = ((note_number_diff % scale_length) + scale_length) % scale_length;
        let period_diff = (note_number_diff - remainder) / scale_length; // octave diff
        let period_ratio = self.period();
        let frequency_ratio = self.scale[remainder as usize];
        let frequency = base_frequency * period_ratio.powi(period_diff as i32) * frequency_ratio;
        frequency
    }
}
