//! WASM-specific code.

use aimc_hal::clock::Clock;
use std::time::Duration;
use wasm_bindgen::JsValue;

/// A [`Clock`] which uses the browser's native `performance.now()` function
/// to keep track of time.
#[derive(Debug, Clone, Default)]
pub struct PerformanceClock;

impl Clock for PerformanceClock {
    fn elapsed(&self) -> Duration {
        let perf = web_sys::window()
            .expect("The window always exists")
            .performance()
            .expect("The window should always have a performance timer");

        let now_ms = perf.now();
        let secs = (now_ms / 1000.0).floor();
        let secs = secs as u64;

        let nanos = ((now_ms / 1000.0).fract() * 1e9).floor();
        let nanos = nanos as u32;

        Duration::new(secs as u64, nanos as u32)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Inputs {
    clock: PerformanceClock,
}

impl crate::app::Inputs for Inputs {
    fn clock(&self) -> &dyn Clock { &self.clock }
}

#[derive(Debug, Clone, Default)]
pub struct Browser;

impl crate::app::Frontend for Browser {
    fn log(&mut self, msg: &str) {
        let msg = JsValue::from(msg);
        web_sys::console::log_1(&msg);
    }
}
