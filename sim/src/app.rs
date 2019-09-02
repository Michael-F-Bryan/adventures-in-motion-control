use aimc_hal::{clock::Clock, System};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct App;

impl<In: Inputs, Out: Frontend> System<In, Out> for App {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        // TODO: Implement this
     }
}

/// The mechanism used by the [`App`] to interact with the outside world.
pub trait Frontend {
    /// Log a message somewhere.
    fn log(&mut self, message: &str);
}

pub trait Inputs {
    /// Get the system clock.
    fn clock(&self) -> &dyn Clock;
}