use aimc_motion::GcodeProgram;
use js_sys::Uint8Array;
use scroll::{Endian, Pwrite};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode_gcode_program(
    chunk_number: u16,
    first_line: u32,
    text: &str,
) -> Uint8Array {
    let mut buffer = [0; anpp::Packet::MAX_PACKET_SIZE];
    let msg = GcodeProgram::new(chunk_number, first_line, text);
    buffer
        .pwrite_with(msg, 0, Endian::network())
        .expect("Will always succeed");

    Uint8Array::from(&buffer[..])
}
