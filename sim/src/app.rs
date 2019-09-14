use crate::{router::Router, Browser, Inputs};
use aimc_comms::Communications;
use aimc_fps_counter::FpsCounter;
use aimc_hal::System;
use anpp::Packet;
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
    pub fn new(inputs: Inputs, browser: Browser) -> Self {
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
        let mut outputs =
            aimc_comms::Outputs::new(&mut self.browser, &mut router);
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

    /// Sends the backend a message (via [`App::on_data_received()`]) to echo
    /// back a string of text.
    pub fn echo(&mut self, text: &str) -> Result<(), JsValue> {
        let pkt = Packet::with_data(42, text.as_bytes())
            .map_err(|_| "The input text is too long")?;

        let mut buffer = [0; Packet::MAX_PACKET_SIZE + 5];
        let bytes_written = pkt
            .write_to_buffer(&mut buffer)
            .map_err(|_| "Unable to write the packet to a buffer")?;

        self.on_data_received(&buffer[..bytes_written]);

        Ok(())
    }
}
