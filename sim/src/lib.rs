#![no_std]

mod app;
mod browser;
mod clock;
mod inputs;
mod router;
mod utils;

pub use app::App;
pub use browser::{Browser, B};
pub use clock::PerformanceClock;
pub use inputs::Inputs;
pub use utils::encode_gcode_program;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn on_module_loaded() {
    // wire up pretty panic messages when the WASM module is loaded into memory
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Creates a new world, initializing the various systems and wiring up any
/// necessary interrupts.
#[wasm_bindgen]
pub fn setup_world() -> App { App::new(Inputs::default()) }

/// Poll the application, running each system in turn and letting them make
/// progress.
#[wasm_bindgen]
pub fn poll(app: &mut App, browser: &Browser) { app.poll(browser); }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
