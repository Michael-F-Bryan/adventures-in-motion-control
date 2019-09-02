//! WASM-specific code.

use aimc_hal::clock::Clock;
use crate::Frontend;
use std::time::Duration;

/// A [`Clock`] which uses the browser's native `performance.now()` function
/// to keep track of time.
#[derive(Debug, Clone, Default)]
pub struct PerformanceClock;

impl Clock for PerformanceClock {
    fn elapsed(&self) -> Duration {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Browser;

impl Frontend for Browser {}