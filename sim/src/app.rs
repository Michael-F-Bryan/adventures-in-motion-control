use crate::{router::Router, Browser, Inputs, B};
use aimc_comms::Communications;
use aimc_fps_counter::FpsCounter;
use aimc_hal::System;
use aimc_motion::Motion;
use anpp::Packet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct App {
    inputs: Inputs,
    fps: FpsCounter,
    comms: Communications,
    motion: Motion,
}

impl App {
    pub fn new(inputs: Inputs) -> Self {
        let fps = FpsCounter::default();
        let comms = Communications::new();
        let motion = Motion::default();
        App {
            inputs,
            fps,
            comms,
            motion,
        }
    }

    pub fn poll(&mut self, browser: &Browser) {
        self.inputs.begin_tick();
        let mut browser = B(browser);

        self.handle_comms(&mut browser);
        self.fps.poll(&self.inputs, &mut browser);

        self.inputs.end_tick();
    }

    fn handle_comms(&mut self, browser: &mut B) {
        let mut router = Router {
            fps: &mut self.fps,
            motion: &mut self.motion,
        };
        let mut outputs = aimc_comms::Outputs::new(browser, &mut router);
        self.comms.poll(&self.inputs, &mut outputs);
    }
}

#[wasm_bindgen]
impl App {
    /// Send data from the frontend to the simulator.
    pub fn on_data_received(&mut self, data: &[u8]) {
        self.inputs.on_data_received(data);
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
