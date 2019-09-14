#![no_std]

pub mod automation;
pub mod axes;
pub mod clock;
pub mod messaging;
mod system;

pub use crate::{messaging::Handler, system::System};
