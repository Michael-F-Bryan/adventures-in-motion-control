use crate::PerformanceClock;
use aimc_hal::clock::{Clock, HasClock};
use arrayvec::ArrayVec;
use core::{cell::Cell, time::Duration};
use fps_counter::FpsInputs;

#[derive(Debug, Clone, Default)]
pub struct Inputs {
    clock: PerformanceClock,
    last_tick: Cell<Duration>,
    rx_buffer: ArrayVec<[u8; 256]>,
}

impl Inputs {
    pub fn begin_tick(&self) { self.last_tick.set(self.clock.elapsed()); }

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
