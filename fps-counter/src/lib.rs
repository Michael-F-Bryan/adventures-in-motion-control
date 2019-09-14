#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[allow(unused_imports)]
use aimc_hal::clock::Clock;
use aimc_hal::{
    clock::HasClock,
    messaging::{Ack, Handler},
    System,
};
use arraydeque::{ArrayDeque, Wrapping};
use core::{convert::TryInto, time::Duration};
use scroll_derive::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FpsCounter {
    ticks: ArrayDeque<[Snapshot; FpsCounter::SNAPSHOTS], Wrapping>,
}

impl FpsCounter {
    pub const SNAPSHOTS: usize = 256;

    pub fn new() -> Self {
        FpsCounter {
            ticks: ArrayDeque::new(),
        }
    }

    fn calculate_fps(&self) -> Fps {
        Fps {
            frequency: self.frequency().unwrap_or_default(),
            tick_duration: self.average_tick_duration().unwrap_or_default(),
        }
    }

    fn frequency(&self) -> Option<f32> {
        if self.ticks.len() < 2 {
            // you can't calculate frequency when the thing doesn't repeat...
            return None;
        }

        let oldest = self.ticks.back().unwrap().start;
        let most_recent = self.ticks.front().unwrap().start;

        let total_elapsed_time = secs(most_recent - oldest);

        // -1 for the fencepost problem
        let total_ticks = self.ticks.len() - 1;
        Some(total_ticks as f32 / total_elapsed_time)
    }

    fn average_tick_duration(&self) -> Option<Duration> {
        if self.ticks.is_empty() {
            return None;
        }

        let got = self
            .ticks
            .iter()
            .fold(Duration::default(), |acc, elem| acc + elem.elapsed());

        Some(got / self.ticks.len().try_into().unwrap())
    }
}

fn secs(duration: Duration) -> f32 {
    let secs = duration.as_secs() as f32;
    let nanos = duration.subsec_nanos() as f32;
    secs + (nanos / 1e9)
}

impl<In: FpsInputs, Out: FpsSink> System<In, Out> for FpsCounter {
    fn poll(&mut self, inputs: &In, outputs: &mut Out) {
        let snapshot = Snapshot {
            start: inputs.tick_started(),
            end: inputs.clock().elapsed(),
        };
        self.ticks.push_front(snapshot);

        outputs.emit_fps(self.calculate_fps());
    }
}

/// Record when a particular thing started and ended.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Snapshot {
    start: Duration,
    end: Duration,
}

impl Snapshot {
    fn elapsed(&self) -> Duration { self.end - self.start }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Fps {
    pub frequency: f32,
    pub tick_duration: Duration,
}

pub trait FpsSink {
    fn emit_fps(&mut self, fps: Fps);
}

pub trait FpsInputs: HasClock {
    /// When did the last tick start? (i.e. from [`Clock::elapsed()`])
    fn tick_started(&self) -> Duration;
}

/// Clear the buffer used when [`FpsCounter`] calculates its rolling average.
#[derive(Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Clear {}

impl Handler<Clear> for FpsCounter {
    type Response = Ack;

    fn handle(&mut self, _msg: Clear) -> Self::Response {
        self.ticks.clear();
        Ack::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aimc_hal::clock::DummyClock;
    use std::prelude::v1::*;

    #[derive(Debug, Default)]
    pub struct Sink(Vec<Fps>);

    impl Sink {
        fn first(&self) -> Fps {
            self.0.first().copied().expect("The sink is empty")
        }
    }

    impl FpsSink for Sink {
        fn emit_fps(&mut self, fps: Fps) { self.0.insert(0, fps); }
    }

    #[derive(Debug, Default)]
    pub struct DummyInputs {
        clock: DummyClock,
        tick_started: Duration,
    }

    impl DummyInputs {
        fn new(tick_started: Duration, now: Duration) -> DummyInputs {
            DummyInputs {
                clock: DummyClock(now),
                tick_started,
            }
        }

        fn with_elapsed(elapsed: Duration) -> DummyInputs {
            DummyInputs::new(Duration::new(0, 0), elapsed)
        }

        fn add(&mut self, duration: Duration) {
            core::mem::swap(&mut self.clock.0, &mut self.tick_started);
            self.clock.0 = self.tick_started + duration;
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
        let mut sink = Sink::default();
        let inputs = DummyInputs::with_elapsed(Duration::new(1, 23));

        fps.poll(&inputs, &mut sink);

        let should_be = Snapshot {
            start: inputs.tick_started,
            end: inputs.clock.0,
        };
        assert_eq!(fps.ticks[0], should_be);
    }

    #[test]
    fn record_snapshot() {
        let should_be = Snapshot {
            start: Duration::from_millis(50),
            end: Duration::from_millis(100),
        };
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let inputs = DummyInputs::new(should_be.start, should_be.end);

        fps.poll(&inputs, &mut sink);

        assert_eq!(fps.ticks.len(), 1);
        assert_eq!(fps.ticks[0], should_be);
    }

    #[test]
    fn first_reading_has_no_frequency() {
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let tick_duration = Duration::from_millis(20);
        let inputs = DummyInputs::with_elapsed(tick_duration);

        fps.poll(&inputs, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(
            sink.first(),
            Fps {
                frequency: 0.0,
                tick_duration,
            }
        );
    }

    #[test]
    fn calculate_last_tick_duration() {
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let tick_duration = Duration::from_millis(25);
        let tick_started = Duration::new(1, 0);
        let inputs =
            DummyInputs::new(tick_started, tick_started + tick_duration);

        fps.poll(&inputs, &mut sink);

        assert_eq!(sink.0.len(), 1);
        assert_eq!(sink.first().tick_duration, tick_duration);
    }

    #[test]
    fn calculate_fps_by_averaging_ticks() {
        let mut fps = FpsCounter::default();
        let mut sink = Sink::default();
        let mut inputs = DummyInputs::with_elapsed(Duration::from_millis(20));

        // It'll think the first tick happened after 20ms (50Hz)
        fps.poll(&inputs, &mut sink);

        // then the second tick happens 50ms later (20Hz)
        inputs.add(Duration::from_millis(50));
        fps.poll(&inputs, &mut sink);

        println!("{:?} => {:?}", fps, sink);
        let got = sink.first();
        // The ticks started 20ms apart => 50Hz
        let should_be = Fps {
            frequency: 50.0,
            tick_duration: Duration::from_millis((50 + 20) / 2),
        };
        assert_eq!(got, should_be);
    }
}
