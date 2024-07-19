
use crate::synthette_trap;
use crate::temperament::TemperamentBase;
use crate::note::{
    NoteNumber,
    NoteNumberValue,
};

pub(crate) const A4_FREQUENCY: f32 = 440.0;

#[derive(Debug, Clone, Copy)]
pub struct StandardTemperamentBase {
    base_note_number: NoteNumber,
}

impl StandardTemperamentBase {
    pub const fn new(base: NoteNumberValue) -> Self {
        StandardTemperamentBase {
            base_note_number: NoteNumber { value: base},
        }
    }

    pub const fn note_number(&self) -> NoteNumberValue {
        self.base_note_number.value
    }
}

impl TemperamentBase for StandardTemperamentBase {
    #[inline]
    fn base_note_number(&self) -> NoteNumber {
        self.base_note_number
    }

    #[inline]
    fn base_frequency(&self) -> f32 {
        A4_FREQUENCY * 2.0_f32.powf((self.base_note_number.value - note_number::A4) as f32 / 12.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomTemperamentBase {
    base_note_number: NoteNumber,
    base_frequency: f32,
}

impl CustomTemperamentBase {
    pub fn new(base: NoteNumberValue, base_frequency: f32) -> Self {
        if base_frequency <= 0.0 {
            synthette_trap!("Base frequency must be greater than 0.0");
        }

        if base_frequency.is_infinite() {
            synthette_trap!("Base frequency must be finite");
        }

        if base_frequency.is_nan() {
            synthette_trap!("Base frequency must not be NaN");
        }

        CustomTemperamentBase {
            base_note_number: NoteNumber { value: base },
            base_frequency,
        }
    }

    pub fn new_a4(a4_frequency: f32) -> Self {
        CustomTemperamentBase::new(note_number::A4, a4_frequency)
    }

    /// Creates a new custom temperament base with the given base note number and A4 frequency, using 12-tone equal temperament.
    pub fn new_custom_with_a4_frequency(base: NoteNumberValue, a4_frequency: f32) -> Self {
        CustomTemperamentBase::new(base, a4_frequency * 2.0_f32.powf((base - note_number::A4) as f32 / 12.0))
    }
}

impl TemperamentBase for CustomTemperamentBase {
    #[inline]
    fn base_note_number(&self) -> NoteNumber {
        self.base_note_number
    }

    #[inline]
    fn base_frequency(&self) -> f32 {
        self.base_frequency
    }
}

/// Standard MIDI note numbers for the 12-tone equal temperament.
pub(crate) mod note_number {
    use crate::note::NoteNumberValue;

    pub const C4: NoteNumberValue = 60;
    pub const C4_SHARP: NoteNumberValue = 61;
    pub const D4_FLAT: NoteNumberValue = 61;
    pub const D4: NoteNumberValue = 62;
    pub const D4_SHARP: NoteNumberValue = 63;
    pub const E4_FLAT: NoteNumberValue = 63;
    pub const E4: NoteNumberValue = 64;
    pub const F4: NoteNumberValue = 65;
    pub const F4_SHARP: NoteNumberValue = 66;
    pub const G4_FLAT: NoteNumberValue = 66;
    pub const G4: NoteNumberValue = 67;
    pub const G4_SHARP: NoteNumberValue = 68;
    pub const A4_FLAT: NoteNumberValue = 68;
    pub const A4: NoteNumberValue = 69;
    pub const A4_SHARP: NoteNumberValue = 70;
    pub const B4_FLAT: NoteNumberValue = 70;
    pub const B4: NoteNumberValue = 71;
}

pub mod base {
    use super::note_number;
    use super::StandardTemperamentBase;

    pub const C4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::C4);
    pub const C4_SHARP: StandardTemperamentBase = StandardTemperamentBase::new(note_number::C4_SHARP);
    pub const D4_FLAT: StandardTemperamentBase = StandardTemperamentBase::new(note_number::D4_FLAT);
    pub const D4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::D4);
    pub const D4_SHARP: StandardTemperamentBase = StandardTemperamentBase::new(note_number::D4_SHARP);
    pub const E4_FLAT: StandardTemperamentBase = StandardTemperamentBase::new(note_number::E4_FLAT);
    pub const E4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::E4);
    pub const F4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::F4);
    pub const F4_SHARP: StandardTemperamentBase = StandardTemperamentBase::new(note_number::F4_SHARP);
    pub const G4_FLAT: StandardTemperamentBase = StandardTemperamentBase::new(note_number::G4_FLAT);
    pub const G4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::G4);
    pub const G4_SHARP: StandardTemperamentBase = StandardTemperamentBase::new(note_number::G4_SHARP);
    pub const A4_FLAT: StandardTemperamentBase = StandardTemperamentBase::new(note_number::A4_FLAT);
    pub const A4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::A4);
    pub const A4_SHARP: StandardTemperamentBase = StandardTemperamentBase::new(note_number::A4_SHARP);
    pub const B4_FLAT: StandardTemperamentBase = StandardTemperamentBase::new(note_number::B4_FLAT);
    pub const B4: StandardTemperamentBase = StandardTemperamentBase::new(note_number::B4);
}

pub use base::*;
