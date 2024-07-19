
pub mod node;
pub mod temperament;
pub mod note;

pub const SYNTHETTE_BLOCK_SIZE: usize = 128;

pub const SYNTHETTE_ENDIANNESS_BIG: u8 = 0;
pub const SYNTHETTE_ENDIANNESS_LITTLE: u8 = 1;

#[cfg(target_endian = "big")]
pub const SYNTHETTE_ENDIANNESS: u8 = SYNTHETTE_ENDIANNESS_BIG;

#[cfg(target_endian = "little")]
pub const SYNTHETTE_ENDIANNESS: u8 = SYNTHETTE_ENDIANNESS_LITTLE;

#[cfg(not(any(target_endian = "big", target_endian = "little")))]
pub const SYNTHETTE_ENDIANNESS: u8 = SYNTHETTE_ENDIANNESS_LITTLE;

static mut SYNTHETTE_TRAP_MESSAGE: [u8; 256] = [0; 256];

fn synthette_trap_message_set(message: &str) {
    let message_bytes = message.as_bytes();
    let message_len = message_bytes.len();
    let message_len = if message_len > 255 { 255 } else { message_len };
    unsafe {
        SYNTHETTE_TRAP_MESSAGE[..message_len].copy_from_slice(&message_bytes[..message_len]);
        SYNTHETTE_TRAP_MESSAGE[message_len] = 0;
    }
}

#[allow(dead_code)]
fn synthette_trap(message: &str) -> ! {
    synthette_trap_message_set(message);
    panic!("{}", message);
}

/// Macro to trap with a formatted message.
#[macro_export]
macro_rules! synthette_trap {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        $crate::synthette_trap(&res)
    }}
}

pub(crate) static mut SYNTHETTE_INPUT_BLOCK: [f32; SYNTHETTE_BLOCK_SIZE] = [0.0; SYNTHETTE_BLOCK_SIZE];

pub(crate) static mut SYNTHETTE_OUTPUT_BLOCK: [f32; SYNTHETTE_BLOCK_SIZE] = [0.0; SYNTHETTE_BLOCK_SIZE];

pub(crate) static mut SYNTHETTE_SAMPLE_RATE: usize = 48000;

/// Get the last trap message.
#[no_mangle]
pub extern "C" fn synthette_trap_message_get() -> *const u8 {
    unsafe { SYNTHETTE_TRAP_MESSAGE.as_ptr() }
}

/// Get the processing block size of the target platform.
#[no_mangle]
pub extern "C" fn synthette_block_size_get() -> usize {
    SYNTHETTE_BLOCK_SIZE
}

/// Get the endianness of the target platform.
/// 0 for big endian, 1 for little endian.
#[no_mangle]
pub extern "C" fn synthette_endianness_get() -> u8 {
    SYNTHETTE_ENDIANNESS
}

/// Get the sample rate of the target platform.
#[no_mangle]
pub extern "C" fn synthette_sample_rate_get() -> usize {
    unsafe { SYNTHETTE_SAMPLE_RATE }
}

/// Set the sample rate of the target platform.
#[no_mangle]
pub extern "C" fn synthette_sample_rate_set(sample_rate: usize) {
    if 0 == sample_rate {
        synthette_trap!("sample rate must be greater than zero");
    }
    unsafe { SYNTHETTE_SAMPLE_RATE = sample_rate }
}

/// Returns a pointer to the input block.
#[no_mangle]
pub extern "C" fn synthette_input_block_get() -> *mut f32 {
    unsafe { SYNTHETTE_INPUT_BLOCK.as_mut_ptr() }
}

/// Returns a pointer to the output block.
#[no_mangle]
pub extern "C" fn synthette_output_block_get() -> *mut f32 {
    unsafe { SYNTHETTE_OUTPUT_BLOCK.as_mut_ptr() }
}
