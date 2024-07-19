
use crate::note::NoteNumber;
use crate::synthette_trap;
use crate::temperament::{Temperament, TemperamentBase};

pub struct EqualTemperament<Base: TemperamentBase> {
    base: Base,
    n: usize,
}

impl<Base: TemperamentBase> EqualTemperament<Base> {
    pub fn new(base: Base, n: usize) -> Self {
        if n < 1 {
            synthette_trap!("n must be at least 1");
        }

        EqualTemperament {
            base,
            n,
        }
    }
}

impl<Base: TemperamentBase> Temperament for EqualTemperament<Base> {
    fn frequency(&self, note_number: NoteNumber) -> f32 {
        self.base.equal_temperament_frequency(self.n, note_number)
    }
}
