use crate::PerformanceClock;
use aimc_hal::clock::{Clock, HasClock};
use core::{cell::Cell, time::Duration};
use fps_counter::FpsInputs;

#[derive(Debug, Clone, Default)]
pub struct Inputs {
    clock: PerformanceClock,
    last_tick: Cell<Duration>,
}

impl Inputs {
    pub fn begin_tick(&self) { self.last_tick.set(self.clock.elapsed()); }
}

impl HasClock for Inputs {
    fn clock(&self) -> &dyn Clock { &self.clock }
}

impl FpsInputs for Inputs {
    fn tick_started(&self) -> Duration { self.last_tick.get() }
}
