
use crate::SYNTHETTE_BLOCK_SIZE;

#[derive(Debug)]
pub struct SimpleFloatBlock {
    pub data: [f32; SYNTHETTE_BLOCK_SIZE],
}

impl SimpleFloatBlock {
    pub fn new() -> Self {
        SimpleFloatBlock {
            data: [0.0; SYNTHETTE_BLOCK_SIZE],
        }
    }
}
