mod app;
#[cfg(target_arch = "wasm32")]
mod platform_specific;

pub use app::{App, Frontend};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn on_module_loaded() {
    // wire up pretty panic messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Creates a new world, initializing the various systems and wiring up any 
/// necessary interrupts.
#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn setup_world() -> App {
    App
}

/// Poll the application, running each system in turn and letting them make
/// progress.
#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn poll(app: &mut App) {
    use aimc_hal::System;
    use platform_specific::{PerformanceClock, Browser};

    let clock = PerformanceClock::default();
    let mut frontend = Browser::default();

    app.poll(&clock, &mut frontend);
}
