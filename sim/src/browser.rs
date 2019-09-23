use aimc_comms::Tx;
use aimc_fps_counter::{Fps, FpsSink};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An arbitrary JavaScript object which implements the [`Browser`]
    /// interface.
    pub type Browser;

    #[wasm_bindgen(structural, method)]
    pub fn set_fps(this: &Browser, frequency: f32, tick_duration_ms: f32);

    #[wasm_bindgen(structural, method)]
    pub fn send_data(this: &Browser, data: &[u8]);
}

/// Wrapper around a JavaScript [`Browser`] object which implements the various
/// system traits.
pub struct B<'a>(pub &'a Browser);

impl<'a> FpsSink for B<'a> {
    fn emit_fps(&mut self, fps: Fps) {
        self.0
            .set_fps(fps.frequency, fps.tick_duration.as_secs_f32() * 1000.0)
    }
}

impl<'a> Tx for B<'a> {
    fn send(&mut self, data: &[u8]) { self.0.send_data(data); }
}
