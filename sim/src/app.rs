use crate::{router::Router, Browser, Inputs};
use aimc_hal::System;
use comms::Communications;
use fps_counter::FpsCounter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct App {
    inputs: Inputs,
    browser: Browser,
    fps: FpsCounter,
    comms: Communications,
}

impl App {
    pub fn new(inputs: Inputs, browser: Browser) -> App {
        let fps = FpsCounter::default();
        let comms = Communications::new();
        App {
            inputs,
            browser,
            fps,
            comms,
        }
    }

    pub fn poll(&mut self) {
        self.inputs.begin_tick();

        self.handle_comms();
        self.fps.poll(&self.inputs, &mut self.browser);

        self.inputs.end_tick();
    }

    fn handle_comms(&mut self) {
        let mut router = Router { fps: &mut self.fps };
        let mut outputs = comms::Outputs::new(&mut self.browser, &mut router);
        self.comms.poll(&self.inputs, &mut outputs);
    }
}

#[wasm_bindgen]
impl App {
    pub fn on_data_received(&mut self, data: &[u8]) {
        self.inputs.on_data_received(data);
    }
}
