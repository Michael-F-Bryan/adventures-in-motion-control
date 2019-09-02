use aimc_hal::{System, clock::Clock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct App;

impl<In: Clock, Out: Frontend> System<In, Out> for App {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {}
}

/// The mechanism used by the [`App`] to interact with the outside world.
pub trait Frontend {}