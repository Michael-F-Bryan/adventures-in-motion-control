use std::time::{Duration, Instant};

/// A source of time.
pub trait Clock: Sync {
    /// The amount of time that has elapsed since some arbitrary point in time
    /// (e.g. when the program started).
    fn elapsed(&self) -> Duration;
}

/// A [`Clock`] which uses the operating system clock to track time.
#[derive(Debug, Clone)]
pub struct OsClock {
    started: Instant,
}

impl OsClock {
    fn new() -> OsClock { OsClock::with_epoch(Instant::now()) }

    fn with_epoch(started: Instant) -> OsClock { OsClock { started } }
}

impl Clock for OsClock {
    fn elapsed(&self) -> Duration { self.started.elapsed() }
}

impl Default for OsClock {
    fn default() -> OsClock { OsClock::new() }
}

/// A [`Clock`] which always returns a pre-defined [`Duration`].
#[derive(Debug, Clone, PartialEq)]
pub struct DummyClock(pub Duration);

impl Clock for DummyClock {
    fn elapsed(&self) -> Duration { self.0 }
}
