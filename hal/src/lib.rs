#![no_std]
#![feature(const_generics, const_generic_impls_guard)]

pub mod automation;
pub mod axes;
pub mod clock;
pub mod messaging;
mod system;

pub use crate::{messaging::Handler, system::System};
