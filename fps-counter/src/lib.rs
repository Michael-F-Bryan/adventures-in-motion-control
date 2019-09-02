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

impl<In: HasClock, Out: FpsSink> System<In, Out> for FpsCounter {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        self.last_tick = inputs.clock().elapsed();

        let tick_period = self.last_tick.as_secs_f32();
        outputs.emit_fps(Fps {
            frequency: 1.0 / tick_period,
        });
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
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let time = DummyClock(Duration::new(2, 500_000_000));

        fps.poll(&time, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(
            sink.0[0],
            Fps {
                frequency: 1.0 / 2.5
            }
        );
    }
}
