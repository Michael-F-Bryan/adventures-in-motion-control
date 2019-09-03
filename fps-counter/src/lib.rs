#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

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

impl<In: HasClock, Out: FpsSink> System<In, Out> for FpsCounter {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        let now = inputs.clock().elapsed();

        let tick_period = (now - self.last_tick).as_secs_f32();
        outputs.emit_fps(Fps {
            frequency: 1.0 / tick_period,
        });

        self.last_tick = now;
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Fps {
    pub frequency: f32,
}

pub trait FpsSink {
    fn emit_fps(&mut self, fps: Fps);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aimc_hal::clock::DummyClock;
    use std::prelude::v1::*;

    #[derive(Debug, Default)]
    pub struct Sink(Vec<Fps>);

    impl FpsSink for Sink {
        fn emit_fps(&mut self, fps: Fps) { self.0.push(fps); }
    }

    #[test]
    fn track_time_of_last_tick() {
        let mut fps = FpsCounter::default();
        let should_be = Duration::new(1, 23);
        let mut sink = Sink::default();
        let time = DummyClock(should_be);

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
        let time = DummyClock(now);

        fps.poll(&time, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(
            sink.0[0],
            Fps {
                frequency: 1000.0 / 20.0,
            }
        );
        assert_eq!(fps.last_tick, now);
    }
}
