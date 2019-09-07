use crate::{router::Router, Browser, Inputs};
use aimc_hal::System;
use comms::Communications;
use fps_counter::FpsCounter;
use js_sys::Function;
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
    /// Send data from the frontend to the simulator.
    pub fn on_data_received(&mut self, data: &[u8]) {
        self.inputs.on_data_received(data);
    }

    /// Set the callback to be invoked whenever the simulator wants to send data
    /// to the frontend.
    ///
    /// The callback will be passed a [`js_sys::Uint8Array`] as the first
    /// argument.
    pub fn on_data_sent(&mut self, callback: Function) {
        self.browser.set_data_sent(callback);
    }
}
