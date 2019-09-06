use crate::{Browser, Inputs};
use aimc_hal::System;
use fps_counter::FpsCounter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct App {
    inputs: Inputs,
    browser: Browser,
    fps: FpsCounter,
}

impl App {
    pub fn new(inputs: Inputs, browser: Browser) -> App {
        let fps = FpsCounter::default();
        App {
            inputs,
            browser,
            fps,
        }
    }

    pub fn poll(&mut self) {
        self.inputs.begin_tick();

        self.fps.poll(&self.inputs, &mut self.browser);
    }
}

#[wasm_bindgen]
impl App {
    pub fn on_data_received(&mut self, data: &[u8]) {
        self.inputs.on_data_received(data);
    }
}
