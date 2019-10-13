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
    let bytes_written = buffer
        .pwrite_with(msg, 0, Endian::network())
        .expect("Will always succeed");

    // note: this is effectively a &[u8] slice into the buffer on the stack,
    // hence the seemingly redundant copy
    let view_into_stack_buffer = Uint8Array::from(&buffer[..bytes_written]);
    Uint8Array::new(&view_into_stack_buffer )
}
