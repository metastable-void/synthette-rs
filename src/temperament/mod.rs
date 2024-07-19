
pub mod base;
pub mod just;
pub mod twelve_tone;

mod periodic;
mod equal;

pub use periodic::PeriodicTemperament;
pub use equal::EqualTemperament;

use std::collections::HashMap;

use crate::synthette_trap;
use crate::note::{
    NoteNumber,
    NoteNumberValue,
};

pub trait Temperament {
    fn frequency(&self, note_number: NoteNumber) -> f32;
}

pub trait TemperamentBase {
    fn base_frequency(&self) -> f32;
    fn base_note_number(&self) -> NoteNumber;
    fn equal_temperament_frequency(&self, n: usize, note_number: NoteNumber) -> f32 {
        if n < 1 {
            synthette_trap!("n must be at least 1");
        }
        let note_number_diff = (note_number.value - self.base_note_number().value) as isize;
        let frequency = self.base_frequency() * 2.0_f32.powi(note_number_diff as i32 / n as i32);
        frequency
    }
}

pub struct CachedTemperament<T: Temperament> {
    temperament: T,
    cache: HashMap<NoteNumberValue, f32>,
    min_note_number: NoteNumberValue,
    max_note_number: NoteNumberValue,
}

impl<T: Temperament> CachedTemperament<T> {
    pub fn new(temperament: T, min_note_number: NoteNumberValue, max_note_number: NoteNumberValue) -> Self {
        if min_note_number > max_note_number {
            synthette_trap!("min_note_number must be less than or equal to max_note_number");
        }
        let mut cache = HashMap::new();
        for note_number in min_note_number..=max_note_number {
            cache.insert(note_number, temperament.frequency(NoteNumber { value: note_number }));
        }
        CachedTemperament {
            temperament,
            cache,
            min_note_number,
            max_note_number,
        }
    }

    pub fn temperament(&self) -> &T {
        &self.temperament
    }

    pub fn min_note_number(&self) -> NoteNumberValue {
        self.min_note_number
    }

    pub fn max_note_number(&self) -> NoteNumberValue {
        self.max_note_number
    }

    pub fn frequency(&self, note_number: NoteNumber) -> f32 {
        if note_number.value < self.min_note_number || note_number.value > self.max_note_number {
            synthette_trap!("note_number out of range");
        }
        *self.cache.get(&note_number.value).unwrap()
    }
}

impl<T: Temperament> Temperament for CachedTemperament<T> {
    fn frequency(&self, note_number: NoteNumber) -> f32 {
        self.frequency(note_number)
    }
}
