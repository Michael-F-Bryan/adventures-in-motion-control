#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[allow(unused_imports)]
use aimc_hal::clock::Clock;
use aimc_hal::{clock::HasClock, System};
use core::time::Duration;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FpsCounter {
    last_tick: Duration,
}

impl FpsCounter {
    pub fn with_start_time(start: Duration) -> FpsCounter {
        FpsCounter { last_tick: start }
    }
}

impl<In: FpsInputs, Out: FpsSink> System<In, Out> for FpsCounter {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        let now = inputs.clock().elapsed();

        let tick_period = (now - self.last_tick).as_secs_f32();
        outputs.emit_fps(Fps {
            frequency: 1.0 / tick_period,
            last_tick_duration: now - inputs.tick_started(),
        });

        self.last_tick = now;
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Fps {
    pub frequency: f32,
    pub last_tick_duration: Duration,
}

pub trait FpsSink {
    fn emit_fps(&mut self, fps: Fps);
}

pub trait FpsInputs: HasClock {
    /// When did the last tick start? (i.e. from [`Clock::elapsed()`])
    fn tick_started(&self) -> Duration;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aimc_hal::clock::DummyClock;
    use std::prelude::v1::*;

    #[derive(Debug, Default)]
    pub struct Sink(Vec<Fps>);

    impl Sink {
        fn first(&self) -> Fps { self.0[0] }
    }

    impl FpsSink for Sink {
        fn emit_fps(&mut self, fps: Fps) { self.0.push(fps); }
    }

    #[derive(Debug, Default)]
    pub struct DummyInputs {
        clock: DummyClock,
        tick_started: Duration,
    }

    impl DummyInputs {
        fn with_elapsed(elapsed: Duration) -> DummyInputs {
            DummyInputs {
                clock: DummyClock(elapsed),
                tick_started: elapsed,
            }
        }
    }

    impl HasClock for DummyInputs {
        fn clock(&self) -> &dyn Clock { &self.clock }
    }

    impl FpsInputs for DummyInputs {
        fn tick_started(&self) -> Duration { self.tick_started }
    }

    #[test]
    fn track_time_of_last_tick() {
        let mut fps = FpsCounter::default();
        let should_be = Duration::new(1, 23);
        let mut sink = Sink::default();
        let time = DummyInputs::with_elapsed(should_be);

        fps.poll(&time, &mut sink);

        assert_eq!(fps.last_tick, should_be);
    }

    #[test]
    fn record_fps() {
        let start = Duration::new(42, 0);
        let period = Duration::from_millis(20);
        let now = start + period;
        let mut fps = FpsCounter::with_start_time(start);
        let mut sink = Sink::default();
        let time = DummyInputs::with_elapsed(now);

        fps.poll(&time, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(sink.first().frequency, 1000.0 / 20.0);
        assert_eq!(fps.last_tick, now);
    }

    #[test]
    fn name() {
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let tick_duration = Duration::from_millis(25);
        let tick_started = Duration::new(1, 0);
        let time = DummyInputs {
            tick_started,
            clock: DummyClock(tick_started + tick_duration),
        };

        fps.poll(&time, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(sink.first().last_tick_duration, tick_duration);
    }
}
