use core::time::Duration;

/// A source of time.
pub trait Clock: Sync {
    /// The amount of time that has elapsed since some arbitrary point in time
    /// (e.g. when the program started).
    fn elapsed(&self) -> Duration;
}

/// Something which has a clock.
pub trait HasClock {
    fn clock(&self) -> &dyn Clock;
}

impl<C: Clock> HasClock for C {
    fn clock(&self) -> &dyn Clock { self }
}

/// A [`Clock`] which always returns a pre-defined [`Duration`].
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DummyClock(pub Duration);

impl Clock for DummyClock {
    fn elapsed(&self) -> Duration { self.0 }
}
