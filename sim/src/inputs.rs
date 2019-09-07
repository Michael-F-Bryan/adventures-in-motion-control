use crate::PerformanceClock;
use aimc_hal::clock::{Clock, HasClock};
use arrayvec::ArrayVec;
use comms::Rx;
use core::{cell::Cell, time::Duration};
use fps_counter::FpsInputs;

#[derive(Debug, Clone, Default)]
pub struct Inputs {
    clock: PerformanceClock,
    last_tick: Cell<Duration>,
    rx_buffer: ArrayVec<[u8; 256]>,
}

impl Inputs {
    /// A method which should be called at the beginning of every tick.
    pub fn begin_tick(&self) { self.last_tick.set(self.clock.elapsed()); }

    /// A method which should be called at the end of every tick.
    pub fn end_tick(&mut self) { self.rx_buffer.clear(); }

    pub(crate) fn on_data_received(&mut self, data: &[u8]) {
        // writes up to `capacity` bytes to the buffer. Extra items are silently
        // dropped on the floor.
        self.rx_buffer.extend(data.into_iter().copied());
    }
}

impl HasClock for Inputs {
    fn clock(&self) -> &dyn Clock { &self.clock }
}

impl FpsInputs for Inputs {
    fn tick_started(&self) -> Duration { self.last_tick.get() }
}

impl Rx for Inputs {
    fn receive(&self) -> &[u8] { &self.rx_buffer }
}
