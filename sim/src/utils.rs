use aimc_motion::GcodeProgram;
use js_sys::Uint8Array;
use scroll::{Endian, Pwrite};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode_gcode_program(first_line: u32, text: &str) -> Uint8Array {
    let mut buffer = [0; anpp::Packet::MAX_PACKET_SIZE];
    let msg = GcodeProgram::new(first_line, text);
    let bytes_written = buffer
        .pwrite_with(msg, 0, Endian::network())
        .expect("Will always succeed");

    // note: this is effectively a &[u8] slice into the buffer on the stack,
    // hence the seemingly redundant copy
    Uint8Array::from(&buffer[..bytes_written])
}
