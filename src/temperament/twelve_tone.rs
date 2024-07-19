

pub trait TwelveTone {
    const C: u16 = 1;
    const C_SHARP: u16 = 2;
    const D_FLAT: u16 = 2;
    const D: u16 = 4;
    const D_SHARP: u16 = 8;
    const E_FLAT: u16 = 8;
    const E: u16 = 16;
    const F: u16 = 32;
    const F_SHARP: u16 = 64;
    const G_FLAT: u16 = 64;
    const G: u16 = 128;
    const G_SHARP: u16 = 256;
    const A_FLAT: u16 = 256;
    const A: u16 = 512;
    const A_SHARP: u16 = 1024;
    const B_FLAT: u16 = 1024;
    const B: u16 = 2048;

    const ALL: u16 = 0b111111111111;

    fn twelve_tone_value(&self) -> u16;

    fn len(&self) -> usize {
        let value = self.twelve_tone_value();
        value.count_ones() as usize
    }
}

impl TwelveTone for u16 {
    fn twelve_tone_value(&self) -> u16 {
        *self & Self::ALL
    }
}
