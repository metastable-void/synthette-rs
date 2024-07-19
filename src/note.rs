
pub type NoteNumberValue = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteNumber {
    pub value: NoteNumberValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteVelocity {
    pub value: i8,
}

impl NoteVelocity {
    pub fn valid(&self) -> bool {
        self.value >= 0
    }
}

impl From<i8> for NoteVelocity {
    fn from(value: i8) -> Self {
        NoteVelocity { value }
    }
}

impl From<NoteVelocity> for i8 {
    fn from(note_velocity: NoteVelocity) -> i8 {
        note_velocity.value
    }
}

impl From<NoteNumberValue> for NoteNumber {
    fn from(value: NoteNumberValue) -> Self {
        NoteNumber { value }
    }
}

impl From<NoteNumber> for NoteNumberValue {
    fn from(note_number: NoteNumber) -> NoteNumberValue {
        note_number.value
    }
}
